#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;

use heck::ToKebabCase;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use tauri_bindgen_gen_rust::FnSig;
use tauri_bindgen_gen_rust::{BorrowMode, RustGenerator};
use wit_parser::{Function, Interface, TypeInfo};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `rustfmt` is executed to format generated code.
    #[cfg_attr(feature = "clap", clap(long))]
    pub fmt: bool,

    /// Whether or not the bindings assume interface values are always
    /// well-formed or whether checks are performed.
    #[cfg_attr(feature = "clap", clap(long))]
    pub unchecked: bool,

    /// If true, code generation should avoid any features that depend on `std`.
    #[cfg_attr(feature = "clap", clap(long))]
    pub no_std: bool,
}

impl Opts {
    pub fn build(self) -> RustWasm {
        RustWasm {
            opts: self,
            inner: RustGenerator::new(get_serde_attrs),
        }
    }
}

pub struct RustWasm {
    opts: Opts,
    inner: RustGenerator,
}

impl RustWasm {
    pub fn print_function(&self, func: &Function) -> TokenStream {
        let sig = FnSig {
            async_: true,
            unsafe_: false,
            private: false,
            self_arg: None,
            func,
        };

        let sig = self.inner.print_function_signature(
            &sig,
            &BorrowMode::AllBorrowed(parse_quote!('_)),
            &BorrowMode::Owned,
        );

        quote! {
            #sig {
                todo!()
            }
        }
    }
}

impl tauri_bindgen_core::Generate for RustWasm {
    fn to_tokens(&self, iface: &Interface) -> TokenStream {
        let docs = self.inner.print_docs(&iface.docs);

        let ident = format_ident!("{}", iface.ident.to_snake_case());

        let typedefs = self.inner.print_typedefs(
            iface.typedefs.iter().map(|typedef| typedef.borrow()),
            &BorrowMode::AllBorrowed(parse_quote!('a)),
        );

        let functions = iface.functions.iter().map(|func| self.print_function(func));

        quote! {
            #docs
            pub mod #ident {
                use ::tauri_bindgen_guest_rust::tauri_bindgen_abi;
                use ::tauri_bindgen_guest_rust::bitflags;
                #typedefs

                #(#functions)*
            }
        }
    }

    fn to_string(&self, iface: &Interface) -> (PathBuf, String) {
        let mut filename = PathBuf::from(iface.ident.to_kebab_case());
        filename.set_extension("rs");

        let tokens = self.to_tokens(iface);

        if self.opts.fmt {
            let syntax_tree = syn::parse2(tokens).unwrap();
            (filename, prettyplease::unparse(&syntax_tree))
        } else {
            (filename, tokens.to_string())
        }
    }
}

fn get_serde_attrs(name: &str, info: TypeInfo) -> Option<TokenStream> {
    let mut attrs = vec![];
    if tauri_bindgen_gen_rust::uses_two_names(info) {
        if name.ends_with("Param") {
            attrs.push(quote! { tauri_bindgen_abi::Writable })
        } else if name.ends_with("Result") {
            attrs.push(quote! { tauri_bindgen_abi::Readable })
        }
    } else {
        if info.contains(TypeInfo::PARAM) {
            attrs.push(quote! { tauri_bindgen_abi::Writable })
        }
        if info.contains(TypeInfo::RESULT) {
            attrs.push(quote! { tauri_bindgen_abi::Readable })
        }
    }

    Some(quote! { #[derive(#(#attrs),*)] })
}

#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;

use heck::ToKebabCase;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use tauri_bindgen_core::Generate;
use tauri_bindgen_core::GeneratorBuilder;
use tauri_bindgen_gen_rust::FnSig;
use tauri_bindgen_gen_rust::{BorrowMode, RustGenerator, TypeInfo, TypeInfos};
use wit_parser::{Function, Interface};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Builder {
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

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        let mut infos = TypeInfos::new();

        for func in &interface.functions {
            infos.collect_param_info(&interface.typedefs, &func.params);
            infos.collect_result_info(&interface.typedefs, &func.result);
        }

        for (id, typedef) in &interface.typedefs {
            log::debug!("type info: {} {:#?}", typedef.ident, infos[id]);
        }

        Box::new(RustWasm {
            opts: self,
            interface,
            infos,
        })
    }
}

pub struct RustWasm {
    opts: Builder,
    interface: Interface,
    infos: TypeInfos,
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

        let sig = self.print_function_signature(
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

impl RustGenerator for RustWasm {
    fn interface(&self) -> &Interface {
        &self.interface
    }

    fn infos(&self) -> &TypeInfos {
        &self.infos
    }

    fn additional_attrs(&self, ident: &str, info: TypeInfo) -> Option<TokenStream> {
        let mut attrs = vec![];
        if tauri_bindgen_gen_rust::uses_two_names(info) {
            if ident.ends_with("Param") {
                attrs.push(quote! { tauri_bindgen_abi::Writable })
            } else if ident.ends_with("Result") {
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
}

impl tauri_bindgen_core::Generate for RustWasm {
    fn to_tokens(&self) -> TokenStream {
        let docs = self.print_docs(&self.interface.docs);

        let ident = format_ident!("{}", self.interface.ident.to_snake_case());

        let typedefs = self.print_typedefs(
            self.interface.typedefs.iter().map(|(id, _)| id),
            &BorrowMode::AllBorrowed(parse_quote!('a)),
        );

        let functions = self
            .interface
            .functions
            .iter()
            .map(|func| self.print_function(func));

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

    fn to_file(&self) -> (PathBuf, String) {
        let mut filename = PathBuf::from(self.interface.ident.to_kebab_case());
        filename.set_extension("rs");

        let tokens = self.to_tokens();

        if self.opts.fmt {
            let syntax_tree = syn::parse2(tokens).unwrap();
            (filename, prettyplease::unparse(&syntax_tree))
        } else {
            (filename, tokens.to_string())
        }
    }
}

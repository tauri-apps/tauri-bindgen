#![allow(clippy::must_use_candidate)]

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use tauri_bindgen_gen_rust::FnSig;
use tauri_bindgen_gen_rust::{variants_of, BorrowMode, RustGenerator, TypeVariant};
use wit_parser::{Function, Interface, TypeDefKind};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Opts {
    /// Whether or not `rustfmt` is executed to format generated code.
    #[cfg_attr(feature = "clap", clap(long))]
    pub rustfmt: bool,

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
            inner: RustGenerator::new(),
        }
    }
}

pub struct RustWasm {
    opts: Opts,
    inner: RustGenerator,
}

impl RustWasm {
    pub fn print_interface(&self, iface: &Interface) -> TokenStream {
        let docs = self.inner.print_docs(&iface.docs);

        let ident = format_ident!("{}", iface.ident.to_snake_case());

        let typedefs = iface.typedefs.iter().flat_map(|typedef| {
            let typedef = typedef.borrow();

            eprintln!("printing typedef {:?}", typedef.ident);

            let variants = variants_of(
                &typedef.ident.to_upper_camel_case(),
                typedef.info,
                &BorrowMode::AllBorrowed(syn::parse_quote!('a)),
            );

            let variants = variants.iter().map(|TypeVariant { ident, borrow_mode }| {
                let docs = &typedef.docs;

                match &typedef.kind {
                    TypeDefKind::Alias(ty) => {
                        self.inner
                            .print_alias(docs, &ident, &ty, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Record(fields) => {
                        self.inner
                            .print_record(docs, &ident, &fields, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Flags(fields) => self.inner.print_flags(docs, &ident, &fields),
                    TypeDefKind::Variant(cases) => {
                        self.inner
                            .print_variant(docs, &ident, &cases, typedef.info, borrow_mode)
                    }
                    TypeDefKind::Enum(cases) => self.inner.print_enum(docs, &ident, &cases),
                    TypeDefKind::Union(cases) => {
                        self.inner
                            .print_union(docs, &ident, &cases, typedef.info, borrow_mode)
                    }
                }
            });

            quote! { #(#variants)* }
        });

        let functions = iface.functions.iter().map(|func| self.print_function(func));

        quote! {
            #docs
            pub mod #ident {
                #(#typedefs)*

                #(#functions)*
            }
        }
    }

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

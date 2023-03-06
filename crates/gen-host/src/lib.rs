#![allow(clippy::must_use_candidate)]

use heck::{ToUpperCamelCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::format_ident;
use syn::parse_quote;
use wit_parser::{Function, Interface, TypeDefKind};
use tauri_bindgen_gen_rust::{RustGenerator, variants_of, TypeVariant, BorrowMode, FnSig};
use quote::quote;

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
    pub fn build(self) -> Host {
        Host {
            opts: self,
            inner: RustGenerator::new(),
        }
    }
}

pub struct Host {
    opts: Opts,
    inner: RustGenerator,
}

impl Host {
    fn print_interface(&self, iface: &Interface) -> TokenStream {
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

        todo!()
        // let functions = iface.functions.iter().map(|func| self.print_function(func));

        // quote! {
        //     #docs
        //     pub mod #ident {
        //         #(#typedefs)*

        //         #(#functions)*
        //     }
        // }
    }

    // fn print_function(&self, func: &Function) -> TokenStream {
    //     let sig = self.inner.print_function_signature(func);

    //     quote! {
    //         #sig {
    //             todo!()
    //         }
    //     }
    // }

    fn print_trait<'a>(&self, ident: &str, functions: impl Iterator<Item = &'a Function>) -> TokenStream {
        let ident = format_ident!("{}", ident.to_snake_case());

        let functions = functions.map(|func| {
            let sig = FnSig { async_: false, unsafe_: false, private: false, self_arg: Some(parse_quote!(&mut self)), func };

            let sig = self.inner.print_function_signature(&sig,
                &BorrowMode::Owned,
                &BorrowMode::AllBorrowed(parse_quote!('_)),
            );

            quote! { #sig; }
        });

        quote! {
            pub trait #ident: Sized {
                #(#functions)* 
            }
        }
    }

    fn print_invoke_handler(&self, ) {}
}
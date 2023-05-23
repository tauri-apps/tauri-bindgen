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
use tauri_bindgen_core::TypeInfo;
use tauri_bindgen_core::TypeInfos;
use tauri_bindgen_gen_rust::FnSig;
use tauri_bindgen_gen_rust::{BorrowMode, RustGenerator};
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
        let infos = TypeInfos::collect_from_functions(&interface.typedefs, &interface.functions);

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
    pub fn print_function(&self, mod_ident: &str, func: &Function) -> TokenStream {
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

        let ident = func.ident.to_snake_case();

        let param_idents = func
            .params
            .iter()
            .map(|(ident, _)| format_ident!("{}", ident));

        quote! {
            #sig {
                ::tauri_bindgen_guest_rust::invoke(#mod_ident, #ident, &(#(#param_idents),*)).await.unwrap()
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
        if self.uses_two_names(info) {
            if ident.ends_with("Param") {
                attrs.push(quote! { serde::Serialize });
            } else if ident.ends_with("Result") {
                attrs.push(quote! { serde::Deserialize });
            }
        } else {
            if info.contains(TypeInfo::PARAM) {
                attrs.push(quote! { serde::Serialize });
            }
            if info.contains(TypeInfo::RESULT) {
                attrs.push(quote! { serde::Deserialize });
            }
        }

        Some(quote! { #[derive(#(#attrs),*)] })
    }

    fn default_param_mode(&self) -> BorrowMode {
        BorrowMode::AllBorrowed(parse_quote!('a))
    }

    fn print_resource(
        &self,
        docs: &str,
        ident: &proc_macro2::Ident,
        functions: &[Function],
        info: TypeInfo,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let functions = functions.iter().map(|func| {
            let sig = FnSig {
                async_: true,
                unsafe_: false,
                private: false,
                self_arg: Some(quote!(&self)),
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
        });

        quote! {
            #docs
            #additional_attrs
            pub struct #ident {
                id: u64
            }

            impl #ident {
                #(#functions)*
            }
        }
    }
}

impl tauri_bindgen_core::Generate for RustWasm {
    fn to_tokens(&mut self) -> TokenStream {
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
            .map(|func| self.print_function(&self.interface.ident.to_snake_case(), func));

        quote! {
            #docs
            #[allow(unused_imports, unused_variables, dead_code)]
            #[rustfmt::skip]
            pub mod #ident {
                use ::tauri_bindgen_guest_rust::serde;
                use ::tauri_bindgen_guest_rust::bitflags;
                #typedefs

                #(#functions)*
            }
        }
    }

    fn to_file(&mut self) -> (PathBuf, String) {
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

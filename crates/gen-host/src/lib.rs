#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;

use heck::ToKebabCase;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use tauri_bindgen_core::{Generate, GeneratorBuilder};
use tauri_bindgen_gen_rust::{BorrowMode, FnSig, RustGenerator, TypeInfo, TypeInfos};
use wit_parser::{Function, Interface};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Args))]
pub struct Builder {
    /// Whether or not `rustfmt` is executed to format generated code.
    #[cfg_attr(feature = "clap", clap(long))]
    pub fmt: bool,

    /// Whether or not to emit `tracing` macro calls on function entry/exit.
    #[cfg_attr(feature = "clap", clap(long))]
    pub tracing: bool,

    /// Whether or not to use async rust functions and traits.
    #[cfg_attr(feature = "clap", clap(long = "async"))]
    pub async_: bool,
}

impl GeneratorBuilder for Builder {
    fn build(self, interface: Interface) -> Box<dyn Generate> {
        let mut infos = TypeInfos::new();

        for func in &interface.functions {
            infos.collect_param_info(&interface.typedefs, &func.params);
            infos.collect_result_info(&interface.typedefs, &func.result);
        }

        Box::new(Host {
            opts: self,
            interface,
            infos,
        })
    }
}

pub struct Host {
    opts: Builder,
    interface: Interface,
    infos: TypeInfos,
}

impl RustGenerator for Host {
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
                attrs.push(quote! { serde::Deserialize })
            } else if ident.ends_with("Result") {
                attrs.push(quote! { serde::Serialize })
            }
        } else {
            if info.contains(TypeInfo::PARAM) {
                attrs.push(quote! { serde::Deserialize })
            }
            if info.contains(TypeInfo::RESULT) {
                attrs.push(quote! { serde::Serialize })
            }
        }

        Some(quote! { #[derive(#(#attrs),*)] })
    }

    fn default_param_mode(&self) -> BorrowMode {
        BorrowMode::Owned
    }
}

impl Generate for Host {
    fn to_tokens(&self) -> TokenStream {
        let docs = self.print_docs(&self.interface.docs);

        let ident = format_ident!("{}", self.interface.ident.to_snake_case());

        let typedefs = self.print_typedefs(
            self.interface.typedefs.iter().map(|(id, _)| id),
            &BorrowMode::LeafBorrowed(parse_quote!('a)),
        );

        let trait_ = self.print_trait(&self.interface.ident, self.interface.functions.iter());

        let add_to_router =
            self.print_add_to_router(&self.interface.ident, self.interface.functions.iter());

        quote! {
            #docs
            #[allow(unused_imports, unused_variables)]
            pub mod #ident {
                use ::tauri_bindgen_host::serde;
                use ::tauri_bindgen_host::bitflags;

                #typedefs

                #trait_

                #add_to_router
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

impl Host {
    fn print_trait<'a>(
        &self,
        ident: &str,
        functions: impl Iterator<Item = &'a Function>,
    ) -> TokenStream {
        let ident = format_ident!("{}", ident.to_upper_camel_case());

        let functions = functions.map(|func| {
            let sig = FnSig {
                async_: false,
                unsafe_: false,
                private: true,
                self_arg: Some(quote!(&mut self)),
                func,
            };

            let sig = self.print_function_signature(
                &sig,
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

    fn print_add_to_router<'a>(
        &self,
        mod_ident: &str,
        _functions: impl Iterator<Item = &'a Function>,
    ) -> TokenStream {
        let trait_ident = format_ident!("{}", mod_ident.to_upper_camel_case());

        // let mod_name = mod_ident.to_snake_case();

        // let functions = functions.map(|func| {
        //     let func_name = func.ident.to_snake_case();
        //     let func_ident = format_ident!("{}", func_name);

        //     let params = self
        //         .print_function_params(&func.params, &BorrowMode::Owned);

        //     let param_idents = func
        //         .params
        //         .iter()
        //         .map(|(ident, _)| format_ident!("{}", ident));

        //     let results = self
        //         .print_function_result(&func.result, &BorrowMode::AllBorrowed(parse_quote!('_)));

        //     quote! {
        //         router.func_wrap(#mod_name, #func_name, move |cx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, #params| #results {
        //             let cx = get_cx(cx.data_mut());

        //             cx.#func_ident(#(#param_idents),*)
        //         })?;
        //     }
        // });

        quote! {
            pub fn add_to_router<T, U>(
                router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
            where
                U: #trait_ident
            {
                // #( #functions )*

                Ok(())
            }
        }
    }
}

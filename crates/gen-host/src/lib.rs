#![allow(clippy::must_use_candidate)]

use std::path::PathBuf;

use heck::ToKebabCase;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::parse_quote;
use tauri_bindgen_gen_rust::{BorrowMode, FnSig, RustGenerator};
use wit_parser::TypeInfo;
use wit_parser::{Function, Interface};

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
    pub fn build(self) -> Host {
        Host {
            opts: self,
            inner: RustGenerator::new(get_serde_attrs),
        }
    }
}

pub struct Host {
    opts: Opts,
    inner: RustGenerator,
}

impl Host {
    pub fn generate(&self, iface: &Interface) -> (PathBuf, String) {
        let mut filename = PathBuf::from(iface.ident.to_kebab_case());
        filename.set_extension("rs");

        let tokens = self.print_interface(iface);

        if self.opts.fmt {
            let syntax_tree = syn::parse2(tokens).unwrap();
            (filename, prettyplease::unparse(&syntax_tree))
        } else {
            (filename, tokens.to_string())
        }
    }

    pub fn print_interface(&self, iface: &Interface) -> TokenStream {
        let docs = self.inner.print_docs(&iface.docs);

        let ident = format_ident!("{}", iface.ident.to_snake_case());

        let typedefs = self.inner.print_typedefs(
            iface.typedefs.iter().map(|typedef| typedef.borrow()),
            &BorrowMode::AllBorrowed(parse_quote!('a)),
        );

        let trait_ = self.print_trait(&iface.ident, iface.functions.iter());

        let add_to_router = self.print_add_to_router(&iface.ident, iface.functions.iter());

        quote! {
            #docs
            pub mod #ident {
                use ::tauri_bindgen_host::tauri_bindgen_abi;

                #typedefs

                #trait_

                #add_to_router
            }
        }
    }

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

            let sig = self.inner.print_function_signature(
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
        functions: impl Iterator<Item = &'a Function>,
    ) -> TokenStream {
        let trait_ident = format_ident!("{}", mod_ident.to_upper_camel_case());

        let mod_name = mod_ident.to_snake_case();

        let functions = functions.map(|func| {
            let func_name = func.ident.to_snake_case();
            let func_ident = format_ident!("{}", func_name);

            let params = self
                .inner
                .print_function_params(func.params.iter(), &BorrowMode::Owned);

            let param_idents = func
                .params
                .iter()
                .map(|(ident, _)| format_ident!("{}", ident));

            let results = self
                .inner
                .print_function_result(&func.result, &BorrowMode::AllBorrowed(parse_quote!('_)));

            quote! {
                router.func_wrap(#mod_name, #func_name, |cx, #(#params),*| #results {
                    // let cx = get_cx(cx.state);
                    let cx: U = todo!();

                    cx.#func_ident(#(#param_idents),*)
                })?;
            }
        });

        quote! {
            pub fn add_to_router<T, U>(
                router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
                get_cx: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
            where
                U: #trait_ident
            {
                #( #functions )*

                Ok(())
            }
        }
    }
}

fn get_serde_attrs(name: &str, info: TypeInfo) -> Option<TokenStream> {
    let mut attrs = vec![];
    if tauri_bindgen_gen_rust::uses_two_names(info) {
        if name.ends_with("Param") {
            attrs.push(quote! { tauri_bindgen_abi::Readable })
        } else if name.ends_with("Result") {
            attrs.push(quote! { tauri_bindgen_abi::Writable })
        }
    } else {
        if info.contains(TypeInfo::PARAM) {
            attrs.push(quote! { tauri_bindgen_abi::Readable })
        }
        if info.contains(TypeInfo::RESULT) {
            attrs.push(quote! { tauri_bindgen_abi::Writable })
        }
    }

    Some(quote! { #[derive(#(#attrs),*)] })
}

#[cfg(test)]
mod test {
    use wit_parser::{FunctionResult, Type};

    use super::*;

    #[test]
    fn trait_() {
        let funcs = [
            Function {
                docs: "".to_string(),
                ident: "foo".to_string(),
                params: vec![("x".to_string(), Type::String)],
                result: FunctionResult::Named(vec![]),
            },
            Function {
                docs: "".to_string(),
                ident: "bar".to_string(),
                params: vec![("x".to_string(), Type::List(Box::new(Type::U8)))],
                result: FunctionResult::Anon(Type::U64),
            },
            Function {
                docs: "".to_string(),
                ident: "fiz".to_string(),
                params: vec![("x".to_string(), Type::List(Box::new(Type::U8)))],
                result: FunctionResult::Anon(Type::List(Box::new(Type::String))),
            },
        ];

        let gen = Opts::default().build();
        let tokens = gen.print_trait("test", funcs.iter());
        println!("{}", tokens.to_string());
        let syntax_tree = syn::parse2(tokens).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        println!("{}", formatted);
    }

    #[test]
    fn full() {
        let source = include_str!("test.wit");

        let iface = wit_parser::parse_str(source, |_| false).unwrap();

        let gen = Opts::default().build();
        let tokens = gen.print_interface(&iface);
        println!("{}", tokens.to_string());
        let syntax_tree = syn::parse2(tokens).unwrap();
        let formatted = prettyplease::unparse(&syntax_tree);
        println!("{}", formatted);
    }
}

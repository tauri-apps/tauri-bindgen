#![allow(
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::unused_self
)]

use std::collections::HashSet;
use std::path::PathBuf;

use heck::ToKebabCase;
use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Literal, TokenStream};
use quote::format_ident;
use quote::quote;
use tauri_bindgen_core::{Generate, GeneratorBuilder, TypeInfo, TypeInfos};
use tauri_bindgen_gen_rust::{print_generics, BorrowMode, FnSig, RustGenerator};
use wit_parser::{Function, Interface, Type, TypeDefKind};

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
        let infos = TypeInfos::collect_from_functions(&interface.typedefs, &interface.functions);

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
                attrs.push(quote! { serde::Deserialize });
            } else if ident.ends_with("Result") {
                attrs.push(quote! { serde::Serialize });
            }
        } else {
            if info.contains(TypeInfo::PARAM) {
                attrs.push(quote! { serde::Deserialize });
            }
            if info.contains(TypeInfo::RESULT) {
                attrs.push(quote! { serde::Serialize });
            }
        }

        Some(quote! { #[derive(#(#attrs),*)] })
    }

    fn default_param_mode(&self) -> BorrowMode {
        BorrowMode::Owned
    }

    fn print_resource(
        &self,
        docs: &str,
        ident: &proc_macro2::Ident,
        functions: &[Function],
        _info: TypeInfo,
    ) -> TokenStream {
        let docs = self.print_docs(docs);

        let mut resources = HashSet::new();
        for func in functions {
            for (_, ty) in &func.params {
                self.extract_resources(ty, &mut resources);
            }
            if let Some(result) = &func.result {
                for ty in result.types() {
                    self.extract_resources(ty, &mut resources);
                }
            }
        }

        let resources = resources.iter().map(|r| {
            let ident = format_ident!("{}", r.to_upper_camel_case());

            quote! { type #ident: #ident; }
        });

        let trait_ = self.print_trait(&ident.to_string(), functions.iter(), resources, false);

        quote! {
            #docs
            #trait_
        }
    }

    fn print_ty(&self, ty: &Type, mode: &BorrowMode) -> TokenStream {
        match ty {
            Type::Bool => quote! { bool },
            Type::U8 => quote! { u8 },
            Type::U16 => quote! { u16 },
            Type::U32 => quote! { u32 },
            Type::U64 => quote! { u64 },
            Type::U128 => quote! { u128 },
            Type::S8 => quote! { i8 },
            Type::S16 => quote! { i16 },
            Type::S32 => quote! { i32 },
            Type::S64 => quote! { i64 },
            Type::S128 => quote! { i128 },
            Type::Float32 => quote! { f32 },
            Type::Float64 => quote! { f64 },
            Type::Char => quote! { char },
            Type::String => match mode {
                BorrowMode::Owned => quote! { String },
                BorrowMode::AllBorrowed(lt) | BorrowMode::LeafBorrowed(lt) => quote! { &#lt str },
            },
            Type::List(ty) => {
                let is_primitive = matches!(
                    **ty,
                    Type::U8
                        | Type::S8
                        | Type::U16
                        | Type::S16
                        | Type::U32
                        | Type::S32
                        | Type::U64
                        | Type::S64
                        | Type::Float32
                        | Type::Float64
                );

                let ty = self.print_ty(ty, mode);

                match mode {
                    BorrowMode::Owned => quote! { Vec<#ty> },
                    BorrowMode::AllBorrowed(lt) => quote! { &#lt [#ty] },
                    BorrowMode::LeafBorrowed(lt) => {
                        if is_primitive {
                            quote! { &#lt [#ty] }
                        } else {
                            quote! { Vec<#ty> }
                        }
                    }
                }
            }
            Type::Tuple(types) => {
                if types.len() == 1 {
                    let ty = self.print_ty(&types[0], mode);

                    quote! { (#ty,) }
                } else {
                    let types = types.iter().map(|ty| self.print_ty(ty, mode));

                    quote! { (#(#types),*) }
                }
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty, mode);

                quote! { Option<#ty> }
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map(|ty| self.print_ty(ty, mode))
                    .unwrap_or(quote! { () });
                let err = err
                    .as_ref()
                    .map(|ty| self.print_ty(ty, mode))
                    .unwrap_or(quote! { () });

                quote! { Result<#ok, #err> }
            }
            Type::Id(id) => {
                let typedef = &self.interface().typedefs[*id];
                let info = self.infos()[*id];

                if let TypeDefKind::Resource(_) = &typedef.kind {
                    return quote! { ::tauri_bindgen_host::ResourceId };
                }

                let ident = if self.uses_two_names(info) {
                    match mode {
                        BorrowMode::Owned => {
                            format_ident!("{}Result", typedef.ident.to_upper_camel_case())
                        }
                        BorrowMode::AllBorrowed(_) | BorrowMode::LeafBorrowed(_) => {
                            format_ident!("{}Param", typedef.ident.to_upper_camel_case())
                        }
                    }
                } else {
                    format_ident!("{}", typedef.ident.to_upper_camel_case())
                };

                let generics = print_generics(info, mode);

                quote! { #ident #generics }
            }
        }
    }
}

impl Host {
    fn print_trait<'a>(
        &self,
        ident: &str,
        functions: impl Iterator<Item = &'a Function>,
        additional_items: impl Iterator<Item = TokenStream>,
        sized: bool,
    ) -> TokenStream {
        let ident = format_ident!("{}", ident.to_upper_camel_case());

        let functions = functions.map(|func| {
            let sig = FnSig {
                async_: self.opts.async_,
                unsafe_: false,
                private: true,
                self_arg: Some(quote!(&self)),
                func,
            };

            let sig = self.print_function_signature(&sig, &BorrowMode::Owned, &BorrowMode::Owned);

            quote! { #sig; }
        });

        let sized = sized.then_some(quote!(: Sized));

        let async_trait = self
            .opts
            .async_
            .then_some(quote! { #[::tauri_bindgen_host::async_trait] });

        quote! {
            #async_trait
            pub trait #ident #sized {
                #(#additional_items)*
                #(#functions)*
            }
        }
    }

    fn extract_resources<'a>(&'a self, ty: &Type, resources: &mut HashSet<&'a str>) {
        match ty {
            Type::List(ty) | Type::Option(ty) => {
                self.extract_resources(ty, resources);
            }
            Type::Tuple(types) => {
                for ty in types {
                    self.extract_resources(ty, resources);
                }
            }
            Type::Result { ok, err } => {
                if let Some(ok) = ok {
                    self.extract_resources(ok, resources);
                }

                if let Some(err) = err {
                    self.extract_resources(err, resources);
                }
            }
            Type::Id(id) => {
                let typedef = &self.interface().typedefs[*id];

                if let TypeDefKind::Resource(_) = &typedef.kind {
                    resources.insert(&typedef.ident);
                }
            }
            _ => {}
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

            let param_decl = match func.params.len() {
                0 => quote! { () },
                1 => {
                    let ty = &func.params.first().unwrap().1;
                    let ty = self.print_ty(ty,  &BorrowMode::Owned);
                    quote! { #ty }
                }
                _ => {
                    let tys = func
                        .params
                        .iter()
                        .map(|(_, ty)| { self.print_ty(ty, &BorrowMode::Owned) });
                    quote! { (#(#tys),*) }
                }
            };

            let param_acc = match func.params.len() {
                0 => quote! { },
                1 => quote! { p },
                _ => {
                    let ids = func
                        .params
                        .iter()
                        .enumerate()
                        .map(|(i, _)| {
                            let i = Literal::usize_unsuffixed(i);
                            quote! { p.#i }
                        });
                    quote! { #(#ids),* }
                }
            };

            // let result = match func.result.as_ref() {
            //     Some(FunctionResult::Anon(ty)) => {
            //         let ty = self.print_ty(ty, &BorrowMode::Owned);

            //         quote! { #ty }
            //     }
            //     Some(FunctionResult::Named(types)) if types.len() == 1 => {
            //         let (_, ty) = &types[0];
            //         let ty = self.print_ty(ty, &BorrowMode::Owned);

            //         quote! { #ty }
            //     }
            //     Some(FunctionResult::Named(types)) => {
            //         let types = types.iter().map(|(_, ty)| self.print_ty(ty, &BorrowMode::Owned));

            //         quote! { (#(#types),*) }
            //     }
            //     _ => quote! { () },
            // };

            if self.opts.async_ {
                quote! {
                    let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
                    router.define_async(
                        #mod_name,
                        #func_name,
                        move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: #param_decl| {
                            let get_cx = get_cx.clone();
                            Box::pin(async move {
                                let ctx = get_cx(ctx.data());
                                Ok(ctx.#func_ident(#param_acc).await)
                            })
                        })?;
                }
            } else {
                quote! {
                    let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
                    router.define(
                        #mod_name,
                        #func_name,
                        move |ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>, p: #param_decl| {
                            let ctx = get_cx(ctx.data());

                            Ok(ctx.#func_ident(#param_acc))
                        },
                    )?;
                }
            }
        });

        quote! {
            pub fn add_to_router<T, U>(
                router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
                get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
            ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
            where
                T: Send + Sync + 'static,
                U: #trait_ident + Send + Sync + 'static,
                R: ::tauri_bindgen_host::tauri::Runtime
            {
                let wrapped_get_cx = ::std::sync::Arc::new(get_cx);

                #( #functions )*

                Ok(())
            }
        }
    }
}

impl Generate for Host {
    fn to_tokens(&mut self) -> TokenStream {
        let docs = self.print_docs(&self.interface.docs);

        let ident = format_ident!("{}", self.interface.ident.to_snake_case());

        let typedefs = self.print_typedefs(
            self.interface.typedefs.iter().map(|(id, _)| id),
            &BorrowMode::Owned,
        );

        let resources = self.interface.typedefs.iter().filter_map(|(_, typedef)| {
            if let TypeDefKind::Resource(_) = &typedef.kind {
                let ident = format_ident!("{}", typedef.ident.to_upper_camel_case());
                let func_ident = format_ident!("get_{}_mut", typedef.ident.to_snake_case());

                Some(quote! {
                    type #ident: #ident;

                    fn #func_ident(&mut self, id: ::tauri_bindgen_host::ResourceId) -> &mut Self::#ident;
                })
            } else {
                None
            }
        });

        let trait_ = self.print_trait(
            &self.interface.ident,
            self.interface.functions.iter(),
            resources,
            true,
        );

        let add_to_router =
            self.print_add_to_router(&self.interface.ident, self.interface.functions.iter());

        quote! {
            #docs
            #[allow(unused_imports, unused_variables, dead_code)]
            #[rustfmt::skip]
            pub mod #ident {
                use ::tauri_bindgen_host::serde;
                use ::tauri_bindgen_host::bitflags;

                #typedefs

                #trait_

                #add_to_router
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

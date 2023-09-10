#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use heck::{ToShoutySnekCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

use syn::Lifetime;
use tauri_bindgen_core::{flags_repr, union_case_names, TypeInfo, TypeInfos};

use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Int, Interface, RecordField, Type, TypeDefId,
    TypeDefKind, UnionCase, VariantCase,
};

pub trait RustGenerator {
    fn interface(&self) -> &Interface;
    fn infos(&self) -> &TypeInfos;
    fn additional_attrs(&self, ident: &str, info: TypeInfo) -> Option<TokenStream>;
    fn default_param_mode(&self) -> BorrowMode;
    fn print_resource(
        &self,
        print_resource: &str,
        docs: &str,
        ident: &Ident,
        functions: &[Function],
        info: TypeInfo,
    ) -> TokenStream;

    fn print_typedefs(
        &self,
        ids: impl Iterator<Item = TypeDefId>,
        mode: &BorrowMode,
    ) -> TokenStream {
        let mut typedefs_out = Vec::new();

        for id in ids {
            let typedef = &self.interface().typedefs[id];
            let info = self.infos()[id];
            let variants = self.variants_of(&typedef.ident.to_upper_camel_case(), info, mode);

            for TypeVariant { ident, borrow_mode } in variants {
                let docs = &typedef.docs;

                log::debug!(
                    "generating {:?} with mode info {:?} and mode {:?}",
                    ident,
                    info,
                    mode
                );

                let typedef = match &typedef.kind {
                    TypeDefKind::Alias(ty) => {
                        self.print_alias(docs, &ident, ty, info, &borrow_mode)
                    }
                    TypeDefKind::Record(fields) => {
                        self.print_record(docs, &ident, fields, info, &borrow_mode)
                    }
                    TypeDefKind::Flags(fields) => self.print_flags(docs, &ident, fields, info),
                    TypeDefKind::Variant(cases) => {
                        self.print_variant(docs, &ident, cases, info, &borrow_mode)
                    }
                    TypeDefKind::Enum(cases) => self.print_enum(docs, &ident, cases, info),
                    TypeDefKind::Union(cases) => {
                        self.print_union(docs, &ident, cases, info, &borrow_mode)
                    }
                    TypeDefKind::Resource(functions) => {
                        self.print_resource(&self.interface().ident, docs, &ident, functions, info)
                    }
                };

                typedefs_out.push(typedef);
            }
        }

        quote! { #(#typedefs_out)* }
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

    fn print_alias(
        &self,
        docs: &str,
        ident: &Ident,
        ty: &Type,
        info: TypeInfo,
        mode: &BorrowMode,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let ty = self.print_ty(ty, mode);
        let generics = print_generics(info, mode);

        quote! {
            #docs
            pub type #ident #generics = #ty;
        }
    }

    fn print_record(
        &self,
        docs: &str,
        ident: &Ident,
        fields: &[RecordField],
        info: TypeInfo,
        mode: &BorrowMode,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let generics = print_generics(info, mode);
        let fields = fields
            .iter()
            .map(|field| self.print_record_field(field, mode));

        quote! {
            #docs
            #additional_attrs
            #[derive(Debug, Clone, PartialEq)]
            pub struct #ident #generics {
                #(#fields),*
            }
        }
    }

    fn print_record_field(&self, field: &RecordField, mode: &BorrowMode) -> TokenStream {
        let docs = self.print_docs(&field.docs);
        let borrow_attr = self
            .needs_borrow(&field.ty, mode)
            .then_some(quote! { #[serde(borrow)] });
        let ident = format_ident!("{}", field.ident.to_snake_case());
        let ty = self.print_ty(&field.ty, mode);

        quote! {
            #docs
            #borrow_attr
            pub #ident: #ty
        }
    }

    fn print_flags(
        &self,
        docs: &str,
        ident: &Ident,
        fields: &[FlagsField],
        info: TypeInfo,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let repr = self.print_int(&flags_repr(fields));

        let fields = fields
            .iter()
            .enumerate()
            .map(|(i, FlagsField { docs, ident })| {
                let docs = self.print_docs(docs);
                let ident = format_ident!("{}", ident.TO_SHOUTY_SNEK_CASE());
                let i = Literal::usize_unsuffixed(i);

                quote! {
                    #docs
                    const #ident = 1 << #i;
                }
            });

        quote! {
            bitflags::bitflags! {
                #docs
                #additional_attrs
                pub struct #ident: #repr {
                    #(#fields)*
                }
            }
        }
    }

    fn print_variant(
        &self,
        docs: &str,
        ident: &Ident,
        cases: &[VariantCase],
        info: TypeInfo,
        mode: &BorrowMode,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let generics = print_generics(info, mode);
        let cases = cases.iter().map(|case| self.print_variant_case(case, mode));

        quote! {
            #docs
            #additional_attrs
            #[derive(Debug, Clone, PartialEq)]
            pub enum #ident #generics {
                #(#cases),*
            }
        }
    }

    fn print_variant_case(&self, case: &VariantCase, mode: &BorrowMode) -> TokenStream {
        let docs = self.print_docs(&case.docs);
        let ident = format_ident!("{}", case.ident.to_upper_camel_case());

        let payload = case.ty.as_ref().map(|ty| {
            let ty = self.print_ty(ty, mode);

            quote! { (#ty) }
        });

        quote! {
            #docs
            #ident #payload
        }
    }

    fn print_enum(
        &self,
        docs: &str,
        ident: &Ident,
        cases: &[EnumCase],
        info: TypeInfo,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let cases = cases.iter().map(|case| self.print_enum_case(case));

        quote! {
            #docs
            #additional_attrs
            #[derive(Debug, Clone, PartialEq)]
            pub enum #ident {
                #(#cases),*
            }
        }
    }

    fn print_enum_case(&self, case: &EnumCase) -> TokenStream {
        let docs = self.print_docs(&case.docs);
        let ident = format_ident!("{}", case.ident.to_upper_camel_case());

        quote! {
            #docs
            #ident
        }
    }

    fn print_union(
        &self,
        docs: &str,
        ident: &Ident,
        cases: &[UnionCase],
        info: TypeInfo,
        mode: &BorrowMode,
    ) -> TokenStream {
        let docs = self.print_docs(docs);
        let additional_attrs = self.additional_attrs(&ident.to_string(), info);
        let generics = print_generics(info, mode);

        let cases = union_case_names(&self.interface().typedefs, cases)
            .into_iter()
            .zip(cases)
            .map(|(name, case)| {
                let docs = self.print_docs(&case.docs);
                let ident = format_ident!("{}", name);
                let ty = self.print_ty(&case.ty, mode);

                quote! {
                    #docs
                    #ident (#ty)
                }
            });

        quote! {
            #docs
            #additional_attrs
            #[derive(Debug, Clone, PartialEq)]
            pub enum #ident #generics {
                #(#cases),*
            }
        }
    }

    fn print_function_signature(
        &self,
        sig: &FnSig,
        param_mode: &BorrowMode,
        results_mode: &BorrowMode,
    ) -> TokenStream {
        let docs = self.print_docs(&sig.func.docs);
        let ident = format_ident!("{}", sig.func.ident.to_snake_case());

        let pub_ = (!sig.private).then_some(quote! { pub });
        let unsafe_ = sig.unsafe_.then_some(quote! { unsafe });
        let async_ = sig.async_.then_some(quote! { async });

        let self_arg = sig.self_arg.as_ref().map(|arg| quote! { #arg, });

        let params = self.print_function_params(&sig.func.params, param_mode);

        let result = sig
            .func
            .result
            .as_ref()
            .map(|result| self.print_function_result(result, results_mode));

        quote! {
            #docs
            #pub_ #unsafe_ #async_ fn #ident (#self_arg #params) #result
        }
    }

    fn print_function_params(&self, params: &[(String, Type)], mode: &BorrowMode) -> TokenStream {
        let params = params.iter().map(|(ident, ty)| {
            let ident = format_ident!("{}", ident.to_snake_case());
            let ty = self.print_ty(ty, mode);

            quote! { #ident: #ty }
        });

        quote! { #(#params),* }
    }

    fn print_function_result(&self, result: &FunctionResult, mode: &BorrowMode) -> TokenStream {
        match result {
            FunctionResult::Anon(ty) => {
                let ty = self.print_ty(ty, mode);

                quote! { -> #ty }
            }
            FunctionResult::Named(types) if types.is_empty() => quote! {},
            FunctionResult::Named(types) if types.len() == 1 => {
                let (_, ty) = &types[0];
                let ty = self.print_ty(ty, mode);

                quote! { -> #ty }
            }
            FunctionResult::Named(types) => {
                let types = types.iter().map(|(_, ty)| self.print_ty(ty, mode));

                quote! { -> (#(#types),*) }
            }
        }
    }

    fn print_docs(&self, docs: &str) -> TokenStream {
        (!docs.is_empty())
            .then_some(quote! {
                #[doc = #docs]
            })
            .unwrap_or_default()
    }

    fn print_int(&self, int: &Int) -> TokenStream {
        match int {
            Int::U8 => quote! { u8 },
            Int::U16 => quote! { u16 },
            Int::U32 => quote! { u32 },
            Int::U64 => quote! { u64 },
            Int::U128 => quote! { u128 },
        }
    }

    fn uses_two_names(&self, info: TypeInfo) -> bool {
        info.contains(TypeInfo::HAS_LIST)
            && info.contains(TypeInfo::PARAM | TypeInfo::RESULT)
            && match self.default_param_mode() {
                BorrowMode::AllBorrowed(_) | BorrowMode::LeafBorrowed(_) => true,
                BorrowMode::Owned => false,
            }
    }

    fn variants_of(
        &self,
        ident: &str,
        info: TypeInfo,
        default_mode: &BorrowMode,
    ) -> Vec<TypeVariant> {
        let mut result = Vec::new();

        if !self.uses_two_names(info) {
            return vec![TypeVariant {
                ident: format_ident!("{ident}"),
                borrow_mode: default_mode.clone(),
            }];
        }

        if info.contains(TypeInfo::PARAM) {
            result.push(TypeVariant {
                ident: format_ident!("{ident}Param"),
                borrow_mode: default_mode.clone(),
            });
        }
        if info.contains(TypeInfo::RESULT)
            && (!info.contains(TypeInfo::PARAM) || self.uses_two_names(info))
        {
            result.push(TypeVariant {
                ident: format_ident!("{ident}Result"),
                borrow_mode: BorrowMode::Owned,
            });
        }
        result
    }

    fn needs_borrow(&self, ty: &Type, mode: &BorrowMode) -> bool {
        match ty {
            Type::Id(id) => {
                let info = self.infos()[*id];

                lifetime_for(info, mode).is_some()
            }
            Type::Tuple(types) => types.iter().any(|ty| self.needs_borrow(ty, mode)),
            Type::List(ty) | Type::Option(ty) => self.needs_borrow(ty, mode),
            _ => false,
        }
    }
}

pub struct FnSig<'a> {
    pub async_: bool,
    pub unsafe_: bool,
    pub private: bool,
    pub self_arg: Option<TokenStream>,
    pub func: &'a Function,
}

#[must_use]
pub fn lifetime_for(info: TypeInfo, mode: &BorrowMode) -> Option<&Lifetime> {
    match mode {
        BorrowMode::AllBorrowed(s) | BorrowMode::LeafBorrowed(s)
            if info.contains(TypeInfo::HAS_LIST) =>
        {
            Some(s)
        }
        _ => None,
    }
}

#[must_use]
pub fn print_generics(info: TypeInfo, mode: &BorrowMode) -> Option<TokenStream> {
    let lt = lifetime_for(info, mode);

    lt.map(|lt| {
        quote! {
            <#lt>
        }
    })
}

#[derive(Clone, PartialEq, Eq)]
pub enum BorrowMode {
    Owned,
    AllBorrowed(Lifetime),
    LeafBorrowed(Lifetime),
}

impl std::fmt::Debug for BorrowMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Owned => write!(f, "Owned"),
            Self::AllBorrowed(_) => f.debug_tuple("AllBorrowed").finish(),
            Self::LeafBorrowed(_) => f.debug_tuple("LeafBorrowed").finish(),
        }
    }
}

#[derive(Debug)]
pub struct TypeVariant {
    pub ident: Ident,
    pub borrow_mode: BorrowMode,
}

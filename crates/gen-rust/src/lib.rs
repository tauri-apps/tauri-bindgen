use heck::{ToShoutySnekCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use std::{collections::HashMap, ops::Index};
use syn::Lifetime;

use wit_parser::{
    EnumCase, FlagsField, Function, FunctionResult, Int, Interface, RecordField, Type,
    TypeDefArena, TypeDefId, TypeDefKind, UnionCase, VariantCase,
};

pub trait RustGenerator {
    fn interface(&self) -> &Interface;
    fn infos(&self) -> &TypeInfos;
    fn additional_attrs(&self, ident: &str, info: TypeInfo) -> Option<TokenStream>;
    fn default_param_mode(&self) -> BorrowMode;

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
            Type::S8 => quote! { i8 },
            Type::S16 => quote! { i16 },
            Type::S32 => quote! { i32 },
            Type::S64 => quote! { i64 },
            Type::Float32 => quote! { f32 },
            Type::Float64 => quote! { f64 },
            Type::Char => quote! { char },
            Type::String => match mode {
                BorrowMode::Owned => quote! { String },
                BorrowMode::AllBorrowed(lt) | BorrowMode::LeafBorrowed(lt) => quote! { &#lt str },
            },
            Type::List(ty) => {
                let is_primitive = match **ty {
                    Type::U8
                    | Type::S8
                    | Type::U16
                    | Type::S16
                    | Type::U32
                    | Type::S32
                    | Type::U64
                    | Type::S64
                    | Type::Float32
                    | Type::Float64 => true,
                    _ => false,
                };

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
            #ident: #ty
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

        let cases = self
            .union_case_names(cases)
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

        let result = self.print_function_result(&sig.func.result, results_mode);

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
        if result.is_empty() {
            return quote! {};
        }

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
        }
    }

    fn type_ident(&self, ty: &Type) -> String {
        match ty {
            Type::Bool => "Bool".to_string(),
            Type::U8 => "U8".to_string(),
            Type::U16 => "U16".to_string(),
            Type::U32 => "U32".to_string(),
            Type::U64 => "U64".to_string(),
            Type::S8 => "I8".to_string(),
            Type::S16 => "I16".to_string(),
            Type::S32 => "S32".to_string(),
            Type::S64 => "S64".to_string(),
            Type::Float32 => "F32".to_string(),
            Type::Float64 => "F64".to_string(),
            Type::Char => "Char".to_string(),
            Type::String => "String".to_string(),
            Type::List(ty) => format!("{}List", self.type_ident(ty)),
            Type::Tuple(_) => "Tuple".to_string(),
            Type::Option(ty) => format!("Optional{}", self.type_ident(ty)),
            Type::Result { .. } => "Result".to_string(),
            Type::Id(id) => match &self.interface().typedefs[*id].kind {
                TypeDefKind::Alias(ty) => self.type_ident(ty),
                TypeDefKind::Record(_) => "Record".to_string(),
                TypeDefKind::Flags(_) => "Flags".to_string(),
                TypeDefKind::Variant(_) => "Variant".to_string(),
                TypeDefKind::Enum(_) => "Enum".to_string(),
                TypeDefKind::Union(_) => "Union".to_string(),
            },
        }
    }

    fn union_case_names(&self, cases: &[UnionCase]) -> Vec<String> {
        enum UsedState<'a> {
            /// This name has been used once before.
            ///
            /// Contains a reference to the name given to the first usage so that a suffix can be added to it.
            Once(&'a mut String),
            /// This name has already been used multiple times.
            ///
            /// Contains the number of times this has already been used.
            Multiple(usize),
        }

        // A `Vec` of the names we're assigning each of the union's cases in order.
        let mut case_names = vec![String::new(); cases.len()];
        // A map from case names to their `UsedState`.
        let mut used = HashMap::new();
        for (case, name) in cases.iter().zip(case_names.iter_mut()) {
            name.push_str(&self.type_ident(&case.ty));

            match used.get_mut(name.as_str()) {
                None => {
                    // Initialise this name's `UsedState`, with a mutable reference to this name
                    // in case we have to add a suffix to it later.
                    used.insert(name.clone(), UsedState::Once(name));
                    // Since this is the first (and potentially only) usage of this name,
                    // we don't need to add a suffix here.
                }
                Some(state) => match state {
                    UsedState::Multiple(n) => {
                        // Add a suffix of the index of this usage.
                        name.push_str(&n.to_string());
                        // Add one to the number of times this type has been used.
                        *n += 1;
                    }
                    UsedState::Once(first) => {
                        // Add a suffix of 0 to the first usage.
                        first.push('0');
                        // We now get a suffix of 1.
                        name.push('1');
                        // Then update the state.
                        *state = UsedState::Multiple(2);
                    }
                },
            }
        }

        case_names
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

#[must_use]
pub fn flags_repr(fields: &[FlagsField]) -> Int {
    match fields.len() {
        n if n <= 8 => Int::U8,
        n if n <= 16 => Int::U16,
        n if n <= 32 => Int::U32,
        n if n <= 64 => Int::U64,
        _ => panic!("too many flags to fit in a repr"),
    }
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

// #[must_use]
// pub fn uses_two_names(info: TypeInfo) -> bool {

//     // info.contains(TypeInfo::HAS_LIST) && info.contains(TypeInfo::PARAM | TypeInfo::RESULT)
// }

bitflags::bitflags! {
    #[derive(Default)]
    pub struct TypeInfo: u32 {
        /// Whether or not this type is ever used (transitively) within the
        /// parameter of a function.
        const PARAM = 0b0000_0001;
        /// Whether or not this type is ever used (transitively) within the
        /// result of a function.
        const RESULT = 0b0000_0010;
        /// Whether or not this type (transitively) has a list.
        const HAS_LIST = 0b0000_1000;
    }
}

#[derive(Debug)]
pub struct TypeInfos {
    infos: HashMap<TypeDefId, TypeInfo>,
}

impl TypeInfos {
    #[must_use]
    pub fn new() -> Self {
        TypeInfos {
            infos: HashMap::new(),
        }
    }

    pub fn collect_param_info(&mut self, typedefs: &TypeDefArena, params: &[(String, Type)]) {
        for (_, ty) in params {
            self.collect_type_info(typedefs, ty, TypeInfo::PARAM);
        }
    }

    pub fn collect_result_info(&mut self, typedefs: &TypeDefArena, result: &FunctionResult) {
        match result {
            FunctionResult::Anon(ty) => {
                self.collect_type_info(typedefs, ty, TypeInfo::RESULT);
            }
            FunctionResult::Named(results) => {
                for (_, ty) in results {
                    self.collect_type_info(typedefs, ty, TypeInfo::RESULT);
                }
            }
        }
    }

    fn collect_typedef_info(
        &mut self,
        typedefs: &TypeDefArena,
        id: TypeDefId,
        base_info: TypeInfo,
    ) -> TypeInfo {
        let mut info = base_info;

        match &typedefs[id].kind {
            TypeDefKind::Alias(ty) => {
                info |= self.collect_type_info(typedefs, ty, base_info);
            }
            TypeDefKind::Record(fields) => {
                for field in fields {
                    info |= self.collect_type_info(typedefs, &field.ty, base_info);
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(ty) = &case.ty {
                        info |= self.collect_type_info(typedefs, ty, base_info);
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    info |= self.collect_type_info(typedefs, &case.ty, base_info);
                }
            }
            _ => {}
        }

        log::debug!("collected info for {:?}: {:?}", typedefs[id].ident, info,);

        self.infos
            .entry(id)
            .and_modify(|i| *i |= info)
            .or_insert(info);

        info
    }

    fn collect_type_info(
        &mut self,
        typedefs: &TypeDefArena,
        ty: &Type,
        base_info: TypeInfo,
    ) -> TypeInfo {
        match ty {
            Type::String => base_info | TypeInfo::HAS_LIST,
            Type::List(ty) => self.collect_type_info(typedefs, ty, base_info) | TypeInfo::HAS_LIST,
            Type::Option(ty) => self.collect_type_info(typedefs, ty, base_info),
            Type::Tuple(types) => {
                let mut info = base_info;
                for ty in types {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                info
            }
            Type::Result { ok, err } => {
                let mut info = base_info;
                if let Some(ty) = &ok {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                if let Some(ty) = &err {
                    info |= self.collect_type_info(typedefs, ty, base_info);
                }
                info
            }
            Type::Id(id) => base_info | self.collect_typedef_info(typedefs, *id, base_info),
            _ => base_info,
        }
    }
}

impl Index<TypeDefId> for TypeInfos {
    type Output = TypeInfo;

    fn index(&self, id: TypeDefId) -> &Self::Output {
        &self.infos[&id]
    }
}

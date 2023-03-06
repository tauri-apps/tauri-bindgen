use std::collections::HashMap;
use heck::{ToShoutySnekCase, ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};
use syn::Lifetime;

use crate::typecheck::{
    EnumCase, FlagsField, Function, FunctionResult, Int, RecordField, Type, TypeDef,
    TypeDefKind, TypeInfo, UnionCase, VariantCase,
};

pub struct RustGenerator {
    default_param_mode: BorrowMode,
}

impl RustGenerator {
    fn print_typedefs<'a>(&self, typedefs: impl Iterator<Item = &'a TypeDef>) -> TokenStream {
        let typedefs = typedefs.map(|typedef| {
            let docs = &typedef.docs;
            let ident = format_ident!("{}", &typedef.ident.to_upper_camel_case());

            match &typedef.kind {
                TypeDefKind::Alias(ty) => self.print_alias(docs, &ident, &ty),
                TypeDefKind::Record(fields) => self.print_record(docs, &ident, &fields),
                TypeDefKind::Flags(fields) => self.print_flags(docs, &ident, &fields),
                TypeDefKind::Variant(cases) => self.print_variant(docs, &ident, &cases),
                TypeDefKind::Enum(cases) => self.print_enum(docs, &ident, &cases),
                TypeDefKind::Union(cases) => self.print_union(docs, &ident, &cases),
            }
        });

        quote! { #(#typedefs)* }
    }

    // fn print_interface(&self, interface: &Interface) -> TokenStream {
    //     let docs = self.print_docs(&interface.docs);
    //     let ident = format_ident!("{}", interface.ident.to_snake_case());

    //     let typedefs = interface.typedefs.iter().flat_map(|typedef| {

    //         variants_of(&ident, typedef.info, &self.default_param_mode)
    //             .into_iter()
    //             .map(|variant| )
    //             .collect::<Vec<_>>()
    //     });

    //     let functions = interface
    //         .functions
    //         .iter()
    //         .map(|func| self.print_function(func));

    //     quote! {
    //         #docs
    //         mod #ident {
    //             #(#typedefs)*

    //             #(#functions)*
    //         }
    //     }
    // }

    fn print_ty(&self, ty: &Type) -> TokenStream {
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
            Type::String => quote! { &str },
            Type::List(ty) => {
                let ty = self.print_ty(ty);

                quote! { &[#ty] }
            }
            Type::Tuple(types) => {
                let types = types.iter().map(|ty| self.print_ty(ty));

                quote! { (#(#types),*) }
            }
            Type::Option(ty) => {
                let ty = self.print_ty(ty);

                quote! { Option<#ty> }
            }
            Type::Result { ok, err } => {
                let ok = ok
                    .as_ref()
                    .map(|ty| self.print_ty(ty))
                    .unwrap_or(quote! { () });
                let err = err
                    .as_ref()
                    .map(|ty| self.print_ty(ty))
                    .unwrap_or(quote! { () });

                quote! { Result<#ok, #err> }
            }
            Type::Id(typedef) => {
                let ident = format_ident!("{}", typedef.borrow().ident.to_upper_camel_case());
                quote! { #ident }
            }
        }
    }

    fn print_alias(&self, docs: &str, ident: &Ident, ty: &Type) -> TokenStream {
        let docs = self.print_docs(docs);
        let ty = self.print_ty(ty);
        let generics = print_generics();

        quote! {
            #docs
            pub type #ident #generics = #ty;
        }
    }

    fn print_record(&self, docs: &str, ident: &Ident, fields: &[RecordField]) -> TokenStream {
        let docs = self.print_docs(docs);
        let fields = fields.iter().map(|field| self.print_record_field(field));
        let generics = print_generics();

        quote! {
            #docs
            pub struct #ident #generics {
                #(#fields),*
            }
        }
    }

    fn print_record_field(&self, field: &RecordField) -> TokenStream {
        let docs = self.print_docs(&field.docs);
        let ident = format_ident!("{}", field.ident.to_snake_case());
        let ty = self.print_ty(&field.ty);

        quote! {
            #docs
            #ident: #ty
        }
    }

    fn print_flags(&self, docs: &str, ident: &Ident, fields: &[FlagsField]) -> TokenStream {
        let docs = self.print_docs(docs);
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
                    pub const #ident = 1 << #i;
                }
            });

        quote! {
            bitflags::bitflags! {
                #docs
                pub struct #ident: #repr {
                    #(#fields)*
                }
            }
        }
    }

    fn print_variant(&self, docs: &str, ident: &Ident, cases: &[VariantCase]) -> TokenStream {
        let docs = self.print_docs(docs);
        let cases = cases.iter().map(|case| self.print_variant_case(case));
        let generics = print_generics();

        quote! {
            #docs
            pub enum #ident #generics {
                #(#cases),*
            }
        }
    }

    fn print_variant_case(&self, case: &VariantCase) -> TokenStream {
        let docs = self.print_docs(&case.docs);
        let ident = format_ident!("{}", case.ident.to_snake_case());

        let payload = case.ty.as_ref().map(|ty| {
            let ty = self.print_ty(ty);

            quote! { (#ty) }
        });

        quote! {
            #docs
            #ident #payload
        }
    }

    fn print_enum(&self, docs: &str, ident: &Ident, cases: &[EnumCase]) -> TokenStream {
        let docs = self.print_docs(docs);
        let cases = cases.iter().map(|case| self.print_enum_case(case));

        quote! {
            #docs
            pub enum #ident {
                #(#cases),*
            }
        }
    }

    fn print_enum_case(&self, case: &EnumCase) -> TokenStream {
        let docs = self.print_docs(&case.docs);
        let ident = format_ident!("{}", case.ident.to_snake_case());

        quote! {
            #docs
            #ident
        }
    }

    fn print_union(&self, docs: &str, ident: &Ident, cases: &[UnionCase]) -> TokenStream {
        let docs = self.print_docs(docs);
        let generics = print_generics();

        let cases = union_case_names(cases)
            .into_iter()
            .zip(cases)
            .map(|(name, case)| {
                let docs = self.print_docs(&case.docs);
                let ident = format_ident!("{}", name);
                let ty = self.print_ty(&case.ty);

                quote! {
                    #docs
                    #ident (#ty)
                }
            });

        quote! {
            #docs
            pub enum #ident #generics {
                #(#cases),*
            }
        }
    }

    fn print_function_signature(&self, func: &Function) -> TokenStream {
        let docs = self.print_docs(&func.docs);
        let ident = format_ident!("{}", func.ident.to_snake_case());

        let params = func.params.iter().map(|(ident, ty)| {
            let ident = format_ident!("{}", ident.to_snake_case());
            let ty = self.print_ty(ty);

            quote! { #ident: #ty }
        });

        let result = self.print_function_result(&func.result);
        
        quote! { 
            #docs
            pub fn #ident (#(#params),*) #result
        }
    }

    fn print_function_result(&self, result: &FunctionResult) -> TokenStream {
        match result {
            FunctionResult::Anon(ty) => {
                let ty = self.print_ty(ty);

                quote! { -> #ty }
            }
            FunctionResult::Named(types) if types.is_empty() => quote! {},
            FunctionResult::Named(types) if types.len() == 1 => {
                let (_, ty) = &types[0];
                let ty = self.print_ty(ty);

                quote! { -> #ty }
            }
            FunctionResult::Named(types) => {
                let types = types.iter().map(|(_, ty)| self.print_ty(ty));

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
}

fn print_generics() -> Option<TokenStream> {
    todo!()
}

fn flags_repr(fields: &[FlagsField]) -> Int {
    match fields.len() {
        n if n <= 8 => Int::U8,
        n if n <= 16 => Int::U16,
        n if n <= 32 => Int::U32,
        n if n <= 64 => Int::U64,
        _ => panic!("too many flags to fit in a repr"),
    }
}

fn union_case_names(cases: &[UnionCase]) -> Vec<String> {
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
        name.push_str(&type_ident(&case.ty));

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

fn type_ident(ty: &Type) -> String {
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
        Type::List(ty) => format!("{}List", type_ident(ty)),
        Type::Tuple(_) => "Tuple".to_string(),
        Type::Option(ty) => format!("Optional{}", type_ident(ty)),
        Type::Result { .. } => "Result".to_string(),
        Type::Id(typedef) => match &typedef.borrow().kind {
            TypeDefKind::Alias(ty) => type_ident(ty),
            TypeDefKind::Record(_) => "Record".to_string(),
            TypeDefKind::Flags(_) => "Flags".to_string(),
            TypeDefKind::Variant(_) => "Variant".to_string(),
            TypeDefKind::Enum(_) => "Enum".to_string(),
            TypeDefKind::Union(_) => "Union".to_string(),
        },
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum BorrowMode {
    Owned,
    AllBorrowed(Lifetime),
    LeafBorrowed(Lifetime),
}

pub struct TypeVariant {
    ident: Ident,
    borrow_mode: BorrowMode,
}

fn variants_of(ident: &str, info: TypeInfo, default_mode: &BorrowMode) -> Vec<TypeVariant> {
    let mut result = Vec::new();
    if info.contains(TypeInfo::PARAM) {
        result.push(TypeVariant {
            ident: format_ident!("{ident}Param"),
            borrow_mode: default_mode.clone(),
        });
    }
    if info.contains(TypeInfo::RESULT)
        && (!info.contains(TypeInfo::PARAM) || uses_two_names(&info, default_mode))
    {
        result.push(TypeVariant {
            ident: format_ident!("{ident}Result"),
            borrow_mode: BorrowMode::Owned,
        });
    }
    result
}

fn uses_two_names(info: &TypeInfo, default_mode: &BorrowMode) -> bool {
    info.contains(TypeInfo::HAS_LIST)
        && info.contains(TypeInfo::PARAM | TypeInfo::RESULT)
        && match default_mode {
            BorrowMode::AllBorrowed(_) | BorrowMode::LeafBorrowed(_) => true,
            BorrowMode::Owned => false,
        }
}

#[cfg(test)]
mod test {
    use crate::{
        parse::{self, FromTokens},
        typecheck::Resolver,
        Result,
    };
    use logos::Lexer;

    // #[test]
    // fn full() -> Result<()> {
    //     let source = include_str!("test.wit");

    //     let mut tokens = Lexer::new(source).spanned().peekable();

    //     let iface = parse::Interface::parse(&mut tokens)?;
    //     let iface = Resolver::new(source, &iface).resolve()?;

    //     // println!("{}", iface.into_token_stream().to_string());
    //     let syntax_tree = syn::parse2(print_interface(&iface)).unwrap();
    //     let formatted = prettyplease::unparse(&syntax_tree);
    //     println!("{}", formatted);

    //     Ok(())
    // }
}

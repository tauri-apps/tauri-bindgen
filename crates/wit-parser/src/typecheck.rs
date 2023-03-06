use crate::{Error, Result};
use logos::Span;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    ops::Deref,
    rc::Rc,
};

use crate::parse;

type TypeDefRef = Rc<RefCell<TypeDef>>;

pub enum Int {
    U8,
    U16,
    U32,
    U64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    pub docs: String,
    pub ident: String,
    pub typedefs: Vec<TypeDefRef>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    S8,
    S16,
    S32,
    S64,
    Float32,
    Float64,
    Char,
    String,
    List(Box<Type>),
    Tuple(Vec<Type>),
    Option(Box<Type>),
    Result {
        ok: Option<Box<Type>>,
        err: Option<Box<Type>>,
    },
    Id(TypeDefRef),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef {
    pub info: TypeInfo,
    pub docs: String,
    pub ident: String,
    pub kind: TypeDefKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefKind {
    Alias(Type),
    Record(Vec<RecordField>),
    Flags(Vec<FlagsField>),
    Variant(Vec<VariantCase>),
    Enum(Vec<EnumCase>),
    Union(Vec<UnionCase>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub docs: String,
    pub ident: String,
    pub ty: Type,
}

// impl Flags {
//     pub fn repr(&self) -> Int {
//         match self.fields.len() {
//             n if n <= 8 => Int::U8,
//             n if n <= 16 => Int::U16,
//             n if n <= 32 => Int::U32,
//             n if n <= 64 => Int::U64,
//             _ => panic!("too many flags to fit in a repr"),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsField {
    pub docs: String,
    pub ident: String,
}

// impl Variant {
//     pub fn tag(&self) -> Int {
//         match self.cases.len() {
//             n if u8::try_from(n).is_ok() => Int::U8,
//             n if u16::try_from(n).is_ok() => Int::U16,
//             n if u32::try_from(n).is_ok() => Int::U32,
//             n if u64::try_from(n).is_ok() => Int::U64,
//             _ => panic!("too many cases to fit in a repr"),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub docs: String,
    pub ident: String,
    pub ty: Option<Type>,
}

// impl Enum {
//     pub fn tag(&self) -> Int {
//         match self.cases.len() {
//             n if u8::try_from(n).is_ok() => Int::U8,
//             n if u16::try_from(n).is_ok() => Int::U16,
//             n if u32::try_from(n).is_ok() => Int::U32,
//             n if u64::try_from(n).is_ok() => Int::U64,
//             _ => panic!("too many cases to fit in a repr"),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub docs: String,
    pub ident: String,
}

// impl Union {
//     pub fn tag(&self) -> Int {
//         match self.cases.len() {
//             n if u8::try_from(n).is_ok() => Int::U8,
//             n if u16::try_from(n).is_ok() => Int::U16,
//             n if u32::try_from(n).is_ok() => Int::U32,
//             n if u64::try_from(n).is_ok() => Int::U64,
//             _ => panic!("too many cases to fit in a repr"),
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionCase {
    pub docs: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub docs: String,
    pub ident: String,
    pub params: NamedTypeList,
    pub result: FunctionResult,
}

pub type NamedTypeList = Vec<(String, Type)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionResult {
    Anon(Type),
    Named(NamedTypeList),
}

impl FunctionResult {
    pub fn types(&self) -> ResultsTypeIter {
        match self {
            FunctionResult::Named(ps) => ResultsTypeIter::Named(ps.iter()),
            FunctionResult::Anon(ty) => ResultsTypeIter::Anon(std::iter::once(ty)),
        }
    }
}

pub enum ResultsTypeIter<'a> {
    Anon(std::iter::Once<&'a Type>),
    Named(std::slice::Iter<'a, (String, Type)>),
}

impl<'a> Iterator for ResultsTypeIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ResultsTypeIter::Anon(iter) => iter.next(),
            ResultsTypeIter::Named(iter) => iter.next().map(|pair| &pair.1),
        }
    }
}

pub struct Resolver<'a> {
    source: &'a str,
    interface: &'a parse::Interface,
    id2interface_item: HashMap<&'a str, &'a parse::InterfaceItem>,
    id2typedefref: HashMap<&'a str, TypeDefRef>,
}

impl<'a> Resolver<'a> {
    pub fn new(source: &'a str, interface: &'a parse::Interface) -> Self {
        let typedefs: HashMap<_, _> = interface
            .items
            .iter()
            .filter_map(|item| {
                if let parse::InterfaceItemInner::Func(_) = &item.inner {
                    None
                } else {
                    Some((&source[item.ident.clone()], item))
                }
            })
            .collect();

        Self {
            source,
            interface,
            id2typedefref: HashMap::with_capacity(typedefs.len()),
            id2interface_item: typedefs,
        }
    }

    fn read_span(&self, span: &Span) -> &'a str {
        &self.source[span.clone()]
    }

    fn resolve_docs(&self, docs: &[Span]) -> String {
        docs.iter()
            .map(|span| {
                let str = self.read_span(span);
                let str = str.strip_prefix("///").unwrap_or(str);
                let str = str.strip_prefix("//*").unwrap_or(str);
                let str = str.trim();

                str
            })
            .collect()
    }

    fn resolve_typedef(
        &mut self,
        typedef: &parse::InterfaceItem,
    ) -> Result<(TypeDefRef, TypeInfo)> {
        let ident_ref = self.read_span(&typedef.ident);

        if let Some(typedef) = self.id2typedefref.get(ident_ref) {
            return Ok((typedef.clone(), typedef.borrow().info));
        }

        let ident = ident_ref.to_string();

        let docs = self.resolve_docs(&typedef.docs);

        let (kind, info) = match &typedef.inner {
            parse::InterfaceItemInner::Alias(ty) => {
                let (ty, info) = self.resolve_type(ty)?;

                (TypeDefKind::Alias(ty), info)
            }
            parse::InterfaceItemInner::Record(fields) => {
                let (fields, info) = fields
                    .iter()
                    .map(|field| {
                        let docs = self.resolve_docs(&field.docs);
                        let ident = self.read_span(&field.ident).to_string();
                        let (ty, info) = self.resolve_type(&field.ty).unwrap();

                        (RecordField { docs, ident, ty }, info)
                    })
                    .unzip();

                (TypeDefKind::Record(fields), info)
            }
            parse::InterfaceItemInner::Flags(fields) => {
                let fields = fields.iter().map(|field| {
                    let docs = self.resolve_docs(&field.docs);
                    let ident = self.read_span(&field.ident).to_string();

                    FlagsField { docs, ident }
                });

                (TypeDefKind::Flags(fields.collect()), TypeInfo::empty())
            }
            parse::InterfaceItemInner::Variant(cases) => {
                let (cases, info) = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let ident = self.read_span(&case.ident).to_string();
                        let (ty, info) = case
                            .ty
                            .as_ref()
                            .map(|ty| self.resolve_type(ty).unwrap())
                            .unzip();

                        let info = info.unwrap_or_default();

                        (VariantCase { docs, ident, ty }, info)
                    })
                    .unzip();

                (TypeDefKind::Variant(cases), info)
            }
            parse::InterfaceItemInner::Enum(cases) => {
                let cases = cases.iter().map(|case| {
                    let docs = self.resolve_docs(&case.docs);
                    let ident = self.read_span(&case.ident).to_string();

                    EnumCase { docs, ident }
                });

                (TypeDefKind::Enum(cases.collect()), TypeInfo::empty())
            }
            parse::InterfaceItemInner::Union(cases) => {
                let (cases, info) = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let (ty, info) = self.resolve_type(&case.ty).unwrap();

                        (UnionCase { docs, ty }, info)
                    })
                    .unzip();

                (TypeDefKind::Union(cases), info)
            }
            parse::InterfaceItemInner::Func(_) => unreachable!(),
        };

        let typedefref = Rc::new(RefCell::new(TypeDef {
            info,
            docs,
            ident,
            kind,
        }));

        self.id2typedefref.insert(ident_ref, typedefref.clone());

        Ok((typedefref, info))
    }

    fn resolve_type(&mut self, ty: &parse::Type) -> Result<(Type, TypeInfo)> {
        match ty {
            parse::Type::Bool => Ok((Type::Bool, TypeInfo::empty())),
            parse::Type::U8 => Ok((Type::U8, TypeInfo::empty())),
            parse::Type::U16 => Ok((Type::U16, TypeInfo::empty())),
            parse::Type::U32 => Ok((Type::U32, TypeInfo::empty())),
            parse::Type::U64 => Ok((Type::U64, TypeInfo::empty())),
            parse::Type::S8 => Ok((Type::S8, TypeInfo::empty())),
            parse::Type::S16 => Ok((Type::S16, TypeInfo::empty())),
            parse::Type::S32 => Ok((Type::S32, TypeInfo::empty())),
            parse::Type::S64 => Ok((Type::S64, TypeInfo::empty())),
            parse::Type::Float32 => Ok((Type::Float32, TypeInfo::empty())),
            parse::Type::Float64 => Ok((Type::Float64, TypeInfo::empty())),
            parse::Type::Char => Ok((Type::Char, TypeInfo::empty())),
            parse::Type::String => Ok((Type::String, TypeInfo::HAS_LIST)),
            parse::Type::List(ty) => {
                let (ty, info) = self.resolve_type(ty)?;

                Ok((Type::List(Box::new(ty)), info | TypeInfo::HAS_LIST))
            }
            parse::Type::Option(ty) => {
                let (ty, info) = self.resolve_type(ty)?;

                Ok((Type::Option(Box::new(ty)), info))
            }
            parse::Type::Tuple(types) => {
                let (types, info) = types
                    .iter()
                    .map(|ty| self.resolve_type(ty).unwrap())
                    .unzip();

                Ok((Type::Tuple(types), info))
            }
            parse::Type::Result { ok, err } => {
                let (ok, ok_info) = ok.as_ref().map(|ty| self.resolve_type(ty).unwrap()).unzip();

                let (err, err_info) = err
                    .as_ref()
                    .map(|ty| self.resolve_type(ty).unwrap())
                    .unzip();

                Ok((
                    Type::Result {
                        ok: ok.map(Box::new),
                        err: err.map(Box::new),
                    },
                    ok_info.unwrap_or_default() | err_info.unwrap_or_default(),
                ))
            }
            parse::Type::Id(span) => {
                let ident = self.read_span(span);
                let typedef = self.id2interface_item.get(ident).expect("undefined type");

                let (tyref, info) = self.resolve_typedef(&typedef.clone())?;

                Ok((Type::Id(tyref), info))
            }
        }
    }

    fn resolve_named_types(
        &mut self,
        named_types: &[(Span, parse::Type)],
    ) -> Result<(Vec<(String, Type)>, TypeInfo)> {
        Ok(named_types
            .iter()
            .map(|(ident, ty)| {
                let ident = self.read_span(&ident).to_string();
                let (ty, info) = self.resolve_type(&ty).unwrap();

                ((ident, ty), info)
            })
            .unzip())
    }

    fn resolve_func(
        &mut self,
        docs: &[Span],
        ident: &Span,
        func: &parse::Func,
    ) -> Result<Function> {
        let docs = self.resolve_docs(docs);
        let ident = self.read_span(ident).to_string();

        let (params, _) = self.resolve_named_types(&func.params)?;

        let results = match &func.results {
            parse::FuncResult::Anon(ty) => {
                let (ty, _) = self.resolve_type(ty)?;
                FunctionResult::Anon(ty)
            }
            parse::FuncResult::Named(types) => {
                let (types, _) = self.resolve_named_types(types)?;
                FunctionResult::Named(types)
            }
        };

        for (_, ty) in &params {
            self.mark_ty(ty, TypeInfo::PARAM);
        }

        for ty in results.types() {
            self.mark_ty(ty, TypeInfo::RESULT);
        }

        Ok(Function {
            docs,
            ident,
            params,
            result: results,
        })
    }

    fn mark_typedef(&self, typedef: &TypeDefRef, info: TypeInfo) {
        typedef.borrow_mut().info |= info;

        match &typedef.borrow().kind {
            TypeDefKind::Alias(ty) => self.mark_ty(ty, info),
            TypeDefKind::Record(fields) => {
                for field in fields {
                    self.mark_ty(&field.ty, info);
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(ty) = &case.ty {
                        self.mark_ty(ty, info);
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    self.mark_ty(&case.ty, info);
                }
            }
            _ => {}
        }
    }

    fn mark_ty(&self, ty: &Type, info: TypeInfo) {
        match ty {
            Type::List(ty) | Type::Option(ty) => self.mark_ty(ty, info),
            Type::Tuple(types) => {
                for ty in types {
                    self.mark_ty(ty, info);
                }
            }
            Type::Result { ok, err } => {
                if let Some(ty) = &ok {
                    self.mark_ty(ty, info);
                }
                if let Some(ty) = &err {
                    self.mark_ty(ty, info);
                }
            }
            Type::Id(typedef) => self.mark_typedef(typedef, info),
            _ => {}
        }
    }

    fn verify_not_recursive(
        &self,
        typedef: &TypeDefRef,
        visiting: &mut HashSet<*const RefCell<TypeDef>>,
        valid: &mut BTreeMap<String, TypeDefRef>,
    ) -> Result<()> {
        if valid.contains_key(&typedef.borrow().ident) {
            return Ok(());
        }

        if !visiting.insert(Rc::as_ptr(&typedef)) {
            return Err(Error::recursive_type(
                self.id2interface_item[typedef.borrow().ident.deref()]
                    .ident
                    .clone(),
            ));
        }

        match &self.id2typedefref[typedef.borrow().ident.deref()]
            .borrow()
            .kind
        {
            TypeDefKind::Record(fields) => {
                for field in fields {
                    if let Type::Id(typedef) = &field.ty {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    if let Type::Id(typedef) = &case.ty {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(Type::Id(typedef)) = &case.ty {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Alias(ty) => match ty {
                Type::Tuple(types) => {
                    for ty in types {
                        if let Type::Id(typedef) = ty {
                            self.verify_not_recursive(&typedef, visiting, valid)?;
                        }
                    }
                }
                Type::List(ty) | Type::Option(ty) => {
                    if let Type::Id(typedef) = &**ty {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }
                }
                Type::Result { ok, err } => {
                    if let Some(Type::Id(typedef)) = ok.as_deref() {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }

                    if let Some(Type::Id(typedef)) = err.as_deref() {
                        self.verify_not_recursive(typedef, visiting, valid)?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        valid.insert(typedef.borrow().ident.to_string(), typedef.clone());
        visiting.remove(&Rc::as_ptr(typedef));

        Ok(())
    }

    pub fn resolve(mut self) -> Result<Interface> {
        let docs = self.resolve_docs(&self.interface.docs);
        let ident = self.read_span(&self.interface.ident).to_string();

        let functions: Vec<_> = self
            .interface
            .items
            .iter()
            .filter_map(|item| {
                if let parse::InterfaceItemInner::Func(func) = &item.inner {
                    Some(self.resolve_func(&item.docs, &item.ident, func).unwrap())
                } else {
                    None
                }
            })
            .collect();

        let typedefs: Vec<_> = functions
            .iter()
            .flat_map(|func| {
                let param_typedefs = func.params.iter().flat_map(|(_, ty)| extract_typedefs(ty));

                let result_typedefs = func.result.types().flat_map(|ty| extract_typedefs(ty));

                param_typedefs.chain(result_typedefs)
            })
            .cloned()
            .collect();

        let mut visiting = HashSet::new();
        let mut valid_types = BTreeMap::new();
        for typedef in &typedefs {
            self.verify_not_recursive(typedef, &mut visiting, &mut valid_types)?;
        }

        Ok(Interface {
            docs,
            ident,
            functions,
            typedefs: valid_types.into_values().collect(),
        })
    }
}

fn extract_typedefs<'a>(ty: &'a Type) -> ExtractedTypes<'a> {
    match ty {
        Type::Id(typedef) => ExtractedTypes::One(std::iter::once(typedef)),
        Type::List(ty) | Type::Option(ty) => extract_typedefs(ty),
        Type::Tuple(types) => {
            let iter: Vec<_> = types.iter().flat_map(|ty| extract_typedefs(ty)).collect();

            ExtractedTypes::Many(iter.into_iter())
        }
        Type::Result { ok, err } => {
            let ok = ok.as_deref().into_iter();
            let err = err.as_deref().into_iter();

            let iter: Vec<_> = ok.chain(err).flat_map(|ty| extract_typedefs(ty)).collect();

            ExtractedTypes::Many(iter.into_iter())
        },
        _ => ExtractedTypes::None,
    }
}

enum ExtractedTypes<'a> {
    None,
    One(std::iter::Once<&'a TypeDefRef>),
    Many(std::vec::IntoIter<&'a TypeDefRef>),
}

impl<'a> Iterator for ExtractedTypes<'a> {
    type Item = &'a TypeDefRef;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ExtractedTypes::None => None,
            ExtractedTypes::One(iter) => iter.next(),
            ExtractedTypes::Many(iter) => iter.next(),
        }
    }
}

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

#[cfg(test)]
mod test {
    use super::*;
    use logos::Lexer;
    use parse::FromTokens;
    use pretty_assertions::assert_eq;

    #[test]
    fn interface_() -> Result<()> {
        let source = "interface chars {
            /// A function that accepts a character
            func take_char(x: char)
            /// A function that returns a character
            func return_char() -> char
          }";
        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let iface = Resolver::new(source, &iface).resolve()?;

        println!("{iface:#?}");

        Ok(())
    }

    #[test]
    fn interface__() -> Result<()> {
        let source = "interface test {
            record a {
                foo: nested
            }
            record nested {
                bar: string
            }
            func fn(x: a) -> nested
          }";
        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let iface = Resolver::new(source, &iface).resolve()?;

        println!("{iface:#?}");

        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        let source = include_str!("test.wit");

        let mut tokens = Lexer::new(source).spanned().peekable();

        let iface = parse::Interface::parse(&mut tokens)?;
        let _iface = Resolver::new(source, &iface).resolve()?;

        Ok(())
    }
}

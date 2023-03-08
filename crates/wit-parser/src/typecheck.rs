use crate::{util::IteratorExt, Error, Result};
use id_arena::{Arena, Id};
use logos::Span;
use std::collections::{HashMap, HashSet};

use crate::parse;

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
    pub typedefs: Arena<TypeDef>,
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
    Id(Id<TypeDef>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDef {
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
    pub fn len(&self) -> usize {
        match self {
            FunctionResult::Named(ps) => ps.len(),
            FunctionResult::Anon(_) => 1,
        }
    }
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
    ident2id: HashMap<&'a str, Id<TypeDef>>,
    typedefs: Arena<TypeDef>,
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
            // ident2interface_item: typedefs,
            ident2id: HashMap::with_capacity(typedefs.len()),
            typedefs: Arena::new(),
        }
    }

    fn read_span(&self, span: &Span) -> &'a str {
        &self.source[span.clone()]
    }

    fn resolve_ident(&self, span: &Span) -> &'a str {
        self.read_span(span).trim_start_matches('%')
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
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn resolve_typedef(&mut self, typedef: &parse::InterfaceItem) -> Result<Id<TypeDef>> {
        let ident = self.resolve_ident(&typedef.ident);

        let docs = self.resolve_docs(&typedef.docs);

        let kind = match &typedef.inner {
            parse::InterfaceItemInner::Alias(ty) => {
                let ty = self.resolve_type(ty)?;

                TypeDefKind::Alias(ty)
            }
            parse::InterfaceItemInner::Record(fields) => {
                let res: Result<Vec<_>> = fields
                    .iter()
                    .map(|field| {
                        let docs = self.resolve_docs(&field.docs);
                        let ident = self.resolve_ident(&field.ident).to_string();
                        let ty = self.resolve_type(&field.ty)?;

                        Ok(RecordField { docs, ident, ty })
                    })
                    .partition_result();

                TypeDefKind::Record(res?)
            }
            parse::InterfaceItemInner::Flags(fields) => {
                let fields = fields.iter().map(|field| {
                    let docs = self.resolve_docs(&field.docs);
                    let ident = self.resolve_ident(&field.ident).to_string();

                    FlagsField { docs, ident }
                });

                TypeDefKind::Flags(fields.collect())
            }
            parse::InterfaceItemInner::Variant(cases) => {
                let res: Result<Vec<_>> = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let ident = self.resolve_ident(&case.ident).to_string();
                        let ty = case
                            .ty
                            .as_ref()
                            .map(|ty| self.resolve_type(ty))
                            .transpose()?;

                        Ok(VariantCase { docs, ident, ty })
                    })
                    .partition_result();

                TypeDefKind::Variant(res?)
            }
            parse::InterfaceItemInner::Enum(cases) => {
                let cases = cases.iter().map(|case| {
                    let docs = self.resolve_docs(&case.docs);
                    let ident = self.resolve_ident(&case.ident).to_string();

                    EnumCase { docs, ident }
                });

                TypeDefKind::Enum(cases.collect())
            }
            parse::InterfaceItemInner::Union(cases) => {
                let res: Result<Vec<_>> = cases
                    .iter()
                    .map(|case| {
                        let docs = self.resolve_docs(&case.docs);
                        let ty = self.resolve_type(&case.ty)?;

                        Ok(UnionCase { docs, ty })
                    })
                    .partition_result();

                TypeDefKind::Union(res?)
            }
            parse::InterfaceItemInner::Func(_) => unreachable!(),
        };

        let id = self.typedefs.alloc(TypeDef {
            docs,
            ident: ident.to_string(),
            kind,
        });
        self.ident2id.remove(ident);

        Ok(id)
    }

    fn resolve_type(&mut self, ty: &parse::Type) -> Result<Type> {
        let ty = match ty {
            parse::Type::Bool => Type::Bool,
            parse::Type::U8 => Type::U8,
            parse::Type::U16 => Type::U16,
            parse::Type::U32 => Type::U32,
            parse::Type::U64 => Type::U64,
            parse::Type::S8 => Type::S8,
            parse::Type::S16 => Type::S16,
            parse::Type::S32 => Type::S32,
            parse::Type::S64 => Type::S64,
            parse::Type::Float32 => Type::Float32,
            parse::Type::Float64 => Type::Float64,
            parse::Type::Char => Type::Char,
            parse::Type::String => Type::String,
            parse::Type::List(ty) => Type::List(Box::new(self.resolve_type(ty)?)),
            parse::Type::Option(ty) => Type::Option(Box::new(self.resolve_type(ty)?)),
            parse::Type::Tuple(types) => {
                let res: Result<Vec<Type>> = types
                    .iter()
                    .map(|ty| self.resolve_type(ty))
                    .partition_result();

                Type::Tuple(res?)
            }
            parse::Type::Result { ok, err } => {
                let ok = ok.as_ref().map(|ty| self.resolve_type(ty)).transpose()?;

                let err = err.as_ref().map(|ty| self.resolve_type(ty)).transpose()?;

                Type::Result {
                    ok: ok.map(Box::new),
                    err: err.map(Box::new),
                }
            }
            parse::Type::Id(span) => {
                let ident = self.resolve_ident(span);

                if let Some(id) = self.ident2id.get(ident) {
                    Type::Id(*id)
                } else {
                    let typedef = self
                        .interface
                        .items
                        .iter()
                        .find(|item| {
                            item.ident == *span
                                && !matches!(item.inner, parse::InterfaceItemInner::Func(_))
                        })
                        .unwrap();

                    let id = self.resolve_typedef(&typedef.clone())?;

                    Type::Id(id)
                }
            }
        };

        Ok(ty)
    }

    fn resolve_named_types(
        &mut self,
        named_types: &[(Span, parse::Type)],
    ) -> Result<Vec<(String, Type)>> {
        Ok(named_types
            .iter()
            .map(|(ident, ty)| {
                let ident = self.resolve_ident(&ident).to_string();
                let ty = self.resolve_type(&ty).unwrap();

                (ident, ty)
            })
            .collect())
    }

    fn resolve_func(
        &mut self,
        docs: &[Span],
        ident: &Span,
        func: &parse::Func,
    ) -> Result<Function> {
        let docs = self.resolve_docs(docs);
        let ident = self.resolve_ident(ident).to_string();

        let params = self.resolve_named_types(&func.params)?;

        let results = match &func.results {
            parse::FuncResult::Anon(ty) => {
                let ty = self.resolve_type(ty)?;
                FunctionResult::Anon(ty)
            }
            parse::FuncResult::Named(types) => {
                let types = self.resolve_named_types(types)?;
                FunctionResult::Named(types)
            }
        };

        Ok(Function {
            docs,
            ident,
            params,
            result: results,
        })
    }

    fn verify_not_recursive(
        &self,
        ident: Span,
        id: Id<TypeDef>,
        visiting: &mut HashSet<Id<TypeDef>>,
        valid: &mut HashSet<Id<TypeDef>>,
    ) -> Result<()> {
        if valid.contains(&id) {
            return Ok(());
        }

        if !visiting.insert(id) {
            return Err(Error::recursive_type(ident));
        }

        match &self.typedefs[id].kind {
            TypeDefKind::Record(fields) => {
                for field in fields {
                    if let Type::Id(id) = field.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Union(cases) => {
                for case in cases {
                    if let Type::Id(id) = case.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Variant(cases) => {
                for case in cases {
                    if let Some(Type::Id(id)) = case.ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
            }
            TypeDefKind::Alias(ty) => match ty {
                Type::Tuple(types) => {
                    for ty in types {
                        if let Type::Id(id) = ty {
                            self.verify_not_recursive(ident.clone(), *id, visiting, valid)?;
                        }
                    }
                }
                Type::List(ty) | Type::Option(ty) => {
                    if let Type::Id(id) = **ty {
                        self.verify_not_recursive(ident.clone(), id, visiting, valid)?;
                    }
                }
                Type::Result { ok, err } => {
                    if let Some(Type::Id(id)) = ok.as_deref() {
                        self.verify_not_recursive(ident.clone(), *id, visiting, valid)?;
                    }

                    if let Some(Type::Id(id)) = err.as_deref() {
                        self.verify_not_recursive(ident, *id, visiting, valid)?;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        valid.insert(id);
        visiting.remove(&id);

        Ok(())
    }

    pub fn resolve(mut self) -> Result<Interface> {
        let docs = self.resolve_docs(&self.interface.docs);
        let ident = self.resolve_ident(&self.interface.ident).to_string();

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

        let mut visiting = HashSet::new();
        let mut valid_types = HashSet::new();
        for (id, typedef) in &self.typedefs {
            let ident_span = self
                .interface
                .items
                .iter()
                .find_map(|item| {
                    if self.resolve_ident(&item.ident) == typedef.ident
                        && !matches!(item.inner, parse::InterfaceItemInner::Func(_))
                    {
                        Some(item.ident.clone())
                    } else {
                        None
                    }
                })
                .unwrap();

            self.verify_not_recursive(ident_span, id, &mut visiting, &mut valid_types)?;
        }

        Ok(Interface {
            docs,
            ident,
            functions,
            typedefs: self.typedefs,
        })
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

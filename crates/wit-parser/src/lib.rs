#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

mod error;
mod lex;
mod parse;
pub mod typecheck;
mod util;

pub use error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;

pub type TypeDefId = Id<TypeDef>;
pub type TypeDefArena = Arena<TypeDef>;

use id_arena::{Arena, Id};
use logos::Logos;
use miette::{ErrReport, IntoDiagnostic, NamedSource};
use parse::FromTokens;
use std::path::Path;
use typecheck::Resolver;

pub fn parse_str(input: impl AsRef<str>, skip: impl Fn(&str) -> bool) -> miette::Result<Interface> {
    let iface = parse(input.as_ref(), skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new("virtual file", input.as_ref().to_string()))
    })?;

    Ok(iface)
}

pub fn parse_file(
    path: impl AsRef<Path>,
    skip: impl Fn(&str) -> bool,
) -> miette::Result<Interface> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).into_diagnostic()?;

    let iface = parse(&input, skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new(path.to_string_lossy(), input))
    })?;

    Ok(iface)
}

fn parse(input: &str, _skip: impl Fn(&str) -> bool) -> miette::Result<Interface> {
    let mut tokens = lex::Token::lexer(input).spanned().peekable();

    let iface = parse::Interface::parse(&mut tokens)?;

    let (resolver, rest_data) = Resolver::new(input, iface);
    resolver.resolve(rest_data).map_err(Into::into)
}

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
    Resource(Vec<Function>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub docs: String,
    pub ident: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsField {
    pub docs: String,
    pub ident: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub docs: String,
    pub ident: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub docs: String,
    pub ident: String,
}

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
    pub result: Option<FunctionResult>,
}

impl Function {
    #[must_use]
    pub fn throws(&self) -> bool {
        if let Some(result) = &self.result {
            result.types().any(|ty| matches!(ty, Type::Result { .. }))
        } else {
            false
        }
    }
}

pub type NamedTypeList = Vec<(String, Type)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionResult {
    Anon(Type),
    Named(NamedTypeList),
}

impl FunctionResult {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            FunctionResult::Named(ps) => ps.len(),
            FunctionResult::Anon(_) => 1,
        }
    }
    #[must_use]
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

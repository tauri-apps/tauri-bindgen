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
use serde::{ser::SerializeSeq, Serialize};
use std::path::Path;
use typecheck::Resolver;
use util::detect_invalid_input;

#[inline]
pub fn parse_and_resolve_str(
    input: impl AsRef<str>,
    skip: impl Fn(&str) -> bool,
) -> miette::Result<Interface> {
    let iface = parse_and_resolve(input.as_ref(), skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new("virtual file", input.as_ref().to_string()))
    })?;

    Ok(iface)
}

#[inline]
pub fn parse_and_resolve_file(
    path: impl AsRef<Path>,
    skip: impl Fn(&str) -> bool,
) -> miette::Result<Interface> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).into_diagnostic()?;

    let iface = parse_and_resolve(&input, skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new(path.to_string_lossy(), input))
    })?;

    Ok(iface)
}

#[inline]
fn parse_and_resolve(input: &str, skip: impl Fn(&str) -> bool) -> miette::Result<Interface> {
    let iface = parse(input, skip)?;

    let (resolver, rest_data) = Resolver::new(input, iface);
    resolver.resolve(rest_data).map_err(Into::into)
}

#[inline]
pub fn parse(input: &str, _skip: impl Fn(&str) -> bool) -> miette::Result<parse::Interface> {
    detect_invalid_input(input)?;

    let mut tokens = lex::Token::lexer(input).spanned().peekable();

    let iface = parse::Interface::parse(&mut tokens)?;

    Ok(iface)
}

pub enum Int {
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Interface {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
    #[serde(serialize_with = "serialize_typedefs")]
    pub typedefs: Arena<TypeDef>,
    pub functions: Vec<Function>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "value")]
pub enum Type {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    S8,
    S16,
    S32,
    S64,
    S128,
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
    #[serde(serialize_with = "serialize_id")]
    Id(Id<TypeDef>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TypeDef {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
    #[serde(flatten)]
    pub kind: TypeDefKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "value")]
pub enum TypeDefKind {
    Alias(Type),
    Record(Vec<RecordField>),
    Flags(Vec<FlagsField>),
    Variant(Vec<VariantCase>),
    Enum(Vec<EnumCase>),
    Union(Vec<UnionCase>),
    Resource(Vec<Function>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RecordField {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FlagsField {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct VariantCase {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct EnumCase {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UnionCase {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Function {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub docs: String,
    pub id: String,
    #[serde(serialize_with = "serialize_named_type_list")]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase", tag = "type", content = "value")]
pub enum FunctionResult {
    Anon(Type),
    #[serde(serialize_with = "serialize_named_type_list")]
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

fn serialize_typedefs<S>(typedefs: &Arena<TypeDef>, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut s = s.serialize_seq(Some(typedefs.len()))?;

    for (_, typedef) in typedefs {
        s.serialize_element(typedef)?;
    }

    s.end()
}

fn serialize_id<S>(id: &Id<TypeDef>, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u32(u32::try_from(id.index()).unwrap())
}

fn serialize_named_type_list<S>(
    named_types: &Vec<(String, Type)>,
    s: S,
) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    #[derive(Serialize)]
    struct NamedType<'a> {
        ident: &'a str,
        r#type: &'a Type,
    }

    let mut s = s.serialize_seq(Some(named_types.len()))?;

    for (ident, r#type) in named_types {
        s.serialize_element(&NamedType { ident, r#type })?;
    }

    s.end()
}

mod ast;
mod error;
pub(crate) mod util;

use ast::parse::FromTokens;
pub use error::Error;
use id_arena::{Arena, Id};
use miette::{ErrReport, IntoDiagnostic, NamedSource};
use std::path::Path;

pub fn parse_file<'a>(path: impl AsRef<Path>) -> miette::Result<Interface> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).into_diagnostic()?;

    let iface = parse(&input).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new(path.to_string_lossy(), input))
    })?;

    Ok(iface)
}

fn parse(input: &str) -> miette::Result<Interface> {
    let mut tokens = ast::lex::Tokens::from_str(&input);

    let iface = ast::Interface::parse(&mut tokens)?;

    let iface = ast::resolve::Resolver::new(&input).resolve(iface)?;

    Ok(iface)
}

pub type TypeId = Id<TypeDef>;

pub enum Int {
    U8,
    U16,
    U32,
    U64,
}

pub enum FlagsRepr {
    U8,
    U16,
    U32(usize),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Interface {
    pub docs: Docs,
    pub name: String,
    pub functions: Vec<Function>,
    pub types: Arena<TypeDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub docs: Docs,
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub results: Results,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Results {
    Anon(Type),
    Named(Vec<(String, Type)>),
}

impl Results {
    pub fn len(&self) -> usize {
        self.types().count()
    }

    pub fn types(&self) -> ResultsTypeIter {
        match self {
            Results::Named(ps) => ResultsTypeIter::Named(ps.iter()),
            Results::Anon(ty) => ResultsTypeIter::Anon(std::iter::once(ty)),
        }
    }

    pub fn throws(&self) -> Option<(Option<&Type>, Option<&Type>)> {
        if self.len() != 1 {
            return None;
        }
        match self.types().next().unwrap() {
            Type::Result(r) => Some((r.ok.as_ref(), r.err.as_ref())),
            // Type::Id(id) => match &iface.types[*id].kind {
            //     _ => None,
            // },
            _ => None,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
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
    Bool,
    Tuple(Tuple),
    List(Box<Type>),
    Option(Box<Type>),
    Result(Box<Result_>),
    Id(Id<TypeDef>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Result_ {
    pub ok: Option<Type>,
    pub err: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeDef {
    // pub pos: Range<usize>,
    pub docs: Docs,
    pub name: String,
    pub kind: TypeDefKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefKind {
    Record(Record),
    Variant(Variant),
    Flags(Flags),
    Union(Union),
    Enum(Enum),
    Type(Type),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordField {
    pub docs: Docs,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub cases: Vec<VariantCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase {
    pub docs: Docs,
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags {
    pub flags: Vec<Flag>,
}

impl Flags {
    pub fn repr(&self) -> Int {
        match self.flags.len() {
            n if n <= 8 => Int::U8,
            n if n <= 16 => Int::U16,
            n if n <= 32 => Int::U32,
            n if n <= 64 => Int::U64,
            _ => panic!("too many flags to fit in a repr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flag {
    pub docs: Docs,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Union {
    pub cases: Vec<UnionCase>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionCase {
    pub docs: Docs,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum {
    pub cases: Vec<EnumCase>,
}

impl Enum {
    pub fn tag(&self) -> Int {
        match self.cases.len() {
            n if n <= u8::MAX as usize => Int::U8,
            n if n <= u16::MAX as usize => Int::U16,
            n if n <= u32::MAX as usize => Int::U32,
            n if n <= u64::MAX as usize => Int::U64,
            _ => panic!("too many cases to fit in a repr"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumCase {
    pub docs: Docs,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Docs {
    pub contents: String,
}

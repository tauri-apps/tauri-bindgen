mod ast;

use std::{collections::{hash_map, HashMap}, path::Path, io};
use id_arena::{Arena, Id};

pub type TypeId = Id<TypeDef>;

pub fn parse_file<'a>(path: impl AsRef<Path>) -> Result<Interface, Error> {
    let input = std::fs::read_to_string(path)?;

    let mut tokens = ast::lex::Tokenizer::from_str(&input);
    let iface = ast::parse::interface(&mut tokens)?;

    let iface = ast::resolve::Resolver::default().resolve(iface)?;

    Ok(iface)
}

pub enum Int {
    U8,
    U16,
    U32,
    U64
}

pub enum FlagsRepr {
    U8,
    U16,
    U32(usize)
}


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Parse(#[from] ast::parse::Error),
    #[error(transparent)]
    Resolve(#[from] ast::resolve::Errors),
    #[error(transparent)]
    Io(#[from] io::Error)
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
    pub params: HashMap<String, Type>,
    pub results: Results,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Results {
    Anon(Type),
    Named(HashMap<String, Type>),
}

impl Results {
    pub fn len(&self) -> usize {
        self.types().count()
    }

    pub fn types(&self) -> ResultsTypeIter {
        match self {
            Results::Named(ps) => ResultsTypeIter::Named(ps.values()),
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
    Named(hash_map::Values<'a, String, Type>),
}

impl<'a> Iterator for ResultsTypeIter<'a> {
    type Item = &'a Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ResultsTypeIter::Anon(iter) => iter.next(),
            ResultsTypeIter::Named(iter) => iter.next(),
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
            n if n <= u8::MAX as usize => Int::U8,
            n if n <= u16::MAX as usize =>Int::U16,
            n if n <= u32::MAX as usize => Int::U32,
            n if n <= u64::MAX as usize => Int::U64,
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

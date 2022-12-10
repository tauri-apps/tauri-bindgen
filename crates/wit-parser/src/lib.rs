mod ast;
mod error;
pub(crate) mod util;

use ast::parse::FromTokens;
pub use error::Error;
use miette::{ErrReport, IntoDiagnostic, NamedSource};
use std::{path::Path, sync::Arc, collections::HashMap};

pub fn parse_file(path: impl AsRef<Path>) -> miette::Result<Interface> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).into_diagnostic()?;

    let iface = parse(&input).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new(path.to_string_lossy(), input))
    })?;

    Ok(iface)
}

pub fn parse(input: &str) -> miette::Result<Interface> {
    let mut tokens = ast::lex::Tokens::from_str(input);

    let iface = ast::Interface::parse(&mut tokens)?;

    let iface = ast::resolve::Resolver::new(input).resolve(iface)?;

    Ok(iface)
}

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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Interface {
    pub docs: Docs,
    pub name: String,
    pub functions: Vec<Function>,
    pub types: HashMap<String, Arc<TypeDef>>,
}

type NamedTypeList = Vec<(String, Type)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub docs: Docs,
    pub name: String,
    pub params: NamedTypeList,
    pub results: Results,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Results {
    Anon(Type),
    Named(NamedTypeList),
}

impl Results {
    pub fn len(&self) -> usize {
        self.types().count()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn types(&self) -> ResultsTypeIter {
        match self {
            Results::Named(ps) => ResultsTypeIter::Named(ps.iter()),
            Results::Anon(ty) => ResultsTypeIter::Anon(std::iter::once(ty)),
        }
    }

    pub fn types_mut(&mut self) -> ResultsTypeIterMut {
        match self {
            Results::Named(ps) => ResultsTypeIterMut::Named(ps.iter_mut()),
            Results::Anon(ty) => ResultsTypeIterMut::Anon(std::iter::once(ty)),
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

pub enum ResultsTypeIterMut<'a> {
    Anon(std::iter::Once<&'a mut Type>),
    Named(std::slice::IterMut<'a, (String, Type)>),
}

impl<'a> Iterator for ResultsTypeIterMut<'a> {
    type Item = &'a mut Type;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ResultsTypeIterMut::Anon(iter) => iter.next(),
            ResultsTypeIterMut::Named(iter) => iter.next().map(|pair| &mut pair.1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Id(Arc<TypeDef>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple {
    pub types: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Result_ {
    pub ok: Option<Type>,
    pub err: Option<Type>,
}

bitflags::bitflags! {
    pub struct TypeInfo: u32 {
        /// Whether or not this type is ever used (transitively) within the
        /// parameter of a function.
        const PARAM = 0b00000001;
        /// Whether or not this type is ever used (transitively) within the
        /// result of a function.
        const RESULT = 0b00000010;
        /// Whether or not this type is ever used (transitively) within the
        /// error case in the result of a function.
        const ERROR = 0b00000100;
        /// Whether or not this type (transitively) has a list.
        const HAS_LIST = 0b00001000;
        /// Wether this type contains static functions (only for resources)
        const HAS_STATICS = 0b00010000;

        const PARAM_AND_RESULT = Self::PARAM.bits | Self::RESULT.bits;
    }
}

impl TypeInfo {
    pub fn owns_data(&self) -> bool {
        self.contains(TypeInfo::HAS_LIST)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDef {
    Record(Record),
    Variant(Variant),
    Flags(Flags),
    Union(Union),
    Enum(Enum),
    Type(Type),
    Resource(Resource),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
    pub fields: Vec<RecordField>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub docs: Docs,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variant {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
    pub cases: Vec<VariantCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub docs: Docs,
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flags {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub docs: Docs,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Union {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
    pub cases: Vec<UnionCase>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionCase {
    pub docs: Docs,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enum {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub docs: Docs,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Docs {
    pub contents: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Resource {
    pub info: TypeInfo,
    pub docs: Docs,
    pub name: String,
    pub methods: Vec<Method>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method {
    pub docs: Docs,
    pub name: String,
    pub static_: bool,
    pub params: NamedTypeList,
    pub results: Results,
}

impl From<Method> for Function {
    fn from(m: Method) -> Self {
        Self {
            docs: m.docs,
            name: m.name,
            params: m.params,
            results: m.results,
        }
    }
}

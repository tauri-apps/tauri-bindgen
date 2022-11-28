pub(crate) mod lex;
pub(crate) mod parse;
pub(crate) mod resolve;

use lex::Span;

#[derive(Debug, Clone)]
pub struct Interface<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    items: Vec<InterfaceItem<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceItem<'a> {
    Record(Record<'a>),
    Variant(Variant<'a>),
    Flags(Flags<'a>),
    Union(Union<'a>),
    Enum(Enum<'a>),
    Alias(Alias<'a>),
    Func(Func<'a>)
}

type NamedTypeList<'a> = Vec<(Span<'a>, Type<'a>)>;

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    params: NamedTypeList<'a>,
    results: Results<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Results<'a> {
    Anon(Type<'a>),
    Named(NamedTypeList<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'a> {
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
    Tuple(Tuple<'a>),
    List(Box<Type<'a>>),
    Option(Box<Type<'a>>),
    Result(Box<Result_<'a>>),
    Id(Span<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple<'a> {
    types: Vec<Type<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Result_<'a> {
    ok: Option<Type<'a>>,
    err: Option<Type<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    fields: Vec<RecordField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordField<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    cases: Vec<VariantCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    ty: Option<Type<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    flags: Vec<Flag<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flag<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Union<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    cases: Vec<UnionCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionCase<'a> {
    docs: Docs<'a>,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    cases: Vec<EnumCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumCase<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Alias<'a> {
    docs: Docs<'a>,
    name: Span<'a>,
    ty: Type<'a>
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Docs<'a> {
    docs: Vec<Span<'a>>
}
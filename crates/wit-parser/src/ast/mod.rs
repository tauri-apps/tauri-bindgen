use miette::SourceSpan;

pub mod lex;
pub mod parse;
pub mod resolve;

#[derive(Debug, Clone, PartialEq)]
pub struct Interface<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
    items: Vec<InterfaceItem<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceItem<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
    kind: InterfaceItemKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceItemKind<'a> {
    Record(Record<'a>),
    Flags(Flags<'a>),
    Variant(Variant<'a>),
    Union(Union<'a>),
    Enum(Enum<'a>),
    Alias(Type),
    Func(Func),
    Use(Use),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record<'a> {
    fields: Vec<RecordField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordField<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags<'a> {
    fields: Vec<FlagsField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlagsField<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant<'a> {
    cases: Vec<VariantCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
    ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Union<'a> {
    cases: Vec<UnionCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionCase<'a> {
    docs: Docs<'a>,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    cases: Vec<EnumCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumCase<'a> {
    docs: Docs<'a>,
    name: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func {
    params: NamedTypeList,
    results: Results,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTypeList {
    inner: Vec<NamedType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedType {
    name: SourceSpan,
    ty: Type,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Results {
    Anon(Type),
    Named(NamedTypeList),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Use {
    use_names: UseNames,
    from: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UseNames {
    All,
    Subset(Vec<UseName>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseName {
    name: SourceSpan,
    alias: Option<SourceSpan>,
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
    Tuple(Vec<Type>),
    List(Box<Type>),
    Option(Box<Type>),
    Result {
        ok: Option<Box<Type>>,
        err: Option<Box<Type>>,
    },
    Id(SourceSpan),
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Docs<'a> {
    docs: Vec<&'a str>,
}

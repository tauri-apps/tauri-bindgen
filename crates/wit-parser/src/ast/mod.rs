use miette::SourceSpan;

pub mod lex;
pub mod parse;
pub mod resolve;

#[derive(Debug, Clone, PartialEq)]
pub struct Interface<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    name: SourceSpan,
    items: Vec<InterfaceItem<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceItem<'a> {
    pos: SourceSpan,
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
    Alias(Type<'a>),
    Func(Func<'a>),
    Use(Use),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Record<'a> {
    fields: Vec<RecordField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordField<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    name: SourceSpan,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags<'a> {
    fields: Vec<FlagsField<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlagsField<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    name: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variant<'a> {
    cases: Vec<VariantCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantCase<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    name: SourceSpan,
    ty: Option<Type<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Union<'a> {
    cases: Vec<UnionCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionCase<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enum<'a> {
    cases: Vec<EnumCase<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumCase<'a> {
    pos: SourceSpan,
    docs: Docs<'a>,
    name: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'a> {
    params: NamedTypeList<'a>,
    results: Results<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTypeList<'a> {
    pos: SourceSpan,
    inner: Vec<NamedType<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedType<'a> {
    pos: SourceSpan,
    name: SourceSpan,
    ty: Type<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Results<'a> {
    Anon(Type<'a>),
    Named(NamedTypeList<'a>),
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
    pos: SourceSpan,
    name: SourceSpan,
    alias: Option<SourceSpan>,
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
    Tuple(Vec<Type<'a>>),
    List(Box<Type<'a>>),
    Option(Box<Type<'a>>),
    Result {
        ok: Option<Box<Type<'a>>>,
        err: Option<Box<Type<'a>>>,
    },
    Id {
        pos: SourceSpan,
        name: &'a str 
    },
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Docs<'a> {
    docs: Vec<&'a str>,
}
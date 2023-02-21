use crate::{
    lex::Token,
    util::{find_similar, print_list},
    Error, Result,
};
use logos::{Span, SpannedIter};
use std::iter::Peekable;

pub type Tokens<'a> = Peekable<SpannedIter<'a, Token>>;

trait TokensExt {
    fn next_if_token(&mut self, token: Token) -> Option<(Token, Span)>;
    fn expect_token(&mut self, token: Token) -> Result<(Token, Span)>;
}

impl<'a> TokensExt for Tokens<'a> {
    fn next_if_token(&mut self, expected: Token) -> Option<(Token, Span)> {
        self.next_if(|(found, _)| *found == expected)
    }

    fn expect_token(&mut self, expected: Token) -> Result<(Token, Span)> {
        match self.next() {
            Some((found, span)) if found == expected => Ok((found, span)),
            Some((found, span)) => Err(Error::unexpected_token(span, [expected], found)),
            None => Err(Error::UnexpectedEof),
        }
    }
}

pub trait FromTokens<'a>
where
    Self: Sized,
{
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interface {
    ident: Span,
    docs: Vec<Span>,
    items: Vec<InterfaceItem>,
}

impl<'a> FromTokens<'a> for Interface {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        tokens.expect_token(Token::Interface)?;

        let (_, ident) = tokens.expect_token(Token::Id)?;

        let items = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

        Ok(Interface { ident, docs, items })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceItem {
    docs: Vec<Span>,
    ident: Span,
    inner: InterfaceItemInner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterfaceItemInner {
    Alias(Type),
    Record(Vec<RecordField>),
    Flags(Vec<FlagsField>),
    Variant(Vec<VariantCase>),
    Enum(Vec<EnumCase>),
    Union(Vec<UnionCase>),
    Func(Func),
}

impl<'a> FromTokens<'a> for InterfaceItem {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (kind, kind_span) = tokens.next().ok_or(Error::UnexpectedEof)?;

        let (_, ident) = tokens.expect_token(Token::Id)?;

        let inner = match kind {
            Token::Record => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemInner::Record(inner)
            }
            Token::Enum => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemInner::Enum(inner)
            }
            Token::Flags => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemInner::Flags(inner)
            }
            Token::Variant => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemInner::Variant(inner)
            }
            Token::Union => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemInner::Union(inner)
            }
            Token::Type => {
                tokens.expect_token(Token::Equals)?;

                InterfaceItemInner::Alias(Type::parse(tokens)?)
            }
            Token::Func => todo!(),
            Token::Resource => todo!(),
            Token::Use => todo!(),
            found => {
                let expected = [
                    Token::Enum,
                    Token::Flags,
                    Token::Func,
                    Token::Record,
                    Token::Type,
                    Token::Union,
                    Token::Use,
                    Token::Variant,
                ];

                let suggestions =
                    find_similar(expected.iter().map(ToString::to_string), found.to_string());

                if suggestions.is_empty() {
                    return Err(Error::unexpected_token(kind_span, expected, found));
                } else {
                    return Err(Error::unexpected_token_with_help(
                        kind_span,
                        expected,
                        found,
                        format!("Did you mean \"{}\"?", print_list(suggestions)),
                    ));
                }
            }
        };

        Ok(InterfaceItem { docs, ident, inner })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    params: NamedTypeList,
    results: FuncResult,
}

impl<'a> FromTokens<'a> for Func {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let params = NamedTypeList::parse(tokens)?;

        let results = if tokens.next_if_token(Token::RArrow).is_some() {
            FuncResult::parse(tokens)?
        } else {
            FuncResult::Anon(Type::Tuple(vec![]))
        };

        Ok(Func { params, results })
    }
}

pub type NamedTypeList = Vec<(Span, Type)>;

impl<'a> FromTokens<'a> for NamedTypeList {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        parse_list(tokens, Token::LeftParen, Token::RightParen)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncResult {
    Anon(Type),
    Named(NamedTypeList),
}

impl<'a> FromTokens<'a> for FuncResult {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    ident: Span,
    docs: Vec<Span>,
    ty: Type,
}

impl<'a> FromTokens<'a> for RecordField {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect_token(Token::Id)?;

        tokens.expect_token(Token::Colon)?;

        let ty = Type::parse(tokens)?;

        Ok(RecordField { docs, ident, ty })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsField {
    ident: Span,
    docs: Vec<Span>,
}

impl<'a> FromTokens<'a> for FlagsField {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect_token(Token::Id)?;

        Ok(FlagsField { docs, ident })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    ident: Span,
    docs: Vec<Span>,
    ty: Option<Type>,
}

impl<'a> FromTokens<'a> for VariantCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect_token(Token::Id)?;

        let ty = if tokens.next_if_token(Token::LeftParen).is_some() {
            let ty = Type::parse(tokens)?;
            tokens.expect_token(Token::RightParen)?;

            Some(ty)
        } else {
            None
        };

        Ok(VariantCase { docs, ident, ty })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    ident: Span,
    docs: Vec<Span>,
}

impl<'a> FromTokens<'a> for EnumCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect_token(Token::Id)?;

        Ok(EnumCase { docs, ident })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionCase {
    docs: Vec<Span>,
    ty: Type,
}

impl<'a> FromTokens<'a> for UnionCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let ty = Type::parse(tokens)?;

        Ok(UnionCase { docs, ty })
    }
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
    Id(Span),
}

impl<'a> FromTokens<'a> for Type {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let (token, span) = tokens.next().ok_or(Error::UnexpectedEof)?;

        match token {
            Token::Bool => Ok(Self::Bool),
            Token::U8 => Ok(Self::U8),
            Token::U16 => Ok(Self::U16),
            Token::U32 => Ok(Self::U32),
            Token::U64 => Ok(Self::U64),
            Token::S8 => Ok(Self::S8),
            Token::S16 => Ok(Self::S16),
            Token::S32 => Ok(Self::S32),
            Token::S64 => Ok(Self::S64),
            Token::Float32 => Ok(Self::Float32),
            Token::Float64 => Ok(Self::Float64),
            Token::Char => Ok(Self::Char),
            Token::String => Ok(Self::String),
            Token::List => {
                tokens.expect_token(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect_token(Token::GreaterThan)?;

                Ok(Self::List(Box::new(ty)))
            }
            Token::Tuple => {
                let types = parse_list(tokens, Token::LeftParen, Token::RightParen)?;

                Ok(Self::Tuple(types))
            }
            Token::Option => {
                tokens.expect_token(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect_token(Token::GreaterThan)?;

                Ok(Self::Option(Box::new(ty)))
            }
            Token::Result => {
                let mut ok = None;
                let mut err = None;

                if tokens.next_if_token(Token::LessThan).is_some() {
                    if tokens.next_if_token(Token::Underscore).is_some() {
                        tokens.expect_token(Token::Comma)?;
                        err = Some(Box::new(Type::parse(tokens)?));
                    } else {
                        ok = Some(Box::new(Type::parse(tokens)?));

                        if tokens.next_if_token(Token::Comma).is_some() {
                            err = Some(Box::new(Type::parse(tokens)?));
                        }
                    };
                    tokens.expect_token(Token::GreaterThan)?;
                };

                Ok(Self::Result { ok, err })
            }
            Token::Id => Ok(Self::Id(span)),
            found => {
                let expected = [
                    Token::U8,
                    Token::U16,
                    Token::U32,
                    Token::U64,
                    Token::S8,
                    Token::S16,
                    Token::S32,
                    Token::S64,
                    Token::Float32,
                    Token::Float64,
                    Token::Char,
                    Token::String,
                    Token::Bool,
                    Token::Option,
                    Token::Result,
                    Token::List,
                    Token::Tuple,
                    Token::Id,
                ];

                Err(Error::unexpected_token(span, expected, found))
            }
        }
    }
}

fn parse_list<'a, O>(tokens: &mut Tokens<'a>, start: Token, end: Token) -> Result<Vec<O>>
where
    O: FromTokens<'a>,
{
    tokens.expect_token(start)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.next_if_token(end).is_some() {
            break;
        }

        let item = FromTokens::parse(tokens)?;
        items.push(item);

        // if there's no trailing comma then this is required to be the end,
        // otherwise we go through the loop to try to get another item
        if tokens.next_if_token(Token::Comma).is_none() {
            tokens.expect_token(end)?;
            break;
        }
    }
    Ok(items)
}

fn parse_docs<'a>(tokens: &mut Tokens<'a>) -> Vec<Span> {
    let mut spans = Vec::new();

    while let Some((Token::DocComment | Token::BlockDocComment, span)) = tokens.peek() {
        spans.push(span.clone());

        tokens.next();
    }

    spans
}

#[cfg(test)]
mod test {
    use super::*;
    use logos::Lexer;
    use pretty_assertions::assert_eq;

    #[test]
    fn feature() -> Result<()> {
        let mut tokens = Lexer::new("record foo {}").spanned().peekable();

        let ty = InterfaceItem::parse(&mut tokens)?;

        println!("{ty:#?}");

        Ok(())
    }

    #[test]
    fn option() -> Result<()> {
        let mut tokens = Lexer::new("option<u8>").spanned().peekable();

        let ty = Type::parse(&mut tokens)?;

        assert_eq!(ty, Type::Option(Box::new(Type::U8)));

        Ok(())
    }

    #[test]
    fn result() -> Result<()> {
        let mut tokens = Lexer::new("result<u8, string>").spanned().peekable();

        let ty = Type::parse(&mut tokens)?;

        assert_eq!(
            ty,
            Type::Result {
                ok: Some(Box::new(Type::U8)),
                err: Some(Box::new(Type::String))
            }
        );

        Ok(())
    }

    #[test]
    fn result2() -> Result<()> {
        let mut tokens = Lexer::new("result<_, string>").spanned().peekable();

        let ty = Type::parse(&mut tokens)?;

        assert_eq!(
            ty,
            Type::Result {
                ok: None,
                err: Some(Box::new(Type::String))
            }
        );

        Ok(())
    }

    #[test]
    fn typedef_() -> Result<()> {
        let mut tokens = Lexer::new("type foo = result<u8, string>")
            .spanned()
            .peekable();

        let ty = InterfaceItem::parse(&mut tokens)?;

        assert_eq!(
            ty.inner,
            InterfaceItemInner::Alias(Type::Result {
                ok: Some(Box::new(Type::U8)),
                err: Some(Box::new(Type::String))
            })
        );

        Ok(())
    }

    #[test]
    fn enum__() -> Result<()> {
        let mut tokens = Lexer::new(
            "enum color {
            red,
            green,
            blue,
            yellow,
            other,
        }
        ",
        )
        .spanned()
        .peekable();

        let _ty = InterfaceItem::parse(&mut tokens)?;

        // assert_eq!(ty.name, "color");
        // assert_eq!(
        //     ty.kind
        //         .into_iter()
        //         .map(|c| c.name.as_str())
        //         .collect::<Vec<_>>(),
        //     vec!["red", "green", "blue", "yellow", "other"]
        // );

        Ok(())
    }

    #[test]
    fn flags_() -> Result<()> {
        let mut tokens = Lexer::new(
            "flags properties {
            lego,
            marvel_superhero,
            supervillain,
        }",
        )
        .spanned()
        .peekable();

        let _ty = InterfaceItem::parse(&mut tokens)?;

        // assert_eq!(ty.name, "properties");
        // assert_eq!(
        //     ty.flags
        //         .into_iter()
        //         .map(|c| c.name.as_str())
        //         .collect::<Vec<_>>(),
        //     vec!["lego", "marvel-superhero", "supervillain"]
        // );
        Ok(())
    }

    #[test]
    fn union_() -> Result<()> {
        let mut tokens = Lexer::new(
            "union configuration {
            string,
            list<string>,
        }
        ",
        )
        .spanned()
        .peekable();

        let _ty = InterfaceItem::parse(&mut tokens)?;

        // assert_eq!(ty.name, "configuration");
        // assert_eq!(
        //     ty.cases.into_iter().map(|c| c.ty).collect::<Vec<_>>(),
        //     vec![Type::String, Type::List(Box::new(Type::String))]
        // );
        Ok(())
    }

    #[test]
    fn variant_() -> Result<()> {
        let mut tokens = Lexer::new(
            "variant filter {
            all,
            none,
            some(list<string>),
        }
        ",
        )
        .spanned()
        .peekable();

        let _ty = InterfaceItem::parse(&mut tokens)?;

        // assert_eq!(ty.name, "filter");
        // assert_eq!(ty.cases[0].name.as_str(), "all");
        // assert_eq!(ty.cases[0].ty, None);

        // assert_eq!(ty.cases[1].name.as_str(), "none");
        // assert_eq!(ty.cases[1].ty, None);

        // assert_eq!(ty.cases[2].name.as_str(), "some");
        // assert_eq!(ty.cases[2].ty, Some(Type::List(Box::new(Type::String))));

        Ok(())
    }

    #[test]
    fn interface_() -> Result<()> {
        let mut tokens = Lexer::new(
            "interface chars {
            /// A function that accepts a character
            func take-char(x: char)
            /// A function that returns a character
            func return-char() -> char
          }",
        )
        .spanned()
        .peekable();

        let iface = Interface::parse(&mut tokens)?;

        println!("{iface:#?}");

        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        let input = include_str!("test.wit");

        let mut tokens = Lexer::new(input).spanned().peekable();

        let iface = Interface::parse(&mut tokens)?;

        println!("{iface:#?}");

        Ok(())
    }
}

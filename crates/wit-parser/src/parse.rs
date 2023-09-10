#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]

use crate::{
    lex::Token,
    util::{find_similar, print_list},
    Error, Result,
};
use logos::{Span, SpannedIter};
use std::iter::Peekable;

pub type Tokens<'a> = Peekable<SpannedIter<'a, Token>>;

trait TokensExt {
    fn next_if_token(&mut self, token: Token) -> Result<Option<(Token, Span)>>;
    fn expect(&mut self, token: Token) -> Result<(Token, Span)>;
}

impl<'a> TokensExt for Tokens<'a> {
    fn next_if_token(&mut self, expected: Token) -> Result<Option<(Token, Span)>> {
        self.next_if(|(found, _)| *found == Ok(expected))
            .map(|(found, range)| found.map(|token| (token, range)))
            .transpose()
            .map_err(Error::Lex)
    }

    fn expect(&mut self, expected: Token) -> Result<(Token, Span)> {
        match self.next() {
            Some((Ok(found), span)) if found == expected => Ok((found, span)),
            Some((Ok(found), span)) => Err(Error::unexpected_token(span, [expected], found)),
            Some((Err(err), _)) => Err(Error::Lex(err)),
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
    pub ident: Span,
    pub docs: Vec<Span>,
    pub items: Vec<InterfaceItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceItem {
    pub docs: Vec<Span>,
    pub ident: Span,
    pub inner: InterfaceItemInner,
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
    Resource(Vec<Method>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub params: NamedTypeList,
    pub result: Option<FuncResult>,
    pub streaming: bool,
}

pub type NamedTypeList = Vec<(Span, Type)>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncResult {
    Anon(Type),
    Named(NamedTypeList),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordField {
    pub ident: Span,
    pub docs: Vec<Span>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlagsField {
    pub ident: Span,
    pub docs: Vec<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariantCase {
    pub ident: Span,
    pub docs: Vec<Span>,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumCase {
    pub ident: Span,
    pub docs: Vec<Span>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionCase {
    pub docs: Vec<Span>,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method {
    pub ident: Span,
    pub docs: Vec<Span>,
    pub inner: Func,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Id(Span),
}

impl<'a> FromTokens<'a> for Interface {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        tokens.expect(Token::Interface)?;

        let (_, ident) = tokens.expect(Token::Ident)?;

        log::trace!("parsing interface items...");

        let items = parse_list(tokens, Token::LeftBrace, Token::RightBrace, None)?;

        log::debug!("successfully parsed interface");

        Ok(Interface { ident, docs, items })
    }
}

impl<'a> FromTokens<'a> for InterfaceItem {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (kind, kind_span) = tokens.next().ok_or(Error::UnexpectedEof)?;

        let (_, ident) = tokens.expect(Token::Ident)?;

        let inner = match kind? {
            Token::Record => {
                let inner = parse_list(
                    tokens,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Some(Token::Comma),
                )?;

                InterfaceItemInner::Record(inner)
            }
            Token::Enum => {
                let inner = parse_list(
                    tokens,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Some(Token::Comma),
                )?;

                InterfaceItemInner::Enum(inner)
            }
            Token::Flags => {
                let inner = parse_list(
                    tokens,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Some(Token::Comma),
                )?;

                InterfaceItemInner::Flags(inner)
            }
            Token::Variant => {
                let inner = parse_list(
                    tokens,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Some(Token::Comma),
                )?;

                InterfaceItemInner::Variant(inner)
            }
            Token::Union => {
                let inner = parse_list(
                    tokens,
                    Token::LeftBrace,
                    Token::RightBrace,
                    Some(Token::Comma),
                )?;

                InterfaceItemInner::Union(inner)
            }
            Token::Type => {
                tokens.expect(Token::Equals)?;

                InterfaceItemInner::Alias(Type::parse(tokens)?)
            }
            Token::Func => {
                let inner = Func::parse(tokens)?;

                InterfaceItemInner::Func(inner)
            }
            Token::Resource => {
                let inner = parse_list(tokens, Token::LeftBrace, Token::RightBrace, None)?;

                InterfaceItemInner::Resource(inner)
            }
            Token::Use => todo!(),
            found => {
                let suggestions = find_similar(
                    Token::IFACE_ITEM_KEYWORD.iter().map(ToString::to_string),
                    found.to_string(),
                );

                if suggestions.is_empty() {
                    return Err(Error::unexpected_token(
                        kind_span,
                        Token::IFACE_ITEM_KEYWORD,
                        found,
                    ));
                }
                return Err(Error::unexpected_token_with_help(
                    kind_span,
                    Token::IFACE_ITEM_KEYWORD,
                    found,
                    format!("Did you mean \"{}\"?", print_list(suggestions)),
                ));
            }
        };

        Ok(InterfaceItem { docs, ident, inner })
    }
}

impl<'a> FromTokens<'a> for Func {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let params = NamedTypeList::parse(tokens)?;
        let mut streaming = false;

        let result = if tokens.next_if_token(Token::RArrow)?.is_some() {
            if tokens.next_if_token(Token::Stream)?.is_some() {
                tokens.expect(Token::LessThan)?;
                let inner = FuncResult::parse(tokens)?;
                tokens.expect(Token::GreaterThan)?;

                streaming = true;

                Some(inner)
            } else {
                Some(FuncResult::parse(tokens)?)
            }
        } else {
            None
        };

        Ok(Func {
            params,
            result,
            streaming,
        })
    }
}

impl<'a> FromTokens<'a> for NamedTypeList {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        parse_list(
            tokens,
            Token::LeftParen,
            Token::RightParen,
            Some(Token::Comma),
        )
    }
}

impl<'a> FromTokens<'a> for (Span, Type) {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let (_, ident) = tokens.expect(Token::Ident)?;

        tokens.expect(Token::Colon)?;

        let ty = Type::parse(tokens)?;

        Ok((ident, ty))
    }
}

impl<'a> FromTokens<'a> for FuncResult {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        if let Some((Ok(Token::LeftParen), _)) = tokens.peek() {
            Ok(FuncResult::Named(NamedTypeList::parse(tokens)?))
        } else {
            Ok(FuncResult::Anon(Type::parse(tokens)?))
        }
    }
}

impl<'a> FromTokens<'a> for RecordField {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect(Token::Ident)?;

        tokens.expect(Token::Colon)?;

        let ty = Type::parse(tokens)?;

        Ok(RecordField { ident, docs, ty })
    }
}

impl<'a> FromTokens<'a> for FlagsField {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect(Token::Ident)?;

        Ok(FlagsField { ident, docs })
    }
}

impl<'a> FromTokens<'a> for VariantCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect(Token::Ident)?;

        let ty = if tokens.next_if_token(Token::LeftParen)?.is_some() {
            let ty = Type::parse(tokens)?;
            tokens.expect(Token::RightParen)?;

            Some(ty)
        } else {
            None
        };

        Ok(VariantCase { ident, docs, ty })
    }
}

impl<'a> FromTokens<'a> for EnumCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let (_, ident) = tokens.expect(Token::Ident)?;

        Ok(EnumCase { ident, docs })
    }
}

impl<'a> FromTokens<'a> for UnionCase {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        let ty = Type::parse(tokens)?;

        Ok(UnionCase { docs, ty })
    }
}

impl<'a> FromTokens<'a> for Method {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = parse_docs(tokens);

        tokens.expect(Token::Func)?;

        let (_, ident) = tokens.expect(Token::Ident)?;

        let inner = Func::parse(tokens)?;

        Ok(Method { ident, docs, inner })
    }
}

impl<'a> FromTokens<'a> for Type {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let (token, span) = tokens.next().ok_or(Error::UnexpectedEof)?;

        match token? {
            Token::Bool => Ok(Self::Bool),
            Token::U8 => Ok(Self::U8),
            Token::U16 => Ok(Self::U16),
            Token::U32 => Ok(Self::U32),
            Token::U64 => Ok(Self::U64),
            Token::U128 => Ok(Self::U128),
            Token::S8 => Ok(Self::S8),
            Token::S16 => Ok(Self::S16),
            Token::S32 => Ok(Self::S32),
            Token::S64 => Ok(Self::S64),
            Token::S128 => Ok(Self::S128),
            Token::Float32 => Ok(Self::Float32),
            Token::Float64 => Ok(Self::Float64),
            Token::Char => Ok(Self::Char),
            Token::String => Ok(Self::String),
            Token::List => {
                tokens.expect(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect(Token::GreaterThan)?;

                Ok(Self::List(Box::new(ty)))
            }
            Token::Tuple => {
                let types = parse_list(
                    tokens,
                    Token::LessThan,
                    Token::GreaterThan,
                    Some(Token::Comma),
                )?;

                Ok(Self::Tuple(types))
            }
            Token::Option => {
                tokens.expect(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect(Token::GreaterThan)?;

                Ok(Self::Option(Box::new(ty)))
            }
            Token::Result => {
                let mut ok = None;
                let mut err = None;

                if tokens.next_if_token(Token::LessThan)?.is_some() {
                    if tokens.next_if_token(Token::Underscore)?.is_some() {
                        tokens.expect(Token::Comma)?;
                        err = Some(Box::new(Type::parse(tokens)?));
                    } else {
                        ok = Some(Box::new(Type::parse(tokens)?));

                        if tokens.next_if_token(Token::Comma)?.is_some() {
                            err = Some(Box::new(Type::parse(tokens)?));
                        }
                    };
                    tokens.expect(Token::GreaterThan)?;
                };

                Ok(Self::Result { ok, err })
            }
            Token::Ident => Ok(Self::Id(span)),
            found => Err(Error::unexpected_token(span, Token::TYPE_KEYWORD, found)),
        }
    }
}

fn parse_list<'a, O>(
    tokens: &mut Tokens<'a>,
    start: Token,
    end: Token,
    sep: Option<Token>,
) -> Result<Vec<O>>
where
    O: FromTokens<'a>,
{
    tokens.expect(start)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.next_if_token(end)?.is_some() {
            break;
        }

        let item = FromTokens::parse(tokens)?;
        items.push(item);

        // if there's no trailing separator then this is required to be the end,
        // otherwise we go through the loop to try to get another item
        if let Some(sep) = sep {
            if tokens.next_if_token(sep)?.is_none() {
                tokens.expect(end)?;
                break;
            }
        }
    }
    Ok(items)
}

fn parse_docs(tokens: &mut Tokens) -> Vec<Span> {
    let mut spans = Vec::new();

    while let Some((Ok(Token::DocComment | Token::BlockDocComment), span)) = tokens.peek() {
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
            func take_char(x: char)
            /// A function that returns a character
            func return_char() -> char
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

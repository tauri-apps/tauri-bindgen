use super::{
    lex::{Token, Tokens},
    Docs, Enum, EnumCase, Flags, FlagsField, Func, Interface, InterfaceItem, InterfaceItemKind,
    NamedType, NamedTypeList, Record, RecordField, Results, Type, Union, UnionCase, Use, UseName,
    UseNames, Variant, VariantCase,
};
use crate::{
    util::{find_similar, print_list},
    Error,
};
use miette::{bail, Result};

pub trait FromTokens<'a>
where
    Self: Sized,
{
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self>;
}

impl<'a> FromTokens<'a> for Interface<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        tokens.expect_token(Token::Interface)?;

        let name = tokens.expect_token(Token::Id)?;

        tokens.expect_token(Token::LeftBrace)?;

        let mut items = Vec::new();
        loop {
            // if we found an end token then we're done
            if tokens.take_token(Token::RightBrace)? {
                break;
            }

            items.push(InterfaceItem::parse(tokens)?);
        }

        Ok(Interface { docs, name, items })
    }
}

impl<'a> FromTokens<'a> for InterfaceItem<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let (loc, kind) = tokens.next().ok_or(Error::UnexpectedEof)??;

        let name = tokens.expect_token(Token::Id)?;

        let kind = match kind {
            Token::Record => {
                let fields = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemKind::Record(Record { fields })
            }
            Token::Enum => {
                let cases = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemKind::Enum(Enum { cases })
            }
            Token::Flags => {
                let fields = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemKind::Flags(Flags { fields })
            }
            Token::Variant => {
                let loc = tokens.location()?;
                let cases = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                if cases.is_empty() {
                    bail!(Error::empty_type(loc, Token::Variant))
                }

                InterfaceItemKind::Variant(Variant { cases })
            }
            Token::Union => {
                let cases = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                InterfaceItemKind::Union(Union { cases })
            }
            Token::Type => {
                tokens.expect_token(Token::Equals)?;
                InterfaceItemKind::Alias(FromTokens::parse(tokens)?)
            }
            Token::Func => InterfaceItemKind::Func(FromTokens::parse(tokens)?),
            Token::Use => InterfaceItemKind::Use(FromTokens::parse(tokens)?),

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

                let suggestions = find_similar(
                    expected.iter().map(ToString::to_string),
                    tokens.read_span(loc),
                )?;

                if !suggestions.is_empty() {
                    bail!(Error::unexpected_token_with_help(
                        loc,
                        expected,
                        found,
                        format!("Did you mean \"{}\"?", print_list(suggestions)),
                    ))
                } else {
                    bail!(Error::unexpected_token(loc, expected, found))
                }
            }
        };

        Ok(InterfaceItem { docs, name, kind })
    }
}

impl<'a> FromTokens<'a> for RecordField<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let name = tokens.expect_token(Token::Id)?;

        tokens.expect_token(Token::Colon)?;

        let ty = Type::parse(tokens)?;

        Ok(RecordField { docs, name, ty })
    }
}

impl<'a> FromTokens<'a> for FlagsField<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let name = tokens.expect_token(Token::Id)?;

        Ok(Self { docs, name })
    }
}

impl<'a> FromTokens<'a> for VariantCase<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let name = tokens.expect_token(Token::Id)?;

        let ty = if tokens.take_token(Token::LeftParen)? {
            let ty = Type::parse(tokens)?;
            tokens.expect_token(Token::RightParen)?;

            Some(ty)
        } else {
            None
        };

        Ok(VariantCase { docs, name, ty })
    }
}

impl<'a> FromTokens<'a> for UnionCase<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let ty = Type::parse(tokens)?;

        Ok(UnionCase { docs, ty })
    }
}

impl<'a> FromTokens<'a> for EnumCase<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let docs = Docs::parse(tokens)?;

        let name = tokens.expect_token(Token::Id)?;

        Ok(EnumCase { docs, name })
    }
}

impl<'a> FromTokens<'a> for Func {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let params = NamedTypeList::parse(tokens)?;

        let results = if tokens.take_token(Token::RArrow)? {
            Results::parse(tokens)?
        } else {
            Results::Anon(Type::Tuple(vec![]))
        };

        Ok(Func { params, results })
    }
}

impl<'a> FromTokens<'a> for NamedTypeList {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let inner = list(tokens, Token::LeftParen, Token::RightParen)?;

        Ok(Self { inner })
    }
}

impl<'a> FromTokens<'a> for NamedType {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let name = tokens.expect_token(Token::Id)?;

        tokens.expect_token(Token::Colon)?;

        let ty = Type::parse(tokens)?;

        Ok(Self { name, ty })
    }
}

impl<'a> FromTokens<'a> for Results {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        if tokens.clone().take_token(Token::LeftParen)? {
            Ok(Results::Named(NamedTypeList::parse(tokens)?))
        } else {
            Ok(Results::Anon(Type::parse(tokens)?))
        }
    }
}

impl<'a> FromTokens<'a> for Use {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let use_names = UseNames::parse(tokens)?;

        tokens.expect_token(Token::From)?;

        // TODO this should be a string literal
        let from = tokens.expect_token(Token::Id)?;

        Ok(Self { use_names, from })
    }
}

impl<'a> FromTokens<'a> for UseNames {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        // TODO should be peek()
        let (loc, token) = tokens.clone().next().ok_or(Error::UnexpectedEof)??;

        match token {
            Token::Star => {
                let _ = tokens.next();

                Ok(UseNames::All)
            }
            Token::LeftBrace => {
                let names = list(tokens, Token::LeftBrace, Token::RightBrace)?;

                Ok(UseNames::Subset(names))
            }
            found => bail!(Error::unexpected_token(
                loc,
                [Token::Star, Token::LeftBrace],
                found
            )),
        }
    }
}

impl<'a> FromTokens<'a> for UseName {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let name = tokens.expect_token(Token::Id)?;

        let alias = if tokens.take_token(Token::As)? {
            let alias = tokens.expect_token(Token::Id)?;
            Some(alias)
        } else {
            None
        };

        Ok(UseName { name, alias })
    }
}

fn list<'a, O>(tokens: &mut Tokens<'a>, start: Token, end: Token) -> Result<Vec<O>>
where
    O: FromTokens<'a>,
{
    tokens.expect_token(start)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.take_token(end)? {
            break;
        }

        let item = FromTokens::parse(tokens)?;
        items.push(item);

        // if there's no trailing comma then this is required to be the end,
        // otherwise we go through the loop to try to get another item
        if !tokens.take_token(Token::Comma)? {
            tokens.expect_token(end)?;
            break;
        }
    }
    Ok(items)
}

impl<'a> FromTokens<'a> for Type {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let (pos, token) = tokens.next().ok_or(Error::UnexpectedEof)??;

        let ty = match token {
            Token::U8 => Type::U8,
            Token::U16 => Type::U16,
            Token::U32 => Type::U32,
            Token::U64 => Type::U64,
            Token::S8 => Type::S8,
            Token::S16 => Type::S16,
            Token::S32 => Type::S32,
            Token::S64 => Type::S64,
            Token::Float32 => Type::Float32,
            Token::Float64 => Type::Float64,
            Token::Char => Type::Char,
            Token::String => Type::String,
            Token::Bool => Type::Bool,
            Token::List => {
                tokens.expect_token(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect_token(Token::GreaterThan)?;

                Type::List(Box::new(ty))
            }
            Token::Tuple => {
                let types = list(tokens, Token::LessThan, Token::GreaterThan)?;

                Type::Tuple(types)
            }
            Token::Option => {
                tokens.expect_token(Token::LessThan)?;
                let ty = Type::parse(tokens)?;
                tokens.expect_token(Token::GreaterThan)?;

                Type::Option(Box::new(ty))
            }
            Token::Result => {
                let mut ok = None;
                let mut err = None;

                if tokens.take_token(Token::LessThan)? {
                    if tokens.take_token(Token::Underscore)? {
                        tokens.expect_token(Token::Comma)?;
                        err = Some(Box::new(Type::parse(tokens)?));
                    } else {
                        ok = Some(Box::new(Type::parse(tokens)?));

                        if tokens.take_token(Token::Comma)? {
                            err = Some(Box::new(Type::parse(tokens)?));
                        }
                    };
                    tokens.expect_token(Token::GreaterThan)?;
                };
                Type::Result { ok, err }
            }
            Token::Id => Type::Id(pos),
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

                bail!(Error::unexpected_token(pos, expected, found))
            }
        };

        Ok(ty)
    }
}

impl<'a> FromTokens<'a> for Docs<'a> {
    fn parse(tokens: &mut Tokens<'a>) -> Result<Self> {
        let mut other = tokens.clone();

        let mut docs = Vec::new();

        while let Some(Ok((span, token))) = other.next() {
            match token {
                Token::DocComment => docs.push(tokens.read_span(span)),
                _ => break,
            };
            *tokens = other.clone();
        }

        Ok(Self { docs })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use miette::{NamedSource, Result};

    #[test]
    fn feature() -> Result<()> {
        let input = "record foo {}";
        let mut tokens = Tokens::from_str(input);

        let ty = InterfaceItem::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        println!("{:#?}", ty);

        Ok(())
    }

    #[test]
    fn option() -> Result<()> {
        let input = "option<u8>";
        let mut tokens = Tokens::from_str(input);

        let ty = Type::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        assert_eq!(ty, Type::Option(Box::new(Type::U8)));

        Ok(())
    }

    #[test]
    fn result() -> Result<()> {
        let input = "result<u8, string>";
        let mut tokens = Tokens::from_str(input);

        let ty = Type::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

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
        let input = "result<_, string>";
        let mut tokens = Tokens::from_str(input);

        let ty = Type::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

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
        let input = "type foo = result<u8, string>";
        let mut tokens = Tokens::from_str(input);

        let ty = InterfaceItem::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        // assert_eq!(ty.name, "foo");
        assert_eq!(
            ty.kind,
            InterfaceItemKind::Alias(Type::Result {
                ok: Some(Box::new(Type::U8)),
                err: Some(Box::new(Type::String))
            })
        );

        Ok(())
    }

    #[test]
    fn enum__() -> Result<()> {
        let input = "enum color {
            red,
            green,
            blue,
            yellow,
            other,
        }
        ";
        let mut tokens = Tokens::from_str(input);

        let _ty = InterfaceItem::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

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
        let input = "flags properties {
            lego,
            marvel-superhero,
            supervillain,
        }";
        let mut tokens = Tokens::from_str(input);

        let _ty = InterfaceItem::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

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
        let input = "union configuration {
            string,
            list<string>,
        }
        ";
        let mut tokens = Tokens::from_str(input);

        let _ty = Interface::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        // assert_eq!(ty.name, "configuration");
        // assert_eq!(
        //     ty.cases.into_iter().map(|c| c.ty).collect::<Vec<_>>(),
        //     vec![Type::String, Type::List(Box::new(Type::String))]
        // );
        Ok(())
    }

    #[test]
    fn variant_() -> Result<()> {
        let input = "variant filter {
            all,
            none,
            some(list<string>),
        }
        ";
        let mut tokens = Tokens::from_str(input);

        let _ty = InterfaceItem::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

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
        let input = "interface chars {
            /// A function that accepts a character
            func take-char(x: char)
            /// A function that returns a character
            func return-char() -> char
          }";

        let mut tokens = Tokens::from_str(input);

        let iface = Interface::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        println!("{:#?}", iface);

        Ok(())
    }

    #[test]
    fn full() -> Result<()> {
        let input = include_str!("test.wit");

        let mut tokens = Tokens::from_str(input);

        let iface = Interface::parse(&mut tokens)
            .map_err(|error| error.with_source_code(NamedSource::new("test.wit", input)))?;

        println!("{:#?}", iface);

        Ok(())
    }
}

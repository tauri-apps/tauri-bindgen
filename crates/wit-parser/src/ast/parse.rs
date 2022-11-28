use super::{
    lex::{self, Span, Token, Tokenizer},
    Alias, Docs, Enum, EnumCase, Flag, Flags, Func, Interface, InterfaceItem, Record, RecordField,
    Result_, Results, Tuple, Type, Union, UnionCase, Variant, VariantCase,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Lex(#[from] lex::Error),
    #[error("Unexpected Token {token:?}")]
    Unexpected {
        start: usize,
        end: usize,
        token: Token,
    },
    #[error("Unexpected end of file")]
    UnexpectedEof,
}

pub fn interface<'a>(tokens: &mut Tokenizer<'a>) -> Result<Interface<'a>, Error> {
    let iface_docs = docs(tokens)?;

    tokens.expect_token(Token::Interface)?;

    let name = tokens.expect_token(Token::Id)?;

    tokens.expect_token(Token::LeftBrace)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.take_token(Token::RightBrace)? {
            break;
        }

        let item_docs = docs(tokens)?;

       items.push(interface_item(tokens, item_docs)?);
    }

    Ok(Interface {
        docs: iface_docs,
        name,
        items,
    })
}

fn interface_item<'a>(
    tokens: &mut Tokenizer<'a>,
    docs: Docs<'a>,
) -> Result<InterfaceItem<'a>, Error> {
    match tokens.clone().next().ok_or(Error::UnexpectedEof)?? {
        (_, Token::Id) => func(tokens, docs).map(InterfaceItem::Func),
        (_, Token::Record) =>record(tokens, docs).map(InterfaceItem::Record),
        (_, Token::Variant) => variant(tokens, docs).map(InterfaceItem::Variant),
        (_, Token::Union) => union(tokens, docs).map(InterfaceItem::Union),
        (_, Token::Flags) => flags(tokens, docs).map(InterfaceItem::Flags),
        (_, Token::Enum) => enum_(tokens, docs).map(InterfaceItem::Enum),
        (_, Token::Type) => alias(tokens, docs).map(InterfaceItem::Alias),
        (span, token) => Err(Error::Unexpected {
            start: span.start,
            end: span.end,
            token,
        }),
    }
}

fn record<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Record<'a>, Error> {
    tokens.expect_token(Token::Record)?;
    let name = tokens.expect_token(Token::Id)?;

    let fields = list(tokens, Token::LeftBrace, record_field, Token::RightBrace)?;

    Ok(Record { docs, name, fields })
}

fn record_field<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<RecordField<'a>, Error> {
    let (name, ty) = named_type_(tokens)?;

    Ok(RecordField { docs, name, ty })
}

fn variant<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Variant<'a>, Error> {
    tokens.expect_token(Token::Variant)?;
    let name = tokens.expect_token(Token::Id)?;

    let cases = list(tokens, Token::LeftBrace, variant_case, Token::RightBrace)?;

    Ok(Variant { docs, name, cases })
}

fn variant_case<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<VariantCase<'a>, Error> {
    let name = tokens.expect_token(Token::Id)?;

    let ty = if tokens.take_token(Token::LeftParen)? {
        let ty = type_(tokens)?;
        tokens.expect_token(Token::RightParen)?;

        Some(ty)
    } else {
        None
    };

    Ok(VariantCase { docs, name, ty })
}

fn union<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Union<'a>, Error> {
    tokens.expect_token(Token::Union)?;
    let name = tokens.expect_token(Token::Id)?;

    let cases = list(tokens, Token::LeftBrace, union_case, Token::RightBrace)?;

    Ok(Union { docs, name, cases })
}

fn union_case<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<UnionCase<'a>, Error> {
    let ty = type_(tokens)?;

    Ok(UnionCase { docs, ty })
}

fn flags<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Flags<'a>, Error> {
    tokens.expect_token(Token::Flags)?;
    let name = tokens.expect_token(Token::Id)?;

    let fields = list(tokens, Token::LeftBrace, flag, Token::RightBrace)?;

    Ok(Flags { docs, name, flags: fields })
}

fn flag<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Flag<'a>, Error> {
    let name = tokens.expect_token(Token::Id)?;

    Ok(Flag { docs, name })
}

fn enum_<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Enum<'a>, Error> {
    tokens.expect_token(Token::Enum)?;
    let name = tokens.expect_token(Token::Id)?;

    let cases = list(tokens, Token::LeftBrace, enum_case, Token::RightBrace)?;

    Ok(Enum { docs, name, cases })
}

fn enum_case<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<EnumCase<'a>, Error> {
    let name = tokens.expect_token(Token::Id)?;

    Ok(EnumCase { docs, name })
}

fn alias<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Alias<'a>, Error> {
    tokens.expect_token(Token::Type)?;
    let name = tokens.expect_token(Token::Id)?;

    tokens.expect_token(Token::Equals)?;

    let ty = type_(tokens)?;

    Ok(Alias { docs, name, ty })
}

fn func<'a>(tokens: &mut Tokenizer<'a>, docs: Docs<'a>) -> Result<Func<'a>, Error> {
    let name = tokens.expect_token(Token::Id)?;

    tokens.expect_token(Token::Colon)?;
    tokens.expect_token(Token::Func)?;

    let params = list_no_docs(tokens, Token::LeftParen, named_type_, Token::RightParen)?;

    let results = if tokens.take_token(Token::RArrow)? {
       results(tokens)?
    } else {
        Results::Named(Vec::new())
    };

    Ok(Func {
        docs,
        name,
        params,
        results,
    })
}

fn results<'a>(tokens: &mut Tokenizer<'a>) -> Result<Results<'a>, Error> {
    if tokens.clone().take_token(Token::LeftParen)? {
        Ok(Results::Named(list_no_docs(
            tokens,
            Token::LeftParen,
            named_type_,
            Token::RightParen,
        )?))
    } else {
        Ok(Results::Anon(type_(tokens)?))
    }
}

fn named_type_<'a>(tokens: &mut Tokenizer<'a>) -> Result<(Span<'a>, Type<'a>), Error> {
    let name = tokens.expect_token(Token::Id)?;

    tokens.expect_token(Token::Colon)?;

    let ty = type_(tokens)?;

    Ok((name, ty))
}

fn docs<'a>(tokens: &mut Tokenizer<'a>) -> Result<Docs<'a>, Error> {
    let mut other = tokens.clone();

    let mut docs = Vec::new();

    while let Some(Ok((span, token))) = other.next() {
        match token {
            Token::DocComment => docs.push(span),
            _ => break,
        };
        *tokens = other.clone();
    }

    Ok(Docs { docs })
}

fn type_<'a>(tokens: &mut Tokenizer<'a>) -> Result<Type<'a>, Error> {
    match tokens.next().ok_or(Error::UnexpectedEof)?? {
        (_, Token::U8) => Ok(Type::U8),
        (_, Token::U16) => Ok(Type::U16),
        (_, Token::U32) => Ok(Type::U32),
        (_, Token::U64) => Ok(Type::U64),
        (_, Token::S8) => Ok(Type::S8),
        (_, Token::S16) => Ok(Type::S16),
        (_, Token::S32) => Ok(Type::S32),
        (_, Token::S64) => Ok(Type::S64),
        (_, Token::Float32) => Ok(Type::Float32),
        (_, Token::Float64) => Ok(Type::Float64),
        (_, Token::Char) => Ok(Type::Char),
        (_, Token::String_) => Ok(Type::String),
        (_, Token::Bool) => Ok(Type::Bool),
        (_, Token::Tuple) => {
            let types = list_no_docs(tokens, Token::LessThan, type_, Token::GreaterThan)?;

            Ok(Type::Tuple(Tuple { types }))
        }
        (_, Token::List) => {
            tokens.expect_token(Token::LessThan)?;
            let ty = type_(tokens)?;
            tokens.expect_token(Token::GreaterThan)?;
            Ok(Type::List(Box::new(ty)))
        }
        (_, Token::Option_) => {
            tokens.expect_token(Token::LessThan)?;
            let ty = type_(tokens)?;
            tokens.expect_token(Token::GreaterThan)?;
            Ok(Type::Option(Box::new(ty)))
        }
        (_, Token::Result_) => {
            let mut ok = None;
            let mut err = None;

            if tokens.take_token(Token::LessThan)? {
                if tokens.take_token(Token::Underscore)? {
                    tokens.expect_token(Token::Comma)?;
                    err = Some(type_(tokens)?);
                } else {
                    ok = Some(type_(tokens)?);

                    if tokens.take_token(Token::Comma)? {
                        err = Some(type_(tokens)?);
                    }
                };
                tokens.expect_token(Token::GreaterThan)?;
            };

            Ok(Type::Result(Box::new(Result_ { ok, err })))
        }
        (span, Token::Id) => {
            Ok(Type::Id(span))
        },
        (span, token) => Err(Error::Unexpected {
            start: span.start,
            end: span.end,
            token,
        }),
    }
}

fn list<'a, O>(
    tokens: &mut Tokenizer<'a>,
    start: Token,
    mut inner: impl FnMut(&mut Tokenizer<'a>, Docs<'a>) -> Result<O, Error>,
    end: Token,
) -> Result<Vec<O>, Error> {
    tokens.expect_token(start)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.take_token(end)? {
            break;
        }

        let docs = docs(tokens)?;

        let item = inner(tokens, docs)?;
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

fn list_no_docs<'a, O>(
    tokens: &mut Tokenizer<'a>,
    start: Token,
    mut inner: impl FnMut(&mut Tokenizer<'a>) -> Result<O, Error>,
    end: Token,
) -> Result<Vec<O>, Error> {
    tokens.expect_token(start)?;

    let mut items = Vec::new();
    loop {
        // if we found an end token then we're done
        if tokens.take_token(end)? {
            break;
        }

        let item = inner(tokens)?;
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn option() {
        let input = "option<u8>";
        let mut tokens = Tokenizer::from_str(input);

        let ty = type_(&mut tokens).unwrap();

        assert_eq!(ty, Type::Option(Box::new(Type::U8)));
    }

    #[test]
    fn result() {
        let input = "result<u8, string>";
        let mut tokens = Tokenizer::from_str(input);

        let ty = type_(&mut tokens).unwrap();

        assert_eq!(
            ty,
            Type::Result(Box::new(Result_ {
                ok: Some(Type::U8),
                err: Some(Type::String)
            }))
        );
    }

    #[test]
    fn result2() {
        let input = "result<_, string>";
        let mut tokens = Tokenizer::from_str(input);

        let ty = type_(&mut tokens).unwrap();

        assert_eq!(
            ty,
            Type::Result(Box::new(Result_ {
                ok: None,
                err: Some(Type::String)
            }))
        );
    }

    #[test]
    fn typedef_() {
        let input = "type_ foo = result<u8, string>";
        let mut tokens = Tokenizer::from_str(input);

        let ty = alias(&mut tokens, Docs::default()).unwrap();

        assert_eq!(ty.name.as_str(), "foo");
        assert_eq!(
            ty.ty,
            Type::Result(Box::new(Result_ {
                ok: Some(Type::U8),
                err: Some(Type::String)
            }))
        );
    }

    #[test]
    fn enum__() {
        let input = "enum color {
            red,
            green,
            blue,
            yellow,
            other,
        }
        ";
        let mut tokens = Tokenizer::from_str(input);

        let ty = enum_(&mut tokens, Docs::default()).unwrap();

        assert_eq!(ty.name.as_str(), "color");
        assert_eq!(
            ty.cases
                .into_iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>(),
            vec!["red", "green", "blue", "yellow", "other"]
        );
    }

    #[test]
    fn flags_() {
        let input = "flags properties {
            lego,
            marvel-superhero,
            supervillain,
        }";
        let mut tokens = Tokenizer::from_str(input);

        let ty = flags(&mut tokens, Docs::default()).unwrap();

        assert_eq!(ty.name.as_str(), "properties");
        assert_eq!(
            ty.flags
                .into_iter()
                .map(|c| c.name.as_str())
                .collect::<Vec<_>>(),
            vec!["lego", "marvel-superhero", "supervillain"]
        );
    }

    #[test]
    fn union_() {
        let input = "union configuration {
            string,
            list<string>,
        }
        ";
        let mut tokens = Tokenizer::from_str(input);

        let ty = union(&mut tokens, Docs::default()).unwrap();

        assert_eq!(ty.name.as_str(), "configuration");
        assert_eq!(
            ty.cases.into_iter().map(|c| c.ty).collect::<Vec<_>>(),
            vec![Type::String, Type::List(Box::new(Type::String))]
        );
    }

    #[test]
    fn variant_() {
        let input = "variant filter {
            all,
            none,
            some(list<string>),
        }
        ";
        let mut tokens = Tokenizer::from_str(input);

        let ty = variant(&mut tokens, Docs::default()).unwrap();

        assert_eq!(ty.name.as_str(), "filter");
        assert_eq!(ty.cases[0].name.as_str(), "all");
        assert_eq!(ty.cases[0].ty, None);

        assert_eq!(ty.cases[1].name.as_str(), "none");
        assert_eq!(ty.cases[1].ty, None);

        assert_eq!(ty.cases[2].name.as_str(), "some");
        assert_eq!(ty.cases[2].ty, Some(Type::List(Box::new(Type::String))));
    }

    #[test]
    fn interface_() {
        let input = "interface chars {
            /// A function that accepts a character
            take-char: func(x: char)
            /// A function that returns a character
            return-char: func() -> char
          }";

        let mut tokens = Tokenizer::from_str(input);

        let iface = interface(&mut tokens).unwrap();

        println!("{:?}", iface);
    }

    #[test]
    fn full() {
        let input = include_str!("test.wit");

        let mut tokens = Tokenizer::from_str(input);

        let iface = interface(&mut tokens).unwrap();

        println!("{:?}", iface);
    }
}

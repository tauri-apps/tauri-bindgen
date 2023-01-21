use std::iter::Peekable;

use miette::{bail, Result, SourceSpan};

use crate::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Whitespace,
    Comment,
    DocComment,
    Id,

    // operators
    Equals,
    Comma,
    Colon,
    Semicolon,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    Star,
    RArrow,
    Underscore,

    // keywords
    Type,
    Resource,
    Func,
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
    Record,
    Enum,
    Flags,
    Variant,
    Union,
    Bool,
    Option,
    Result,
    List,
    Interface,
    Tuple,

    // reserved but currently unused
    Use,
    As,
    From,
    Static,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Whitespace => write!(f, "whitespace"),
            Token::Comment => write!(f, "a comment"),
            Token::DocComment => write!(f, "a doc comment"),
            Token::Id => write!(f, "an identifier"),
            Token::Equals => write!(f, "'='"),
            Token::Comma => write!(f, "','"),
            Token::Colon => write!(f, "':'"),
            Token::Semicolon => write!(f, "';'"),
            Token::LeftParen => write!(f, "'('"),
            Token::RightParen => write!(f, "')'"),
            Token::LeftBrace => write!(f, "'{{'"),
            Token::RightBrace => write!(f, "'}}'"),
            Token::LessThan => write!(f, "'<'"),
            Token::GreaterThan => write!(f, "'>'"),
            Token::Star => write!(f, "'*'"),
            Token::RArrow => write!(f, "'->'"),
            Token::Underscore => write!(f, "'_'"),
            Token::Type => write!(f, "'type'"),
            Token::Resource => write!(f, "'resource'"),
            Token::Func => write!(f, "'func'"),
            Token::U8 => write!(f, "'u8'"),
            Token::U16 => write!(f, "'u16'"),
            Token::U32 => write!(f, "'u32'"),
            Token::U64 => write!(f, "'u64'"),
            Token::S8 => write!(f, "'s8'"),
            Token::S16 => write!(f, "'s16'"),
            Token::S32 => write!(f, "'s32'"),
            Token::S64 => write!(f, "'s64'"),
            Token::Float32 => write!(f, "'float32'"),
            Token::Float64 => write!(f, "'float64'"),
            Token::Char => write!(f, "'char'"),
            Token::String => write!(f, "'string'"),
            Token::Record => write!(f, "'record'"),
            Token::Enum => write!(f, "'enum'"),
            Token::Flags => write!(f, "'flags'"),
            Token::Variant => write!(f, "'variant'"),
            Token::Union => write!(f, "'union'"),
            Token::Bool => write!(f, "'bool'"),
            Token::Option => write!(f, "'option'"),
            Token::Result => write!(f, "'result'"),
            Token::List => write!(f, "'list'"),
            Token::Interface => write!(f, "'interface'"),
            Token::Tuple => write!(f, "'tuple'"),
            Token::Use => write!(f, "'use'"),
            Token::As => write!(f, "'as'"),
            Token::From => write!(f, "'from'"),
            Token::Static => write!(f, "'static'"),
        }
    }
}

#[derive(Debug, Clone)]
struct CrlfFold<'a> {
    chars: core::str::CharIndices<'a>,
}

impl<'a> Iterator for CrlfFold<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(i, c)| {
            if c == '\r' {
                let mut attempt = self.chars.clone();
                if let Some((_, '\n')) = attempt.next() {
                    self.chars = attempt;
                    return (i, '\n');
                }
            }
            (i, c)
        })
    }
}

#[derive(Debug, Clone)]
pub struct TokensRaw<'a> {
    src: &'a str,
    chars: Peekable<CrlfFold<'a>>,
}

impl<'a> TokensRaw<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src,
            chars: CrlfFold {
                chars: src.char_indices(),
            }
            .peekable(),
        }
    }

    pub fn read_span(&self, span: SourceSpan) -> &'a str {
        &self.src[span.offset()..span.offset() + span.len()]
    }

    fn take_char(&mut self, exp: char) -> bool {
        self.chars
            .next_if(|(_, got)| *got == exp)
            .map(|e| e.1)
            .is_some()
    }

    fn take_char_while(&mut self, predicate: impl Fn(char) -> bool) -> Option<&'a str> {
        let (start, _) = self.chars.next_if(|(_, ch)| predicate(*ch))?;
        let mut end = start;

        while let Some((pos, _)) = self.chars.next_if(|(_, ch)| predicate(*ch)) {
            end = pos;
        }

        Some(&self.src[start..=end])
    }
}

impl<'a> Iterator for TokensRaw<'a> {
    type Item = Result<(SourceSpan, Token)>;

    fn next(&mut self) -> Option<Self::Item> {
        let (mut start, ch) = self.chars.next()?;

        let token = match ch {
            ' ' | '\t' | '\n' => {
                self.take_char_while(is_space);
                Token::Whitespace
            }
            '/' => {
                let (loc, char) = self.chars.next()?;

                match char {
                    '/' if self.chars.peek().map(|e| e.1) == Some('/') => {
                        self.take_char_while(not_line_ending);
                        Token::DocComment
                    }
                    '*' if self.chars.peek().map(|e| e.1) == Some('*') => {
                        let mut depth = 1;
                        while depth > 0 {
                            let (_, ch) = match self.chars.next() {
                                Some(pair) => pair,
                                None => {
                                    return Some(Err(Error::unterminated_comment(
                                        start,
                                        self.src.len() - 1,
                                    )
                                    .into()))
                                }
                            };
                            match ch {
                                '/' if self.take_char('*') => depth += 1,
                                '*' if self.take_char('/') => depth -= 1,
                                _ => {}
                            }
                        }
                        Token::DocComment
                    }
                    '/' => {
                        self.take_char_while(not_line_ending);
                        Token::Comment
                    }
                    '*' => {
                        let mut depth = 1;
                        while depth > 0 {
                            let (_, ch) = match self.chars.next() {
                                Some(pair) => pair,
                                None => {
                                    return Some(Err(Error::unterminated_comment(
                                        start,
                                        self.src.len() - 1,
                                    )
                                    .into()))
                                }
                            };
                            match ch {
                                '/' if self.take_char('*') => depth += 1,
                                '*' if self.take_char('/') => depth -= 1,
                                _ => {}
                            }
                        }
                        Token::Comment
                    }
                    ch => return Some(Err(Error::unexpected_char_with_help(
                        loc,
                        ch,
                        "A comment is either a line preeeded with `//` or enclosed with `/*` `*/`",
                    )
                    .into())),
                }
            }
            '=' => Token::Equals,
            ',' => Token::Comma,
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            '{' => Token::LeftBrace,
            '}' => Token::RightBrace,
            '<' => Token::LessThan,
            '>' => Token::GreaterThan,
            '*' => Token::Star,
            '-' if self.take_char('>') => Token::RArrow,
            '_' => Token::Underscore,

            '%' => {
                self.take_char_while(is_identifier);

                // skip the percent sign
                start += 1;

                Token::Id
            }

            c if is_alphabetic(c) => {
                let start = self.chars.peek().map(|e| e.0 - 1).unwrap_or_default();
                let len = self
                    .take_char_while(is_identifier)
                    .unwrap_or_default()
                    .len();

                let token = &self.src[start..=start + len];

                match token {
                    "use" => Token::Use,
                    "type" => Token::Type,
                    "resource" => Token::Resource,
                    "func" => Token::Func,
                    "u8" => Token::U8,
                    "u16" => Token::U16,
                    "u32" => Token::U32,
                    "u64" => Token::U64,
                    "s8" => Token::S8,
                    "s16" => Token::S16,
                    "s32" => Token::S32,
                    "s64" => Token::S64,
                    "float32" => Token::Float32,
                    "float64" => Token::Float64,
                    "char" => Token::Char,
                    "record" => Token::Record,
                    "flags" => Token::Flags,
                    "variant" => Token::Variant,
                    "enum" => Token::Enum,
                    "union" => Token::Union,
                    "bool" => Token::Bool,
                    "string" => Token::String,
                    "option" => Token::Option,
                    "result" => Token::Result,
                    "list" => Token::List,
                    "_" => Token::Underscore,
                    "as" => Token::As,
                    "from" => Token::From,
                    "static" => Token::Static,
                    "interface" => Token::Interface,
                    "tuple" => Token::Tuple,
                    _ => Token::Id,
                }
            }
            ch => {
                return Some(Err(Error::unexpected_char(start, ch).into()));
            }
        };

        let end = match self.chars.clone().next() {
            Some((i, _)) => i,
            None => self.src.len(),
        };

        Some(Ok((SourceSpan::from(start..end), token)))
    }
}

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    tokenizer: TokensRaw<'a>,
}

impl<'a> Tokens<'a> {
    pub fn from_str(src: &'a str) -> Self {
        Self {
            tokenizer: TokensRaw::new(src),
        }
    }

    pub fn read_span(&self, span: SourceSpan) -> &'a str {
        self.tokenizer.read_span(span)
    }

    pub fn location(&mut self) -> Result<SourceSpan> {
        match self.clone().next() {
            Some(Ok((loc, _))) => Ok(loc),
            Some(Err(err)) => Err(err),
            None => {
                let pos = self.tokenizer.src.len();
                Ok((pos..pos).into())
            }
        }
    }

    pub fn expect_token(&mut self, expected: Token) -> Result<SourceSpan> {
        match self.next() {
            Some(Ok((loc, found))) => {
                if expected == found {
                    Ok(loc)
                } else {
                    bail!(Error::unexpected_token(loc, [expected], found));
                }
            }
            Some(Err(err)) => Err(err),
            None => bail!(Error::UnexpectedEof),
        }
    }

    pub fn take_token(&mut self, expected: Token) -> Result<bool> {
        let mut other = self.clone();
        match other.next() {
            Some(Ok((_span, found))) if expected == found => {
                *self = other;
                Ok(true)
            }
            Some(Err(e)) => Err(e),
            _ => Ok(false),
        }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Result<(SourceSpan, Token)>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.tokenizer.next()? {
                Ok((_, Token::Whitespace)) | Ok((_, Token::Comment)) => continue,
                res => return Some(res),
            }
        }
    }
}

#[inline]
fn is_alphabetic(chr: char) -> bool {
    ('A'..='Z').contains(&chr) || ('a'..='z').contains(&chr)
}

#[inline]
fn is_digit(chr: char) -> bool {
    ('0'..='9').contains(&chr)
}

#[inline]
fn is_identifier(chr: char) -> bool {
    is_alphabetic(chr) || is_digit(chr) || chr == '-'
}

#[inline]
fn is_space(chr: char) -> bool {
    chr == ' ' || chr == '\t'
}

#[inline]
fn not_line_ending(chr: char) -> bool {
    chr != '\n'
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn comment() {
        let input = "// this is a comment";
        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![((0..20).into(), Token::Comment),]);

        let input = "/* this is a comment */";
        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![((0..23).into(), Token::Comment),]);

        let input = "/* this is a comment /* with nested comments */ */";
        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![((0..50).into(), Token::Comment),]);
    }

    #[test]
    fn basic() {
        let input = "interface this-is-an-interface {}";

        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(
            tokens,
            vec![
                ((0..10).into(), Token::Interface),
                ((9..10).into(), Token::Whitespace),
                ((10..30).into(), Token::Id),
                ((30..31).into(), Token::Whitespace),
                ((31..32).into(), Token::LeftBrace),
                ((32..33).into(), Token::RightBrace),
            ]
        );
    }

    #[test]
    fn basicv() {
        let input = "interface this-is-an-interface {}";

        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(
            tokens,
            vec![
                ((0..9).into(), Token::Interface),
                ((9..10).into(), Token::Whitespace),
                ((10..30).into(), Token::Id),
                ((30..31).into(), Token::Whitespace),
                ((31..32).into(), Token::LeftBrace),
                ((32..33).into(), Token::RightBrace),
            ]
        );
    }

    #[test]
    fn explicit_id() {
        let input = "%foo: func(a: char, b: u8,) -> %bar";

        let tokens: Vec<_> = TokensRaw::new(input).filter_map(Result::ok).collect();

        assert_eq!(
            tokens,
            vec![
                ((1..4).into(), Token::Id),
                ((4..5).into(), Token::Colon),
                ((5..6).into(), Token::Whitespace),
                ((6..10).into(), Token::Func),
                ((10..11).into(), Token::LeftParen),
                ((11..12).into(), Token::Id),
                ((12..13).into(), Token::Colon),
                ((13..14).into(), Token::Whitespace),
                ((14..18).into(), Token::Char),
                ((18..19).into(), Token::Comma),
                ((19..20).into(), Token::Whitespace),
                ((20..21).into(), Token::Id),
                ((21..22).into(), Token::Colon),
                ((22..23).into(), Token::Whitespace),
                ((23..25).into(), Token::U8),
                ((25..26).into(), Token::Comma),
                ((26..27).into(), Token::RightParen),
                ((27..28).into(), Token::Whitespace),
                ((28..30).into(), Token::RArrow),
                ((30..31).into(), Token::Whitespace),
                ((32..35).into(), Token::Id),
            ],
        );
    }
}

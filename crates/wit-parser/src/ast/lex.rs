use core::{
    fmt::{self, Debug},
    iter::Peekable,
};
use std::fmt::Display;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Unterminated Comment")]
    UnterminatedComment(usize),
    #[error("Unexpected character {1}")]
    Unexpected(usize, char),
    #[error("Unexpected end of file")]
    UnexpectedEof,
    #[error("Unexpected Token. Expected {expected:?} but found {found:?}")]
    Wanted {
        at: usize,
        expected: Token,
        found: Token,
    },
}

/// Iterator over chars, but it cleans up newlines for us
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

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct Span<'a> {
    pub start: usize,
    pub end: usize,
    /// The fragment that is spanned.
    /// The fragment represents a part of the input of the parser.
    input: &'a str,
}

impl<'a> Span<'a> {
    pub fn new(fragment: &'a str, start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            input: fragment,
        }
    }

    pub fn as_str(&self) -> &'a str {
        &self.input[self.start..self.end]
    }
}

impl<'a> Debug for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Span(\"{}\", {}-{})",
            self.as_str(),
            self.start,
            self.end
        )
    }
}

impl<'a> Display for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: Peekable<CrlfFold<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn from_str(input: &'a str) -> Self {
        Self {
            input,
            chars: CrlfFold {
                chars: input.char_indices(),
            }
            .peekable(),
        }
    }

    pub fn take_token(&mut self, expected: Token) -> Result<bool, Error> {
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

    pub fn expect_token(&mut self, expected: Token) -> Result<Span<'a>, Error> {
        match self.next() {
            Some(Ok((span, found))) => {
                if expected == found {
                    Ok(span)
                } else {
                    return Err(Error::Wanted {
                        at: span.start,
                        expected,
                        found,
                    });
                }
            }
            Some(Err(err)) => return Err(err),
            None => return Err(Error::UnexpectedEof),
        }
    }

    fn take_char(&mut self, exp: char) -> bool {
        self.chars
            .next_if(|(_, got)| *got == exp)
            .map(|e| e.1)
            .is_some()
    }

    fn take_while(&mut self, predicate: impl Fn(char) -> bool) -> Option<&'a str> {
        let (start, _) = self.chars.next_if(|(_, ch)| predicate(*ch))?;
        let mut end = start;

        while let Some((pos, _)) = self.chars.next_if(|(_, ch)| predicate(*ch)) {
            end = pos;
        }

        Some(&self.input[start..=end])
    }

    fn next_raw(&mut self) -> Option<Result<(Span<'a>, Token), Error>> {
        use Token::*;

        let (mut start, ch) = match self.chars.next() {
            Some(pair) => pair,
            None => return None,
        };

        let token = match ch {
            ' ' | '\t' | '\n' => {
                self.take_while(is_space);
                Whitespace
            }
            '/' => match self.chars.next()? {
                (_, '/') if self.chars.peek().map(|e| e.1) == Some('/') => {
                    self.take_while(not_line_ending);
                    DocComment
                }
                (_, '*') if self.chars.peek().map(|e| e.1) == Some('*') => {
                    let mut depth = 1;
                    while depth > 0 {
                        let (_, ch) = match self.chars.next() {
                            Some(pair) => pair,
                            None => return Some(Err(Error::UnterminatedComment(start))),
                        };
                        match ch {
                            '/' if self.take_char('*') => depth += 1,
                            '*' if self.take_char('/') => depth -= 1,
                            _ => {}
                        }
                    }
                    DocComment
                }
                (_, '/') => {
                    self.take_while(not_line_ending);
                    Comment
                }
                (_, '*') => {
                    let mut depth = 1;
                    while depth > 0 {
                        let (_, ch) = match self.chars.next() {
                            Some(pair) => pair,
                            None => return Some(Err(Error::UnterminatedComment(start))),
                        };
                        match ch {
                            '/' if self.take_char('*') => depth += 1,
                            '*' if self.take_char('/') => depth -= 1,
                            _ => {}
                        }
                    }
                    Comment
                }
                (pos, ch) => return Some(Err(Error::Unexpected(pos, ch))),
            },
            '=' => Equals,
            ',' => Comma,
            ':' => Colon,
            ';' => Semicolon,
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '<' => LessThan,
            '>' => GreaterThan,
            '*' => Star,
            '-' if self.take_char('>') => RArrow,
            '_' => Underscore,

            '%' => {
                self.take_while(is_identifier);

                // skip the percent sign
                start += 1;

                Id
            }

            c if is_alphabetic(c) => {
                let start = self.chars.peek().map(|e| e.0 - 1).unwrap_or_default();
                let len = self.take_while(is_identifier).unwrap_or_default().len();

                let token = &self.input[start..=start + len];

                match token {
                    "use" => Use,
                    "type" => Type,
                    "resource" => Resource,
                    "func" => Func,
                    "u8" => U8,
                    "u16" => U16,
                    "u32" => U32,
                    "u64" => U64,
                    "s8" => S8,
                    "s16" => S16,
                    "s32" => S32,
                    "s64" => S64,
                    "float32" => Float32,
                    "float64" => Float64,
                    "char" => Char,
                    "record" => Record,
                    "flags" => Flags,
                    "variant" => Variant,
                    "enum" => Enum,
                    "union" => Union,
                    "bool" => Bool,
                    "string" => String_,
                    "option" => Option_,
                    "result" => Result_,
                    "list" => List,
                    "_" => Underscore,
                    "as" => As,
                    "from" => From_,
                    "static" => Static,
                    "interface" => Interface,
                    "tuple" => Tuple,
                    _ => Id,
                }
            }
            ch => return Some(Err(Error::Unexpected(start, ch))),
        };

        let end = match self.chars.clone().next() {
            Some((i, _)) => i,
            None => self.input.len(),
        };

        Some(Ok((
            Span {
                start,
                end,
                input: self.input,
            },
            token,
        )))
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<(Span<'a>, Token), Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next_raw()? {
                Ok((_, Token::Whitespace)) | Ok((_, Token::Comment)) => continue,
                res => return Some(res),
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token {
    Whitespace,
    Comment,
    DocComment,
    Id,
    // StrLit,

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
    String_,
    Record,
    Enum,
    Flags,
    Variant,
    Union,
    Bool,
    Option_,
    Result_,
    List,
    Interface,
    Tuple,

    // reserved but currently unused
    Use,
    As,
    From_,
    Static,
}

#[inline]
fn is_alphabetic(chr: char) -> bool {
    (chr >= 'A' && chr <= 'Z') || (chr >= 'a' && chr <= 'z')
}

#[inline]
fn is_digit(chr: char) -> bool {
    chr >= '0' && chr <= '9'
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
    !(chr == '\n')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unexpected_char() {
        let input = "\\";

        let mut tokens = Tokenizer::from_str(input);

        assert_eq!(tokens.next(), Some(Err(Error::Unexpected(0, '\\'))));
    }

    #[test]
    fn malformed_comment() {
        let input = "/- this is a comment";
        let mut tokens = Tokenizer::from_str(input);

        assert_eq!(tokens.next(), Some(Err(Error::Unexpected(1, '-'))));

        let input = "/* this is a comment";
        let mut tokens = Tokenizer::from_str(input);

        assert_eq!(tokens.next(), Some(Err(Error::UnterminatedComment(0))));

        let input = "/* this is a comment /* with nested comments */";
        let mut tokens = Tokenizer::from_str(input);

        assert_eq!(tokens.next(), Some(Err(Error::UnterminatedComment(0))));
    }

    #[test]
    fn comment() {
        let input = "// this is a comment";
        let tokens: Vec<_> = Tokenizer::from_str(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![(Span::new(input, 0, 20), Token::Comment),]);

        let input = "/* this is a comment */";
        let tokens: Vec<_> = Tokenizer::from_str(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![(Span::new(input, 0, 23), Token::Comment),]);

        let input = "/* this is a comment /* with nested comments */ */";
        let tokens: Vec<_> = Tokenizer::from_str(input).filter_map(Result::ok).collect();

        assert_eq!(tokens, vec![(Span::new(input, 0, 50), Token::Comment),]);
    }

    #[test]
    fn basic() {
        let input = "interface this-is-an-interface {}";

        let tokens: Vec<_> = Tokenizer::from_str(input).filter_map(Result::ok).collect();

        assert_eq!(
            tokens,
            vec![
                (Span::new(input, 0, 9), Token::Interface),
                (Span::new(input, 9, 10), Token::Whitespace),
                (Span::new(input, 10, 30), Token::Id),
                (Span::new(input, 30, 31), Token::Whitespace),
                (Span::new(input, 31, 32), Token::LeftBrace),
                (Span::new(input, 32, 33), Token::RightBrace),
            ]
        );
    }

    #[test]
    fn explicit_id() {
        let input = "%foo: func(a: char, b: u8,) -> %bar";

        let tokens: Vec<_> = Tokenizer::from_str(input).filter_map(Result::ok).collect();

        assert_eq!(
            tokens,
            vec![
                (Span::new(input, 0, 4), Token::Id),
                (Span::new(input, 4, 5), Token::Colon),
                (Span::new(input, 5, 6), Token::Whitespace),
                (Span::new(input, 6, 10), Token::Func),
                (Span::new(input, 10, 11), Token::LeftParen),
                (Span::new(input, 11, 12), Token::Id),
                (Span::new(input, 12, 13), Token::Colon),
                (Span::new(input, 13, 14), Token::Whitespace),
                (Span::new(input, 14, 18), Token::Char),
                (Span::new(input, 18, 19), Token::Comma),
                (Span::new(input, 19, 20), Token::Whitespace),
                (Span::new(input, 20, 21), Token::Id),
                (Span::new(input, 21, 22), Token::Colon),
                (Span::new(input, 22, 23), Token::Whitespace),
                (Span::new(input, 23, 25), Token::U8),
                (Span::new(input, 25, 26), Token::Comma),
                (Span::new(input, 26, 27), Token::RightParen),
                (Span::new(input, 27, 28), Token::Whitespace),
                (Span::new(input, 28, 30), Token::RArrow),
                (Span::new(input, 30, 31), Token::Whitespace),
                (Span::new(input, 31, 35), Token::Id),
            ]
        );
    }
}

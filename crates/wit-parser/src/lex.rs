use logos::{FilterResult, Lexer, Logos, Source};

fn block_comment(lex: &mut Lexer<Token>) -> FilterResult<()> {
    let mut depth = 1;
    while depth > 0 {
        let remainder = lex.remainder();
        match remainder.slice(0..2) {
            Some("/*") => depth += 1,
            Some("*/") => depth -= 1,
            None => return FilterResult::Error,
            _ => {}
        }

        // comments might include multi-byte unicode code points
        // and since `Lexer::bump()` panics if it ends up in the middle of a code point we find the next valid character boundary here and jump to that.
        let mut bump_by = 1;
        while !remainder.is_char_boundary(bump_by) {
            bump_by += 1;
        }
        lex.bump(bump_by);
    }

    lex.bump(1);

    FilterResult::Emit(())
}

#[derive(Logos, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Whitespace,
    #[regex("(//[^\n]*)", logos::skip)]
    Comment,
    #[regex(r#"/\*"#, |lex| match block_comment(lex) {
        FilterResult::Emit(_) => FilterResult::Skip,
        v => v
    })]
    BlockComment,
    #[regex("(///[^\n]*)")]
    DocComment,
    #[regex(r#"/\*\*"#, block_comment)]
    BlockDocComment,
    #[regex("%?([a-zA-Z0-9_])+")]
    Ident,

    // operators
    #[token("=")]
    Equals,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("<")]
    LessThan,
    #[token(">")]
    GreaterThan,
    #[token("*")]
    Star,
    #[token("->")]
    RArrow,
    #[token("_")]
    Underscore,

    // keywords
    #[token("type")]
    Type,
    #[token("resource")]
    Resource,
    #[token("func")]
    Func,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("s8")]
    S8,
    #[token("s16")]
    S16,
    #[token("s32")]
    S32,
    #[token("s64")]
    S64,
    #[token("float32")]
    Float32,
    #[token("float64")]
    Float64,
    #[token("char")]
    Char,
    #[token("string")]
    String,
    #[token("record")]
    Record,
    #[token("enum")]
    Enum,
    #[token("flags")]
    Flags,
    #[token("variant")]
    Variant,
    #[token("union")]
    Union,
    #[token("bool")]
    Bool,
    #[token("option")]
    Option,
    #[token("result")]
    Result,
    #[token("list")]
    List,
    #[token("interface")]
    Interface,
    #[token("tuple")]
    Tuple,

    // reserved but currently unused
    #[token("use")]
    Use,
    #[token("as")]
    As,
    #[token("from")]
    From,
    #[token("static")]
    Static,

    #[error]
    UnexpectedCharacter,
}

impl Token {
    pub const IFACE_ITEM_KEYWORD: [Token; 8] = [
        Token::Enum,
        Token::Flags,
        Token::Func,
        Token::Record,
        Token::Type,
        Token::Union,
        Token::Variant,
        Token::Resource,
    ];
    pub const TYPE_KEYWORD: [Token; 18] = [
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
        Token::Ident,
    ];
    pub fn as_str(&self) -> &str {
        match self {
            Token::Whitespace => "whitespace",
            Token::Comment | Token::BlockComment => "a comment",
            Token::DocComment | Token::BlockDocComment => "a doc comment",
            Token::Ident => "an identifier",
            Token::Equals => "'='",
            Token::Comma => "','",
            Token::Colon => "':'",
            Token::LeftParen => "'('",
            Token::RightParen => "')'",
            Token::LeftBrace => "'{{'",
            Token::RightBrace => "'}}'",
            Token::LessThan => "'<'",
            Token::GreaterThan => "'>'",
            Token::Star => "'*'",
            Token::RArrow => "'->'",
            Token::Underscore => "'_'",
            Token::Type => "'type'",
            Token::Resource => "'resource'",
            Token::Func => "'func'",
            Token::U8 => "'u8'",
            Token::U16 => "'u16'",
            Token::U32 => "'u32'",
            Token::U64 => "'u64'",
            Token::S8 => "'s8'",
            Token::S16 => "'s16'",
            Token::S32 => "'s32'",
            Token::S64 => "'s64'",
            Token::Float32 => "'float32'",
            Token::Float64 => "'float64'",
            Token::Char => "'char'",
            Token::String => "'string'",
            Token::Record => "'record'",
            Token::Enum => "'enum'",
            Token::Flags => "'flags'",
            Token::Variant => "'variant'",
            Token::Union => "'union'",
            Token::Bool => "'bool'",
            Token::Option => "'option'",
            Token::Result => "'result'",
            Token::List => "'list'",
            Token::Interface => "'interface'",
            Token::Tuple => "'tuple'",
            Token::Use => "'use'",
            Token::As => "'as'",
            Token::From => "'from'",
            Token::Static => "'static'",
            Token::UnexpectedCharacter => "an unexpected character error",
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comment() {
        let mut lex = Token::lexer("/* this is a comment */");
        assert_eq!(lex.next(), None);

        let mut lex = Token::lexer("// this is a comment");
        assert_eq!(lex.next(), None);
    }

    #[test]
    fn doc_comment() {
        let mut lex = Token::lexer("/// this is a comment");
        assert_eq!(lex.next(), Some(Token::DocComment));

        let mut lex = Token::lexer("/** this is a comment */");
        assert_eq!(lex.next(), Some(Token::BlockDocComment));
    }

    #[test]
    fn ident() {
        let mut lex = Token::lexer("foo");
        assert_eq!(lex.next(), Some(Token::Ident));

        let mut lex = Token::lexer("foo_bar");
        assert_eq!(lex.next(), Some(Token::Ident));

        let mut lex = Token::lexer("%foo");
        assert_eq!(lex.next(), Some(Token::Ident));

        let mut lex = Token::lexer("%foo_bar");
        assert_eq!(lex.next(), Some(Token::Ident));
    }
}

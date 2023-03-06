use logos::{FilterResult, Lexer, Logos, Source};

fn block_comment(lex: &mut Lexer<Token>) -> FilterResult<()> {
    let mut depth = 1;
    while depth > 0 {
        let Some(next_two) = lex
            .remainder()
            .slice(0..2) else {
                return FilterResult::Error;
            };

        match next_two {
            "/*" => depth += 1,
            "*/" => depth -= 1,
            _ => {}
        }
        lex.bump(1);
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Whitespace => write!(f, "whitespace"),
            Token::Comment => write!(f, "a comment"),
            Token::BlockComment => write!(f, "a comment"),
            Token::DocComment => write!(f, "a doc comment"),
            Token::BlockDocComment => write!(f, "a doc comment"),
            Token::Ident => write!(f, "an identifier"),
            Token::Equals => write!(f, "'='"),
            Token::Comma => write!(f, "','"),
            Token::Colon => write!(f, "':'"),
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
            Token::UnexpectedCharacter => write!(f, "an unexpected character error"),
        }
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

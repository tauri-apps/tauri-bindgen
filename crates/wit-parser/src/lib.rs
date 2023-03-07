mod lex;
mod parse;
pub mod typecheck;
mod util;
mod error;

pub use error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;

use logos::Logos;
use parse::FromTokens;
pub use typecheck::*;

pub fn parse_str(input: impl AsRef<str>, _skip: impl Fn(&str) -> bool) -> Result<Interface> {
    let input = input.as_ref();
    let mut tokens = lex::Token::lexer(input).spanned().peekable();
    let iface = parse::Interface::parse(&mut tokens)?;
    let iface = typecheck::Resolver::new(input, &iface).resolve()?;

    Ok(iface)
}
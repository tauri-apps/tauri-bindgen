mod error;
mod lex;
mod parse;
pub mod typecheck;
mod util;

pub use error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;

pub use typecheck::*;

pub type TypeDefId = Id<TypeDef>;
pub type TypeDefArena = Arena<TypeDef>;

use id_arena::{Arena, Id};
use logos::Logos;
use miette::{ErrReport, IntoDiagnostic, NamedSource};
use parse::FromTokens;
use std::path::Path;

pub fn parse_str(input: impl AsRef<str>, skip: impl Fn(&str) -> bool) -> miette::Result<Interface> {
    let iface = parse(input.as_ref(), skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new("virtual file", input.as_ref().to_string()))
    })?;

    Ok(iface)
}

pub fn parse_file(
    path: impl AsRef<Path>,
    skip: impl Fn(&str) -> bool,
) -> miette::Result<Interface> {
    let path = path.as_ref();
    let input = std::fs::read_to_string(path).into_diagnostic()?;

    let iface = parse(&input, skip).map_err(|error: ErrReport| {
        error.with_source_code(NamedSource::new(path.to_string_lossy(), input))
    })?;

    Ok(iface)
}

fn parse(input: &str, _skip: impl Fn(&str) -> bool) -> miette::Result<Interface> {
    let mut tokens = lex::Token::lexer(input).spanned().peekable();

    let iface = parse::Interface::parse(&mut tokens)?;

    let iface = typecheck::Resolver::new(input, &iface).resolve()?;

    Ok(iface)
}

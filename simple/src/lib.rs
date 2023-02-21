mod lex;
mod parse;
mod typecheck;
mod codegen;

mod error;
mod util;

pub use error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;
mod lex;
mod parse;
pub mod typecheck;
mod util;
mod error;

pub use error::Error;
pub(crate) type Result<T> = std::result::Result<T, error::Error>;

pub use typecheck::*;
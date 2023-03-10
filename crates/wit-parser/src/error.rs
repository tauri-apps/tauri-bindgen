use logos::Span;
use miette::Diagnostic;

use crate::{lex::Token, util::print_list};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("expected {}, but found {found}", print_list(expected))]
    #[diagnostic(code(wit_parser::unexpected_token))]
    UnexpectedToken {
        #[label("unexpected token")]
        location: Span,
        expected: Vec<Token>,
        found: Token,
        #[help]
        help: Option<String>,
    },
    #[error("type {ty} cannot be empty")]
    #[diagnostic(code(wit_parser::empty_type))]
    EmptyType {
        #[label("this type cannot be empty")]
        location: Span,
        ty: Token,
    },
    /// A name wasn't defined.
    #[error("name is not defined.")]
    #[diagnostic(code(wit_parser::not_defined))]
    NotDefined {
        #[label("name is not defined")]
        location: Span,
        #[help]
        help: Option<String>,
    },
    /// Names can't be defined more than once.
    #[error("name already defined.")]
    #[diagnostic(code(wit_parser::already_defined))]
    AlreadyDefined {
        #[label("name is already declared here")]
        previous: Span,
        #[label("cannot be defined twice")]
        location: Span,
    },
    /// Types can't recursively refer to themselves as that would make then possibly infinitly-sized.
    /// In Rust the compiler would force you to use heap indirection, however such a thing doesn't exist in out type system.
    ///
    /// This wouldn't be a problem with the current JSON format, but a custom binary one would have this limitation so for future proofing we deny recursive types.
    #[error("Type cannot refer to itself.")]
    #[diagnostic(code(wit_parser::recursive_type))]
    RecursiveType {
        #[label("type cannot refer to itself")]
        location: Span,
    },
    #[error("Unused variable")]
    #[diagnostic(code(wit_parser::unused_type))]
    UnusedType {
        #[label("remove this type")]
        location: Span,
    },
    #[error("Failed with multiple errors:")]
    Multi {
        #[related]
        errors: Vec<Error>,
    },
}

impl FromIterator<Error> for Error {
    fn from_iter<T: IntoIterator<Item = Error>>(iter: T) -> Self {
        Self::Multi {
            errors: iter.into_iter().collect(),
        }
    }
}

impl Error {
    pub fn unexpected_token(
        loc: impl Into<Span>,
        expected: impl IntoIterator<Item = Token>,
        found: Token,
    ) -> Self {
        Self::UnexpectedToken {
            location: loc.into(),
            expected: expected.into_iter().collect(),
            found,
            help: None,
        }
    }

    pub fn unexpected_token_with_help(
        loc: impl Into<Span>,
        expected: impl IntoIterator<Item = Token>,
        found: Token,
        advice: impl Into<String>,
    ) -> Self {
        Self::UnexpectedToken {
            location: loc.into(),
            expected: expected.into_iter().collect(),
            found,
            help: Some(advice.into()),
        }
    }

    pub fn empty_type(loc: impl Into<Span>, ty: Token) -> Self {
        Self::EmptyType {
            location: loc.into(),
            ty,
        }
    }

    pub fn not_defined(loc: impl Into<Span>) -> Self {
        Self::NotDefined {
            location: loc.into(),
            help: None,
        }
    }

    pub fn not_defined_with_help(loc: impl Into<Span>, help: impl Into<String>) -> Self {
        Self::NotDefined {
            location: loc.into(),
            help: Some(help.into()),
        }
    }

    pub fn already_defined(loc: impl Into<Span>, prev: impl Into<Span>) -> Self {
        Self::AlreadyDefined {
            previous: prev.into(),
            location: loc.into(),
        }
    }

    pub fn recursive_type(loc: impl Into<Span>) -> Self {
        Self::RecursiveType {
            location: loc.into(),
        }
    }

    pub fn unused_type(loc: impl Into<Span>) -> Self {
        Self::UnusedType {
            location: loc.into(),
        }
    }
}

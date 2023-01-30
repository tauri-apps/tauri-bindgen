use crate::{ast::lex::Token, util::print_list};
use miette::{Diagnostic, ErrReport, SourceSpan};

#[derive(Debug, thiserror::Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(url(docsrs))]
    IoError(#[from] std::io::Error),
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unterminated block comment")]
    #[diagnostic(
        code(wit_parser::unterminated_comment),
        url(docsrs),
        help("Try adding '*/' at the end of your comment")
    )]
    UnterminatedComment {
        #[label("because '/*' begins a comment")]
        start: SourceSpan,
        #[label("expected comment ending here")]
        end: SourceSpan,
    },
    #[error("expected char '{found}'")]
    #[diagnostic(url(docsrs))]
    UnexpectedCharacter {
        #[label("unexpected character")]
        location: SourceSpan,
        found: char,
        #[help]
        help: Option<String>,
    },
    #[error("expected {}, but found {found}", print_list(expected))]
    #[diagnostic(url(docsrs))]
    UnexpectedToken {
        #[label("unexpected token")]
        location: SourceSpan,
        expected: Vec<Token>,
        found: Token,
        #[help]
        help: Option<String>,
    },
    #[error("type {ty} cannot be empty")]
    #[diagnostic(url(docsrs), help("Types cannot be empty"))]
    EmptyType {
        #[label("this type cannot be empty")]
        location: SourceSpan,
        ty: Token,
    },
    /// A name wasn't defined.
    #[error("name is not defined.")]
    #[diagnostic(url(docsrs))]
    NotDefined {
        #[label("is not defined")]
        location: SourceSpan,
    },
    /// Names can't be defined more than once.
    #[error("name already defined.")]
    #[diagnostic(url(docsrs))]
    AlreadyDefined {
        #[label("name is already declared here")]
        previous: SourceSpan,
        #[label("cannot be defined twice")]
        location: SourceSpan,
    },
    /// Types can't recursively refer to themselves as that would make then possibly infinitly-sized.
    /// In Rust the compiler would force you to use heap indirection, however such a thing doesn't exist in out type system.
    ///
    /// This wouldn't be a problem with the current JSON format, but a custom binary one would have this limitation so for future proofing we deny recursive types.
    #[error("Type cannot refer to itself.")]
    #[diagnostic(url(docsrs))]
    RecursiveType {
        #[label("type cannot refer to itself")]
        location: SourceSpan,
    },
}

#[derive(Diagnostic, Debug, thiserror::Error)]
#[error("oops: multiple errors")]
#[diagnostic()]
pub struct MultiError {
    #[related]
    related: Vec<ErrReport>,
}

impl From<Vec<ErrReport>> for MultiError {
    fn from(related: Vec<ErrReport>) -> Self {
        Self { related }
    }
}

impl Error {
    pub fn unterminated_comment(start: impl Into<SourceSpan>, end: impl Into<SourceSpan>) -> Self {
        Self::UnterminatedComment {
            start: start.into(),
            end: end.into(),
        }
    }

    pub fn unexpected_char(loc: impl Into<SourceSpan>, found: char) -> Self {
        Self::UnexpectedCharacter {
            location: loc.into(),
            found,
            help: None,
        }
    }

    pub fn unexpected_char_with_help(
        loc: impl Into<SourceSpan>,
        found: char,
        advice: impl Into<String>,
    ) -> Self {
        Self::UnexpectedCharacter {
            location: loc.into(),
            found,
            help: Some(advice.into()),
        }
    }

    pub fn unexpected_token(
        loc: impl Into<SourceSpan>,
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
        loc: impl Into<SourceSpan>,
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

    pub fn empty_type(loc: impl Into<SourceSpan>, ty: Token) -> Self {
        Self::EmptyType {
            location: loc.into(),
            ty,
        }
    }

    pub fn not_defined(loc: impl Into<SourceSpan>) -> Self {
        Self::NotDefined {
            location: loc.into(),
        }
    }

    pub fn already_defined(loc: impl Into<SourceSpan>, prev: impl Into<SourceSpan>) -> Self {
        Self::AlreadyDefined {
            previous: prev.into(),
            location: loc.into(),
        }
    }

    pub fn recursive_type(loc: impl Into<SourceSpan>) -> Self {
        Self::RecursiveType {
            location: loc.into(),
        }
    }
}

impl IntoIterator for MultiError {
    type Item = ErrReport;

    type IntoIter = std::vec::IntoIter<ErrReport>;

    fn into_iter(self) -> Self::IntoIter {
        self.related.into_iter()
    }
}

impl FromIterator<ErrReport> for MultiError {
    fn from_iter<T: IntoIterator<Item = ErrReport>>(iter: T) -> Self {
        Self {
            related: Vec::from_iter(iter),
        }
    }
}

impl From<ErrReport> for MultiError {
    fn from(err: ErrReport) -> Self {
        Self { related: vec![err] }
    }
}

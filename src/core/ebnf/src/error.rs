use super::span::{Span, Spanned, Spanning};
use super::{builder, lexer, parser};
use crate::impl_spanning;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    Lexer(lexer::error::Error),
    Parser(parser::error::Error),
    Builder(builder::error::Error),
}

impl_spanning!(Error);

impl From<Spanned<lexer::error::Error>> for Error {
    fn from(error: Spanned<lexer::error::Error>) -> Error {
        Error {
            kind: ErrorKind::Lexer(error.node),
            span: error.span,
        }
    }
}

impl From<Spanned<parser::error::Error>> for Error {
    fn from(error: Spanned<parser::error::Error>) -> Error {
        Error {
            kind: ErrorKind::Parser(error.node),
            span: error.span,
        }
    }
}

impl From<Spanned<builder::error::Error>> for Error {
    fn from(error: Spanned<builder::error::Error>) -> Error {
        Error {
            kind: ErrorKind::Builder(error.node),
            span: error.span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Lexer(inner) => write!(f, "{}", inner),
            ErrorKind::Parser(inner) => write!(f, "{}", inner),
            ErrorKind::Builder(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Lexer(inner) => Some(inner),
            ErrorKind::Parser(inner) => Some(inner),
            ErrorKind::Builder(inner) => Some(inner),
        }
    }
}

use super::span::{Span, Spanned, Spanning};
use super::{lexer, parser, preprocessor};
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
    Preprocessor(preprocessor::error::Error),
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

impl From<Spanned<preprocessor::error::Error>> for Error {
    fn from(error: Spanned<preprocessor::error::Error>) -> Error {
        Error {
            kind: ErrorKind::Preprocessor(error.node),
            span: error.span,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::Lexer(inner) => write!(f, "{}", inner),
            ErrorKind::Parser(inner) => write!(f, "{}", inner),
            ErrorKind::Preprocessor(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            ErrorKind::Lexer(inner) => Some(inner),
            ErrorKind::Parser(inner) => Some(inner),
            ErrorKind::Preprocessor(inner) => Some(inner),
        }
    }
}

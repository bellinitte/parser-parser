use super::{builder, lexer, parser};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    pub column: usize,
    pub line: usize,
}

impl Location {
    pub fn new() -> Location {
        Location { column: 0, line: 0 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Span {
    pub from: Location,
    pub to: Location,
}

impl Span {
    pub fn new() -> Span {
        Span {
            from: Location::new(),
            to: Location::new(),
        }
    }

    pub fn combine(start: &Span, end: &Span) -> Span {
        Span {
            from: start.from,
            to: end.to,
        }
    }

    pub fn between(start: &Span, end: &Span) -> Span {
        Span {
            from: start.to,
            to: end.from,
        }
    }
}

impl From<((usize, usize), (usize, usize))> for Span {
    fn from(tuples: ((usize, usize), (usize, usize))) -> Span {
        Span {
            from: Location {
                column: (tuples.0).0,
                line: (tuples.0).1,
            },
            to: Location {
                column: (tuples.1).0,
                line: (tuples.1).1,
            },
        }
    }
}

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

impl From<lexer::error::Error> for Error {
    fn from(error: lexer::error::Error) -> Error {
        let span = error.span.clone();
        Error {
            kind: ErrorKind::Lexer(error),
            span,
        }
    }
}

impl From<parser::error::Error> for Error {
    fn from(error: parser::error::Error) -> Error {
        let span = error.span.clone();
        Error {
            kind: ErrorKind::Parser(error),
            span,
        }
    }
}

impl From<builder::error::Error> for Error {
    fn from(error: builder::error::Error) -> Error {
        let span = error.span.clone();
        Error {
            kind: ErrorKind::Builder(error),
            span,
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

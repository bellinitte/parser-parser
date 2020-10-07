use super::{/*builder, */lexer/*, parser, scanner*/};
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Location {
    pub column: usize,
    pub line: usize,
}

impl Location {
    pub fn new() -> Location {
        Location {
            column: 0,
            line: 0,
        }
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
            }
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
    // Scanner(scanner::error::Error),
    Lexer(lexer::error::Error),
    // Parser(parser::error::Error),
    // Builder(builder::error::Error),
}

// impl From<scanner::error::Error> for Error {
//     fn from(error: scanner::error::Error) -> Error {
//         let position = error.position..error.position + 1;
//         Error {
//             kind: ErrorKind::Scanner(error),
//             position,
//         }
//     }
// }

impl From<lexer::error::Error> for Error {
    fn from(error: lexer::error::Error) -> Error {
        let span = error.span.clone();
        Error {
            kind: ErrorKind::Lexer(error),
            span,
        }
    }
}

// impl From<parser::error::Error> for Error {
//     fn from(error: parser::error::Error) -> Error {
//         let position = error.position.clone();
//         Error {
//             kind: ErrorKind::Parser(error),
//             position,
//         }
//     }
// }

// impl From<builder::error::Error> for Error {
//     fn from(error: builder::error::Error) -> Error {
//         let position = error.position.clone();
//         Error {
//             kind: ErrorKind::Builder(error),
//             position,
//         }
//     }
// }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            // ErrorKind::Scanner(inner) => write!(f, "{}", inner),
            ErrorKind::Lexer(inner) => write!(f, "{}", inner),
            // ErrorKind::Parser(inner) => write!(f, "{}", inner),
            // ErrorKind::Builder(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.kind {
            // ErrorKind::Scanner(inner) => Some(inner),
            ErrorKind::Lexer(inner) => Some(inner),
            // ErrorKind::Parser(inner) => Some(inner),
            // ErrorKind::Builder(inner) => Some(inner),
        }
    }
}

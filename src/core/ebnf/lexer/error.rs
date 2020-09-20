use std::{fmt, ops::Range};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidSymbol(char),
    AmbiguousSymbol,
    UnterminatedSpecial,
    UnterminatedComment,
    UnterminatedTerminal,
    EmptyTerminal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::InvalidSymbol(c) => write!(f, "invalid symbol '{}'", c),
            ErrorKind::AmbiguousSymbol => write!(f, "ambiguous symbol '(*)'"),
            ErrorKind::UnterminatedSpecial => write!(f, "unterminated special sequence"),
            ErrorKind::UnterminatedComment => write!(f, "unterminated comment"),
            ErrorKind::UnterminatedTerminal => write!(f, "unterminated terminal symbol"),
            ErrorKind::EmptyTerminal => write!(f, "empty terminal symbol"),
        }
    }
}

impl std::error::Error for Error {}

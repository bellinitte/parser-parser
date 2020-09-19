use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSymbol(char),
    AmbiguousSymbol,
    UnterminatedSpecial,
    UnterminatedComment,
    UnterminatedTerminal,
    EmptyTerminal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidSymbol(c) => write!(f, "invalid symbol '{}'", c),
            Error::AmbiguousSymbol => write!(f, "ambiguous symbol '(*)'"),
            Error::UnterminatedSpecial => write!(f, "unterminated special sequence"),
            Error::UnterminatedComment => write!(f, "unterminated comment"),
            Error::UnterminatedTerminal => write!(f, "unterminated terminal symbol"),
            Error::EmptyTerminal => write!(f, "empty terminal symbol"),
        }
    }
}

impl std::error::Error for Error {}

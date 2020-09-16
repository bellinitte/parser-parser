use nom::error::ErrorKind;
use nom::error::ParseError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnterminatedSpecial,
    UnterminatedComment,
    UnterminatedTerminal,
    EmptyTerminal,
    InvalidCharacter(char),
    Internal(ErrorKind),
}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Error::Internal(kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnterminatedSpecial => write!(f, "unterminated special sequence"),
            Error::UnterminatedComment => write!(f, "unterminated comment"),
            Error::UnterminatedTerminal => write!(f, "unterminated terminal symbol"),
            Error::EmptyTerminal => write!(f, "empty terminal symbol"),
            Error::InvalidCharacter(c) => write!(f, "invalid character {}", c),
            Error::Internal(kind) => write!(f, "{}", kind.description()),
        }
    }
}

impl std::error::Error for Error {}

use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidSymbol(String),
    UnterminatedSpecial,
    UnterminatedComment,
    UnterminatedTerminal,
    EmptyTerminal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Error::InvalidSymbol(s) => write!(f, "invalid symbol `{}`", s),
            Error::UnterminatedSpecial => write!(f, "unterminated special sequence"),
            Error::UnterminatedComment => write!(f, "unterminated comment"),
            Error::UnterminatedTerminal => write!(f, "unterminated terminal symbol"),
            Error::EmptyTerminal => write!(f, "empty terminal symbol"),
        }
    }
}

impl std::error::Error for Error {}

impl_spanning!(Error);

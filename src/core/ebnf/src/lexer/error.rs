use super::Span;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    InvalidSymbol(String),
    UnterminatedSpecial,
    UnterminatedComment,
    UnterminatedTerminal,
    EmptyTerminal,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::InvalidSymbol(s) => write!(f, "invalid symbol `{}`", s),
            ErrorKind::UnterminatedSpecial => write!(f, "unterminated special sequence"),
            ErrorKind::UnterminatedComment => write!(f, "unterminated comment"),
            ErrorKind::UnterminatedTerminal => write!(f, "unterminated terminal symbol"),
            ErrorKind::EmptyTerminal => write!(f, "empty terminal symbol"),
        }
    }
}

impl std::error::Error for Error {}

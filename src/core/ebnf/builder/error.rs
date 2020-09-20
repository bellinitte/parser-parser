use std::{fmt, ops::Range};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            _ => write!(f, ""),
        }
    }
}

impl std::error::Error for Error {}

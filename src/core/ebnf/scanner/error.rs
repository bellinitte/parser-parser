use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: usize,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    ControlCharacter(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::ControlCharacter(c) => write!(f, "control character {:#x?}", c),
        }
    }
}

impl std::error::Error for Error {}

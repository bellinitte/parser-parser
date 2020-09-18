use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    ControlCharacter(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ControlCharacter(c) => write!(f, "control character {:#x?}", c),
        }
    }
}

impl std::error::Error for Error {}

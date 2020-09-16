use super::ebnf;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Ebnf(ebnf::error::Error),
}

impl From<ebnf::error::Error> for Error {
    fn from(error: ebnf::error::Error) -> Error {
        Error::Ebnf(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Ebnf(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Ebnf(inner) => Some(inner),
        }
    }
}

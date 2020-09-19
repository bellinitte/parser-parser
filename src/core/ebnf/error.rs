use super::{builder, lexer, parser, scanner};
use std::fmt;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum Error {
    Scanner(scanner::error::Error),
    Lexer(lexer::error::Error),
    Parser(parser::error::Error),
    Builder(builder::error::Error),
}

impl From<scanner::error::Error> for Error {
    fn from(error: scanner::error::Error) -> Error {
        Error::Scanner(error)
    }
}

impl From<lexer::error::Error> for Error {
    fn from(error: lexer::error::Error) -> Error {
        Error::Lexer(error)
    }
}

impl From<parser::error::Error> for Error {
    fn from(error: parser::error::Error) -> Error {
        Error::Parser(error)
    }
}

impl From<builder::error::Error> for Error {
    fn from(error: builder::error::Error) -> Error {
        Error::Builder(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Scanner(inner) => write!(f, "{}", inner),
            Error::Lexer(inner) => write!(f, "{}", inner),
            Error::Parser(inner) => write!(f, "{}", inner),
            Error::Builder(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Scanner(inner) => Some(inner),
            Error::Lexer(inner) => Some(inner),
            Error::Parser(inner) => Some(inner),
            Error::Builder(inner) => Some(inner),
        }
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

use std::fmt;
use super::parser;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub enum Error {
    Parser(parser::error::Error),
}

impl From<parser::error::Error> for Error {
    fn from(error: parser::error::Error) -> Error {
        Error::Parser(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parser(inner) => write!(f, "{}", inner),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parser(inner) => Some(inner),
        }
    }
}

impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        JsValue::from_str(&self.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

use ebnf_parser_parser as ebnf;
use js_sys::{Object, Reflect};
use std::fmt;
use wasm_bindgen::prelude::*;

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

#[allow(unused_unsafe)]
impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        match self {
            Error::Ebnf(inner) => {
                let from = Object::new();
                unsafe {
                    Reflect::set(
                        &from,
                        &"line".into(),
                        &(inner.span.from.line as u32).into(),
                    )
                    .unwrap();
                    Reflect::set(
                        &from,
                        &"ch".into(),
                        &(inner.span.from.column as u32).into(),
                    )
                    .unwrap();
                }
                let to = Object::new();
                unsafe {
                    Reflect::set(
                        &to,
                        &"line".into(),
                        &(inner.span.to.line as u32).into(),
                    )
                    .unwrap();
                    Reflect::set(
                        &to,
                        &"ch".into(),
                        &(inner.span.to.column as u32).into(),
                    )
                    .unwrap();
                }
                let span = Object::new();
                unsafe {
                    Reflect::set(
                        &span,
                        &"from".into(),
                        &from.into(),
                    )
                    .unwrap();
                    Reflect::set(
                        &span,
                        &"to".into(),
                        &to.into(),
                    )
                    .unwrap();
                }
                let error = Object::new();
                unsafe {
                    Reflect::set(&error, &"kind".into(), &inner.to_string().into()).unwrap();
                    Reflect::set(&error, &"span".into(), &span.into()).unwrap();
                }
                return error.into();
            }
        }
    }
}

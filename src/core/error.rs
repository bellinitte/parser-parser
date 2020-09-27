use ebnf_parser_parser as ebnf;
use wasm_bindgen::prelude::*;
use js_sys::{Object, Reflect};
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

#[allow(unused_unsafe)]
impl Into<JsValue> for Error {
    fn into(self) -> JsValue {
        match self {
            Error::Ebnf(inner) => {
                let position = Object::new();
                unsafe {
                    Reflect::set(
                        &position,
                        &"start".into(),
                        &(inner.position.start as u32).into(),
                    )
                    .unwrap();
                    Reflect::set(&position, &"end".into(), &(inner.position.end as u32).into()).unwrap();
                }
                let error = Object::new();
                unsafe {
                    Reflect::set(&error, &"kind".into(), &inner.to_string().into()).unwrap();
                    Reflect::set(&error, &"position".into(), &position.into()).unwrap();
                }
                return error.into();
            }
        }
    }
}

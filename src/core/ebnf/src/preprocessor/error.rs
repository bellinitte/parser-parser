use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    UndefinedRule(String),
    LeftRecursion(Vec<String>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UndefinedRule(rule) => write!(f, "rule {} is undefined", rule),
            Error::LeftRecursion(chain) => {
                let chain_string = chain
                    .iter()
                    .map(|ident| ident.as_ref())
                    .collect::<Vec<_>>()
                    .join(" -> ");
                let rule = chain.first().unwrap();
                return write!(f, "rule {} is left recursive ({})", rule, chain_string);
            },
        }
    }
}

impl std::error::Error for Error {}

impl_spanning!(Error);

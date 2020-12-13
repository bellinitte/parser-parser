use ebnf;
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::prelude::*;
use std::fmt;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct EbnfParserParser {
    grammar: base::Grammar,
}

#[wasm_bindgen]
impl EbnfParserParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<EbnfParserParser, JsValue> {
        match ebnf::parse(input) {
            Ok(parser_parser) => Ok(EbnfParserParser {
                grammar: parser_parser,
            }),
            Err(e) => Err(Error::from(e).into()),
        }
    }

    #[wasm_bindgen(getter = productionRules)]
    pub fn get_production_rules(&self) -> Array {
        self.grammar.keys()
            .map(JsValue::from)
            .collect()
    }

    pub fn check(&self, input: &str, initial_rule: &str) -> Option<Object> {
        base::check(input, &self.grammar, initial_rule).map(tree)
    }
}

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
                    Reflect::set(&from, &"line".into(), &(inner.span.from.line as u32).into())
                        .unwrap();
                    Reflect::set(&from, &"ch".into(), &(inner.span.from.column as u32).into())
                        .unwrap();
                }
                let to = Object::new();
                unsafe {
                    Reflect::set(&to, &"line".into(), &(inner.span.to.line as u32).into()).unwrap();
                    Reflect::set(&to, &"ch".into(), &(inner.span.to.column as u32).into()).unwrap();
                }
                let span = Object::new();
                unsafe {
                    Reflect::set(&span, &"from".into(), &from.into()).unwrap();
                    Reflect::set(&span, &"to".into(), &to.into()).unwrap();
                }
                let error = Object::new();
                unsafe {
                    Reflect::set(&error, &"kind".into(), &inner.to_string().into()).unwrap();
                    Reflect::set(&error, &"span".into(), &span.into()).unwrap();
                }
                error.into()
            }
        }
    }
}

#[allow(unused_unsafe)]
fn tree(node: base::Node) -> Object {
    match node {
        base::Node::Terminal(string) => {
            let obj = Object::new();
            unsafe {
                Reflect::set(&obj, &"name".into(), &format!("\"{}\"", string).into())
                    .unwrap();
            }
            return obj;
        },
        base::Node::Nonterminal(name, nodes) => {
            let obj = Object::new();
            let children: Vec<Object> = nodes.iter().cloned().map(tree).collect();
            let children_array: Array = children.into_iter().map(JsValue::from).collect();
            unsafe {
                Reflect::set(&obj, &"name".into(), &name.into())
                    .unwrap();
                Reflect::set(&obj, &"children".into(), &children_array.into())
                        .unwrap();
            }
            return obj;
        }
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if the code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use crate::core::error::Error;
use ebnf_parser_parser as ebnf;
use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;
use crate::core::tree;

mod core;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct EbnfParserParser {
    parser: ebnf::Parser,
}

#[wasm_bindgen]
impl EbnfParserParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<EbnfParserParser, JsValue> {
        match ebnf::parse(input) {
            Ok(parser_parser) => Ok(EbnfParserParser {
                parser: parser_parser,
            }),
            Err(e) => Err(Error::from(e).into()),
        }
    }

    #[wasm_bindgen(getter = productionRules)]
    pub fn get_production_rules(&self) -> Array {
        ebnf::get_production_rules(&self.parser)
            .iter()
            .map(JsValue::from)
            .collect()
    }

    pub fn check(&self, input: &str, initial_rule: &str) -> Option<Object> {
        ebnf::check(input, &self.parser, initial_rule).map(tree)
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if the code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use crate::core::error::Error;
use ebnf_parser_parser as ebnf;
use wasm_bindgen::prelude::*;

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

    pub fn check(&self, input: &str) -> bool {
        ebnf::check(input, &self.parser)
    }
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if the code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

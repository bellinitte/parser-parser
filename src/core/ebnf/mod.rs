use wasm_bindgen::prelude::*;

mod ast;
mod builder;
pub mod error;
mod parser;

#[wasm_bindgen]
pub struct EbnfParserParser {
    parser: Box<dyn Fn(&str) -> bool>,
}

#[wasm_bindgen]
impl EbnfParserParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<EbnfParserParser, JsValue> {
        use nom::bytes::complete::tag;

        let _ = match parser::parse(&input) {
            Ok(ast) => ast,
            Err(err) => return Err(err.into()),
        };

        return Ok(EbnfParserParser {
            parser: Box::new(|input: &str| -> bool {
                tag::<&str, &str, ()>("test")(input).is_ok()
            }),
        });
    }

    pub fn check(&self, input: &str) -> bool {
        (*self.parser)(input)
    }
}

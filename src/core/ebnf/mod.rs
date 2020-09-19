use error::Error;
use wasm_bindgen::prelude::*;

mod builder;
pub mod error;
mod lexer;
mod parser;
mod scanner;

#[wasm_bindgen]
pub struct EbnfParserParser {
    parser: Box<dyn Fn(&str) -> bool>,
}

#[wasm_bindgen]
impl EbnfParserParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<EbnfParserParser, JsValue> {
        match EbnfParserParser::generate(input) {
            Ok(parser_parser) => Ok(parser_parser),
            Err(e) => Err(e.into()),
        }
    }

    fn generate(input: &str) -> Result<EbnfParserParser, Error> {
        let symbols: Vec<scanner::Symbol> = scanner::scan(input)?;
        let tokens: Vec<lexer::Token> = lexer::lex(&symbols)?;
        let ast: parser::Grammar = parser::parse(&tokens)?;
        let parser = builder::build(&ast)?;
        Ok(EbnfParserParser { parser })
    }

    pub fn check(&self, input: &str) -> bool {
        (*self.parser)(input)
    }
}

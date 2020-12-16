#![feature(box_syntax, box_patterns, bindings_after_at)]

mod compiler;
pub mod error;
mod lexer;
mod parser;
mod preprocessor;
mod span;

use error::Error;

pub fn parse(input: &str) -> Result<base::Grammar, Error> {
    let tokens = lexer::lex(input)?;
    let ast = parser::parse(&tokens)?;
    let ast = preprocessor::preprocess(ast)?;
    let ast = compiler::compile(ast);
    Ok(ast)
}

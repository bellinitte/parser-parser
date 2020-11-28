#![feature(box_syntax, box_patterns)]

use error::Error;

mod checker;
pub mod error;
mod lexer;
mod parser;
mod preprocessor;
mod span;

use parser::Grammar;
use span::Spanned;

pub struct Parser {
    ast: Spanned<Grammar>,
}

pub fn parse(input: &str) -> Result<Parser, Error> {
    let tokens = lexer::lex(input)?;
    let ast = parser::parse(&tokens)?;
    let ast = preprocessor::preprocess(ast)?;
    Ok(Parser { ast })
}

pub fn check(input: &str, parser: &Parser, starting_rule: &str) -> bool {
    checker::check(input, &parser.ast, starting_rule)
}

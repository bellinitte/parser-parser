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
    starting_rule: String,
}

pub fn parse(input: &str) -> Result<Parser, Error> {
    let tokens = lexer::lex(input)?;
    let ast = parser::parse(&tokens)?;
    let (ast, starting_rule) = preprocessor::preprocess(ast)?;
    Ok(Parser { ast, starting_rule })
}

pub fn check(input: &str, parser: &Parser) -> bool {
    checker::check(input, &parser.ast, &parser.starting_rule)
}

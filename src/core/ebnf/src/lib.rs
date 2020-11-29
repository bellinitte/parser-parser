#![feature(box_syntax, box_patterns, bindings_after_at)]

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

// pub fn get_production_rules(parser: &Parser) -> Vec<String> {
//     match &parser.ast {
//         Spanned { node: grammar, .. } => {
//             return Vec::new();
//         }
//     }
// }

pub fn get_production_rules(
    Parser {
        ast: Spanned {
            node: Grammar { productions },
            ..
        },
    }: &Parser,
) -> Vec<String> {
    productions
        .iter()
        .map(|spanned_production| spanned_production.node.lhs.node.clone())
        .collect()
}

pub fn check(input: &str, parser: &Parser, initial_rule: &str) -> bool {
    checker::check(input, &parser.ast, initial_rule)
}

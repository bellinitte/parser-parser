#![feature(box_syntax, box_patterns, bindings_after_at)]

// use crate::error::Error;

pub mod ast;
pub mod checker;
mod error;

pub use ast::{Grammar, Expression};
pub use checker::Node;
pub use checker::check;

// pub struct EbnfParserParser {
//     parser: ebnf::Parser,
// }

// impl EbnfParserParser {
//     pub fn new(input: &str) -> Result<EbnfParserParser, Error> {
//         match ebnf::parse(input) {
//             Ok(parser_parser) => Ok(EbnfParserParser {
//                 parser: parser_parser,
//             }),
//             Err(e) => Err(Error::Ebnf(e)),
//         }
//     }

//     pub fn get_production_rules(&self) -> Array {
//         ebnf::get_production_rules(&self.parser)
//     }

//     pub fn check(&self, input: &str, initial_rule: &str) -> Option<Object> {
//         ebnf::check(input, &self.parser, initial_rule)
//     }
// }

// pub fn check(input: &str, parser: &Parser, initial_rule: &str) -> Option<Node> {
//     checker::check(input, &parser.ast, initial_rule)
// }

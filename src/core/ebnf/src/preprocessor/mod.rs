pub mod error;

use super::error::Span;
use super::parser::Grammar;
use error::Error;
use std::collections::HashSet;

fn generate_alias(input: &str, values: &HashSet<String>) -> String {
    let mut current = input.to_owned();
    loop {
        current.push('\'');
        if !values.contains(&current) {
            return current;
        }
    }
}

fn get_defined_nonterminals(grammar: &Grammar) -> HashSet<String> {
    grammar
        .productions
        .keys()
        .cloned()
        .collect::<HashSet<String>>()
}

pub(super) fn preprocess<'a>(input: Grammar) -> Result<Grammar, Error> {
    let defined_nonterminals = get_defined_nonterminals(&input);

    return Ok(input);
}

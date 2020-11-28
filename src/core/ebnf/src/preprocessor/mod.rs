pub mod error;

use super::parser::Grammar;
use super::span::{Span, Spanned, Spanning};
use error::Error;
// use std::collections::HashSet;

// fn get_defined_nonterminals(grammar: &Grammar) -> HashSet<String> {
//     grammar
//         .productions
//         .iter()
//         .map(|spanned| spanned.node.lhs.node.clone())
//         .collect::<HashSet<String>>()
// }

pub(super) fn preprocess(
    input: Spanned<Grammar>,
) -> Result<(Spanned<Grammar>, String), Spanned<Error>> {
    Ok((input, "expression".to_owned()))
}

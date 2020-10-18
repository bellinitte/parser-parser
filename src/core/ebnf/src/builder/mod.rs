pub mod error;

use super::parser::Grammar;
use super::span::{Span, Spanned, Spanning};
use error::Error;

pub(super) fn build<'a>(
    _input: Spanned<Grammar>,
) -> Result<Box<dyn Fn(&str) -> bool>, Spanned<Error>> {
    use nom::bytes::complete::tag;

    Ok(Box::new(|input: &str| -> bool {
        tag::<&str, &str, ()>("test")(input).is_ok()
    }))
}

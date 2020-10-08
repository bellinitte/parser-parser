pub mod error;

use super::error::Span;
use super::parser::Grammar;
use error::Error;

pub(super) fn build<'a>(_input: Grammar) -> Result<Box<dyn Fn(&str) -> bool>, Error> {
    use nom::bytes::complete::tag;

    Ok(Box::new(|input: &str| -> bool {
        tag::<&str, &str, ()>("test")(input).is_ok()
    }))
}

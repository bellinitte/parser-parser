pub mod error;

use super::parser::ast::Grammar;
use error::Error;

pub(super) fn build<'a>(input: &Grammar) -> Result<Box<dyn Fn(&str) -> bool>, Error> {
    use nom::bytes::complete::tag;

    Ok(Box::new(|input: &str| -> bool {
        tag::<&str, &str, ()>("test")(input).is_ok()
    }))
}

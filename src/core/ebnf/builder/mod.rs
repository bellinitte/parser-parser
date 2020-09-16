use super::ast::Grammar;
use super::error::Result;

pub(super) fn build<'a>(input: &Grammar) -> Result<Box<dyn Fn(&str) -> bool>> {
    todo!()
}

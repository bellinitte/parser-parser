use error::Error;

mod builder;
pub mod error;
mod lexer;
mod parser;
mod span;

pub fn construct(input: &str) -> Result<Box<dyn Fn(&str) -> bool>, Error> {
    let tokens = lexer::lex(input)?;
    let ast = parser::parse(&tokens)?;
    let parser = builder::build(ast)?;
    Ok(parser)
}

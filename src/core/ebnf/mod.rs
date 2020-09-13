use wasm_bindgen::prelude::*;

mod builder;
mod parser;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grammar {
    productions: Vec<Production>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Production {
    lhs: String,
    rhs: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Expression {
    Alternative {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
    },
    Sequence {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
    },
    Optional(Box<Expression>),
    Repeated(Box<Expression>),
    Factor {
        count: usize,
        primary: Box<Expression>
    },
    Exception {
        subject: Box<Expression>,
        restriction: Box<Expression>
    },
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
}

#[wasm_bindgen]
pub struct EbnfParser {
    parser: Box<dyn Fn(&str) -> bool>,
}

#[wasm_bindgen]
impl EbnfParser {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<EbnfParser, JsValue> {
        use nom::bytes::complete::tag;

        // let (_, ast) = parser::parse::<()>(&input)?;
        return Ok(EbnfParser {
            parser: Box::new(|input: &str| -> bool {
                tag::<&str, &str, ()>("test")(input).is_ok()
            })
        });
    }

    pub fn check(&self, input: &str) -> bool {
        (*self.parser)(input)
    }
}

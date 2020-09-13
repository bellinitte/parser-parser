mod builder;
pub mod parser;

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

pub fn parse(input: &str) -> String {
    use parser::syntax;

    format!("{:?}", syntax::<()>(&input).ok().map(|(_, grammar)| grammar))
}

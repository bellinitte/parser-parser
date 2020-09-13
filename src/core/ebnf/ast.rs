#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    pub productions: Vec<Production>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub lhs: String,
    pub rhs: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
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

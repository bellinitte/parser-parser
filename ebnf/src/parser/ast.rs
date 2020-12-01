use super::{Span, Spanned, Spanning};
use crate::impl_spanning;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    pub productions: Vec<Spanned<Production>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub lhs: Spanned<String>,
    pub rhs: Spanned<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Alternative {
        first: Box<Spanned<Expression>>,
        second: Box<Spanned<Expression>>,
        rest: Vec<Spanned<Expression>>,
    },
    Sequence {
        first: Box<Spanned<Expression>>,
        second: Box<Spanned<Expression>>,
        rest: Vec<Spanned<Expression>>,
    },
    Optional(Box<Spanned<Expression>>),
    Repeated(Box<Spanned<Expression>>),
    Factor {
        count: Spanned<usize>,
        primary: Box<Spanned<Expression>>,
    },
    Exception {
        subject: Box<Spanned<Expression>>,
        restriction: Box<Spanned<Expression>>,
    },
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
}

impl_spanning!(Grammar);
impl_spanning!(Production);
impl_spanning!(Expression);

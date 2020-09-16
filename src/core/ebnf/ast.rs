use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    pub productions: Vec<Token<Production>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub lhs: Token<String>,
    pub rhs: Token<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Alternative {
        first: Box<Token<Expression>>,
        second: Box<Token<Expression>>,
        rest: Vec<Token<Expression>>,
    },
    Sequence {
        first: Box<Token<Expression>>,
        second: Box<Token<Expression>>,
        rest: Vec<Token<Expression>>,
    },
    Optional(Box<Token<Expression>>),
    Repeated(Box<Token<Expression>>),
    Factor {
        count: Token<usize>,
        primary: Box<Token<Expression>>,
    },
    Exception {
        subject: Box<Token<Expression>>,
        restriction: Box<Token<Expression>>,
    },
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Position {
    range: Range<usize>,
}

impl Position {
    pub fn new(range: Range<usize>) -> Position {
        Position { range: range }
    }
}

impl From<Range<usize>> for Position {
    fn from(range: Range<usize>) -> Position {
        Position { range: range }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<T> {
    pub inner: T,
    pub position: Position,
}

impl<T> Token<T> {
    pub fn new(inner: T, position: Position) -> Token<T> {
        Token {
            inner: inner,
            position: position,
        }
    }
}

pub trait TokenAt
where
    Self: Sized,
{
    fn token_at(self, range: Range<usize>) -> Token<Self>;
}

#[macro_export]
macro_rules! impl_token_at {
    ($impl_type:ty) => {
        impl TokenAt for $impl_type {
            fn token_at(self, range: Range<usize>) -> Token<$impl_type> {
                Token {
                    inner: self,
                    position: range.into(),
                }
            }
        }
    };
}

impl_token_at!(Grammar);
impl_token_at!(Production);
impl_token_at!(Expression);
impl_token_at!(String);
impl_token_at!(usize);

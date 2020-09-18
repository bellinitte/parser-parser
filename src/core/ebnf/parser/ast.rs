use std::ops::Range;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    pub productions: Vec<Node<Production>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub lhs: Node<String>,
    pub rhs: Node<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Alternative {
        first: Box<Node<Expression>>,
        second: Box<Node<Expression>>,
        rest: Vec<Node<Expression>>,
    },
    Sequence {
        first: Box<Node<Expression>>,
        second: Box<Node<Expression>>,
        rest: Vec<Node<Expression>>,
    },
    Optional(Box<Node<Expression>>),
    Repeated(Box<Node<Expression>>),
    Factor {
        count: Node<usize>,
        primary: Box<Node<Expression>>,
    },
    Exception {
        subject: Box<Node<Expression>>,
        restriction: Box<Node<Expression>>,
    },
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Node<T> {
    pub inner: T,
    pub span: Option<Range<usize>>,
}

impl<T> Node<T> {
    pub fn new(inner: T, span: Option<Range<usize>>) -> Node<T> {
        Node { inner, span }
    }
}

pub trait NodeAt
where
    Self: Sized,
{
    fn node_at(self, range: Range<usize>) -> Node<Self>;
}

#[macro_export]
macro_rules! impl_node_at {
    ($impl_type:ty) => {
        impl NodeAt for $impl_type {
            fn node_at(self, span: Range<usize>) -> Node<$impl_type> {
                Node {
                    inner: self,
                    span: Some(span),
                }
            }
        }
    };
}

impl_node_at!(Grammar);
impl_node_at!(Production);
impl_node_at!(Expression);
impl_node_at!(String);
impl_node_at!(usize);

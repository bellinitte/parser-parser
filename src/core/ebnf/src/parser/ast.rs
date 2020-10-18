use super::Span;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    pub productions: HashMap<String, Vec<Production>>,
    pub span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub expression: Expression,
    pub production_span: Span,
    pub identifier_span: Span,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Alternative {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
        span: Span,
    },
    Sequence {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
        span: Span,
    },
    Optional {
        inner: Box<Expression>,
        span: Span,
    },
    Repeated {
        inner: Box<Expression>,
        span: Span,
    },
    Factor {
        count: usize,
        primary: Box<Expression>,
        span: Span,
    },
    Exception {
        subject: Box<Expression>,
        restriction: Box<Expression>,
        span: Span,
    },
    Nonterminal {
        identifier: String,
        span: Span,
    },
    Terminal {
        content: String,
        span: Span,
    },
    Special {
        content: String,
        span: Span,
    },
    Empty {
        span: Span,
    },
}

impl Expression {
    pub fn span(&self) -> &Span {
        match self {
            Expression::Alternative { span, .. } => span,
            Expression::Sequence { span, .. } => span,
            Expression::Optional { span, .. } => span,
            Expression::Repeated { span, .. } => span,
            Expression::Factor { span, .. } => span,
            Expression::Exception { span, .. } => span,
            Expression::Nonterminal { span, .. } => span,
            Expression::Terminal { span, .. } => span,
            Expression::Special { span, .. } => span,
            Expression::Empty { span } => span,
        }
    }

    pub fn with_span(self, span: Span) -> Expression {
        match self {
            Expression::Alternative {
                first,
                second,
                rest,
                ..
            } => Expression::Alternative {
                first,
                second,
                rest,
                span,
            },
            Expression::Sequence {
                first,
                second,
                rest,
                ..
            } => Expression::Sequence {
                first,
                second,
                rest,
                span,
            },
            Expression::Optional { inner, .. } => Expression::Optional { inner, span },
            Expression::Repeated { inner, .. } => Expression::Repeated { inner, span },
            Expression::Factor { count, primary, .. } => Expression::Factor {
                count,
                primary,
                span,
            },
            Expression::Exception {
                subject,
                restriction,
                ..
            } => Expression::Exception {
                subject,
                restriction,
                span,
            },
            Expression::Nonterminal { identifier, .. } => {
                Expression::Nonterminal { identifier, span }
            }
            Expression::Terminal { content, .. } => Expression::Terminal { content, span },
            Expression::Special { content, .. } => Expression::Special { content, span },
            Expression::Empty { .. } => Expression::Empty { span },
        }
    }
}

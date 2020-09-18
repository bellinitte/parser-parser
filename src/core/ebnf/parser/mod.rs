// TODO decide what to do about the fact that a terminal can contain a newline vs a space.

use super::lexer::{Token, TokenKind};
use ast::NodeAt;
pub use ast::{Expression, Grammar, Node, Production};
use error::Error;
use nom::Err;
use nom::{error::ErrorKind, IResult, InputTakeAtPosition};
use nom::{InputIter, Slice};
use std::ops::{Range, RangeFrom};
use tokens::Tokens;

pub mod ast;
pub mod error;
#[cfg(test)]
mod tests;
mod tokens;

fn literal(c: TokenKind) -> impl Fn(Tokens) -> IResult<Tokens, TokenKind, Error> {
    move |i: Tokens| match i.iter_elements().next().map(|t| {
        let b = t.kind == c;
        (t.kind, b)
    }) {
        Some((c, true)) => Ok((i.slice(1..), c)),
        _ => Err(Err::Error(Error::InvalidToken)),
    }
}

fn nonterminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Nonterminal(s).node_at(span))),
        _ => Err(Err::Error(Error::NonterminalExpected)),
    }
}

fn terminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Terminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Terminal(s).node_at(span))),
        _ => Err(Err::Error(Error::TerminalExpected)),
    }
}

fn special(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Special(s),
            span,
        }) => Ok((i.slice(1..), Expression::Special(s).node_at(span))),
        _ => Err(Err::Error(Error::SpecialExpected)),
    }
}

fn integer(i: Tokens) -> IResult<Tokens, Node<usize>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Integer(s),
            span,
        }) => Ok((i.slice(1..), s.node_at(span))),
        _ => Err(Err::Error(Error::IntegerExpected)),
    }
}

fn grouped(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((
            literal(TokenKind::StartGroup),
            alternative,
            literal(TokenKind::EndGroup),
        )),
        |(open, node, close)| node.inner.node_at(0..0),
    )(i)
}

fn repeated(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((
            literal(TokenKind::StartRepeat),
            alternative,
            literal(TokenKind::EndRepeat),
        )),
        |(open, node, close)| Expression::Repeated(Box::new(node)).node_at(0..0),
    )(i)
}

fn optional(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((
            literal(TokenKind::StartOption),
            alternative,
            literal(TokenKind::EndOption),
        )),
        |(open, node, close)| Expression::Optional(Box::new(node)).node_at(0..0),
    )(i)
}

fn factor(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{
        branch::alt,
        character::complete::char,
        combinator::map,
        combinator::opt,
        sequence::{pair, terminated},
    };

    map(
        pair(
            opt(pair(integer, literal(TokenKind::Repetition))),
            opt(alt((
                optional,
                repeated,
                grouped,
                nonterminal,
                terminal,
                special,
            ))),
        ),
        |(repetition, node)| match (repetition, node) {
            (Some((count @ Node { inner: 0, .. }, asterisk)), _) => {
                Node::new(Expression::Empty, None)
            }
            (Some((count, asterisk)), Some(node)) => {
                let span = count.span.clone().unwrap().start..node.span.clone().unwrap().end;
                Expression::Factor {
                    count,
                    primary: Box::new(node),
                }
                .node_at(span)
            },
            (None, Some(node)) => node,
            (_, None) => Node::new(Expression::Empty, None),
        },
    )(i)
}

fn term(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{
        character::complete::char,
        combinator::map,
        combinator::opt,
        sequence::preceded,
        sequence::{pair, terminated},
    };

    map(
        pair(factor, opt(preceded(literal(TokenKind::Exception), factor))),
        |(primary, exception)| match exception {
            None => primary,
            Some(ex) => Expression::Exception {
                subject: Box::new(primary),
                restriction: Box::new(ex),
            }
            .node_at(0..0),
        },
    )(i)
}

fn sequence(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{character::complete::char, combinator::map, multi::separated_list1};

    map(
        separated_list1(literal(TokenKind::Concatenation), term),
        |nodes| match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Sequence {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
            }
            .node_at(0..0),
        },
    )(i)
}

fn alternative(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{character::complete::one_of, combinator::map, multi::separated_list1};

    map(
        separated_list1(literal(TokenKind::DefinitionSeparator), sequence),
        |nodes| match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Alternative {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
            }
            .node_at(0..0),
        },
    )(i)
}

fn production(i: Tokens) -> IResult<Tokens, Node<Production>, Error> {
    use nom::{
        character::complete::{char, one_of},
        combinator::map,
        sequence::{pair, separated_pair, terminated},
    };

    map(
        terminated(
            separated_pair(nonterminal, literal(TokenKind::Definition), alternative),
            literal(TokenKind::Terminator),
        ),
        |(identifier, definitions)| match identifier {
            Node {
                inner: Expression::Nonterminal(s),
                span,
            } => Production {
                lhs: Node::new(s, span),
                rhs: definitions,
            }
            .node_at(0..0),
            _ => unreachable!(),
        },
    )(i)
}

fn syntax(i: Tokens) -> IResult<Tokens, Node<Grammar>, Error> {
    use nom::{combinator::map, multi::many1};

    map(many1(production), |productions| {
        Grammar { productions }.node_at(0..0)
    })(i)
}

pub(super) fn parse<'a>(input: &[Token]) -> Result<Grammar, Error> {
    match syntax(Tokens::new(input)) {
        Ok((_, grammar)) => Ok(grammar.inner),
        Err(nom::Err::Failure(inner)) => Err(inner.into()),
        Err(nom::Err::Error(inner)) => Err(inner.into()),
        _ => unreachable!(),
    }
}

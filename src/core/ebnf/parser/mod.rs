use super::lexer::{Token, TokenKind};
pub use ast::{Expression, Grammar, Node, NodeAt, Production};
use error::{Error, ErrorKind};
use nom::{
    branch::alt,
    combinator::{cut, map, opt},
    multi::{many1, separated_list1},
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use tokens::*;
use utils::*;

pub mod ast;
pub mod error;
#[cfg(test)]
mod tests;
mod tokens;
mod utils;

fn grouped(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        tuple((start_group_symbol, alternative, cut(end_group_symbol))),
        |(open, node, close)| node.inner.node_at(open.span.start..close.span.end),
    )(i)
}

fn repeated(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        tuple((start_repeat_symbol, alternative, cut(end_repeat_symbol))),
        |(open, node, close)| {
            Expression::Repeated(Box::new(node)).node_at(open.span.start..close.span.end)
        },
    )(i)
}

fn optional(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        tuple((start_option_symbol, alternative, cut(end_option_symbol))),
        |(open, node, close)| {
            Expression::Optional(Box::new(node)).node_at(open.span.start..close.span.end)
        },
    )(i)
}

fn factor(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        pair(
            opt(terminated(integer, cut(repetition_symbol))),
            alt((
                optional,
                repeated,
                grouped,
                nonterminal,
                terminal,
                special,
                empty,
            )),
        ),
        |(repetition, node)| match (repetition, node) {
            (Some(count @ Node { inner: 0, .. }), node) => {
                let span = count.span.start..node.span.end;
                Expression::Empty.node_at(span)
            }
            (Some(count), node) => {
                let span = count.span.start..node.span.end;
                Expression::Factor {
                    count,
                    primary: Box::new(node),
                }
                .node_at(span)
            }
            (None, node) => node,
        },
    )(i)
}

fn term(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        pair(factor, opt(preceded(exception_symbol, cut(factor)))),
        |(primary, exception)| match exception {
            None => primary,
            Some(ex) => {
                let span = primary.span.start..ex.span.end;
                Expression::Exception {
                    subject: Box::new(primary),
                    restriction: Box::new(ex),
                }
                .node_at(span)
            }
        },
    )(i)
}

fn sequence(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map(
        separated_list1(concatenation_symbol, term),
        |nodes| match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Sequence {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
            }
            .node_at(nodes[0].span.start..nodes[nodes.len() - 1].span.end),
        },
    )(i)
}

fn alternative(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    map_err(
        map(
            separated_list1(definition_separator, sequence),
            |nodes| match nodes.len() {
                1 => nodes[0].clone(),
                _ => Expression::Alternative {
                    first: Box::new(nodes[0].clone()),
                    second: Box::new(nodes[1].clone()),
                    rest: nodes[2..].to_vec(),
                }
                .node_at(nodes[0].span.start..nodes[nodes.len() - 1].span.end),
            },
        ),
        |e| match e {
            Error {
                kind: ErrorKind::Nom(nom::error::ErrorKind::SeparatedList),
                position,
            } => Error {
                kind: ErrorKind::DefinitionExpected,
                position,
            },
            e => e,
        },
    )(i)
}

fn production(i: Tokens) -> IResult<Tokens, Node<Production>, Error> {
    map(
        pair(
            separated_pair(identifier, cut(definition_symbol), alternative),
            cut(terminator_symbol),
        ),
        |((identifier, definitions), terminator)| {
            let span = identifier.span.start..terminator.span.end;
            Production {
                lhs: Node::new(identifier.inner, identifier.span),
                rhs: definitions,
            }
            .node_at(span)
        },
    )(i)
}

fn syntax(i: Tokens) -> IResult<Tokens, Node<Grammar>, Error> {
    map_err(
        map(cut(many1(production)), |productions| {
            let span = productions[0].span.start..productions[productions.len() - 1].span.end;
            Grammar { productions }.node_at(span)
        }),
        |e| match e {
            Error {
                kind: ErrorKind::Nom(nom::error::ErrorKind::Many1),
                position,
            } => Error {
                kind: ErrorKind::AtLeastOneProductionRuleRequired,
                position,
            },
            e => e,
        },
    )(i)
}

pub(super) fn parse<'a>(tokens: &'a [Token]) -> Result<Grammar, Error> {
    match syntax(Tokens::new(&tokens)) {
        Ok((_, grammar)) => Ok(grammar.inner),
        Err(nom::Err::Failure(inner)) => Err(inner.into()),
        Err(nom::Err::Error(inner)) => Err(inner.into()),
        _ => unreachable!(),
    }
}

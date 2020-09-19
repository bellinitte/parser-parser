// TODO decide what to do about the fact that a terminal can contain a newline vs a space.

use super::lexer::{Token, TokenKind};
use ast::NodeAt;
pub use ast::{Expression, Grammar, Node, Production};
use error::Error;
use nom::IResult;
use tokens::*;

pub mod ast;
pub mod error;
#[cfg(test)]
mod tests;
mod tokens;

fn grouped(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((start_group, alternative, end_group)),
        |(open, node, close)| node.inner.node_at(open.span.start..close.span.end),
    )(i)
}

fn repeated(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((start_repeat, alternative, end_repeat)),
        |(open, node, close)| {
            Expression::Repeated(Box::new(node)).node_at(open.span.start..close.span.end)
        },
    )(i)
}

fn optional(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, sequence::tuple};

    map(
        tuple((start_option, alternative, end_option)),
        |(open, node, close)| {
            Expression::Optional(Box::new(node)).node_at(open.span.start..close.span.end)
        },
    )(i)
}

fn factor(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{
        branch::alt,
        combinator::map,
        combinator::opt,
        sequence::{pair, terminated},
    };

    map(
        pair(
            opt(terminated(integer, repetition)),
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
    use nom::{combinator::map, combinator::opt, sequence::pair, sequence::preceded};

    map(
        pair(factor, opt(preceded(exception, factor))),
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
    use nom::{combinator::map, multi::separated_list1};

    map(separated_list1(concatenation, term), |nodes| {
        match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Sequence {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
            }
            .node_at(nodes[0].span.start..nodes[nodes.len() - 1].span.end),
        }
    })(i)
}

fn alternative(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    use nom::{combinator::map, multi::separated_list1};

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
    )(i)
}

fn production(i: Tokens) -> IResult<Tokens, Node<Production>, Error> {
    use nom::{
        combinator::map,
        sequence::{pair, separated_pair},
    };

    map(
        pair(
            separated_pair(identifier, definition, alternative),
            terminator,
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
    use nom::{combinator::map, multi::many1};

    map(many1(production), |productions| {
        let span = productions[0].span.start..productions[productions.len() - 1].span.end;
        Grammar { productions }.node_at(span)
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

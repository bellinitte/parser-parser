use super::lexer::Token;
use super::span::{Span, Spanned, Spanning};
pub use ast::{Expression, Grammar, Production};
use error::Error;
use nom::{
    branch::alt,
    combinator::{cut, map, opt},
    multi::many1,
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

fn grouped(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    map(
        tuple((start_group_symbol, alternative, cut(end_group_symbol))),
        |(open, expr, close)| expr.node.spanning(Span::combine(&open.span, &close.span)),
    )(i)
}

fn repeated(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    map(
        tuple((start_repeat_symbol, alternative, cut(end_repeat_symbol))),
        |(open, node, close)| {
            Expression::Repeated(Box::new(node)).spanning(Span::combine(&open.span, &close.span))
        },
    )(i)
}

fn optional(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    map(
        tuple((start_option_symbol, alternative, cut(end_option_symbol))),
        |(open, node, close)| {
            Expression::Optional(Box::new(node)).spanning(Span::combine(&open.span, &close.span))
        },
    )(i)
}

fn factor(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
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
            (Some(count @ Spanned { node: 0, .. }), node) => {
                let span = Span::combine(&count.span, &node.span);
                Expression::Empty.spanning(span)
            }
            (Some(count), node) => {
                let span = Span::combine(&count.span, &node.span);
                Expression::Factor {
                    count,
                    primary: Box::new(node),
                }
                .spanning(span)
            }
            (None, node) => node,
        },
    )(i)
}

fn term(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    map(
        pair(factor, opt(preceded(exception_symbol, cut(factor)))),
        |(primary, exception)| match exception {
            None => primary,
            Some(ex) => {
                let span = Span::combine(&primary.span, &ex.span);
                Expression::Exception {
                    subject: Box::new(primary),
                    restriction: Box::new(ex),
                }
                .spanning(span)
            }
        },
    )(i)
}

fn sequence(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    map(
        separated_list1(concatenation_symbol, term),
        |nodes| match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Sequence {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
            }
            .spanning(Span::combine(&nodes[0].span, &nodes[nodes.len() - 1].span)),
        },
    )(i)
}

fn alternative(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
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
                .spanning(Span::combine(&nodes[0].span, &nodes[nodes.len() - 1].span)),
            },
        ),
        |e| match e {
            Spanned {
                node: Error::Nom(nom::error::ErrorKind::SeparatedList),
                span,
            } => Error::DefinitionExpected.spanning(span),
            e => e,
        },
    )(i)
}

fn production(i: Tokens) -> IResult<Tokens, Spanned<Production>, Spanned<Error>> {
    map(
        non_eof(cut(pair(
            separated_pair(identifier, definition_symbol, alternative),
            terminator_symbol,
        ))),
        |((identifier, definitions), terminator)| {
            let span = Span::combine(&identifier.span, &terminator.span);
            Production {
                lhs: identifier,
                rhs: definitions,
            }
            .spanning(span)
        },
    )(i)
}

fn syntax(i: Tokens) -> IResult<Tokens, Spanned<Grammar>, Spanned<Error>> {
    map(many1(production), |productions| {
        let span = Span::combine(
            &productions[0].span,
            &productions[productions.len() - 1].span,
        );
        Grammar { productions }.spanning(span)
    })(i)
}

pub(super) fn parse<'a>(tokens: &'a [Spanned<Token>]) -> Result<Spanned<Grammar>, Spanned<Error>> {
    match syntax(Tokens::new(&tokens)) {
        Ok((_, grammar)) => Ok(grammar),
        Err(nom::Err::Failure(inner)) => Err(inner.into()),
        Err(nom::Err::Error(inner)) => Err(inner.into()),
        _ => unreachable!(),
    }
}

use super::error::Span;
use super::lexer::{Token, TokenKind};
pub use ast::{Expression, Grammar, Production};
use error::{Error, ErrorKind};
use nom::{
    branch::alt,
    combinator::{cut, map, opt},
    multi::many1,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::collections::HashMap;
use tokens::*;
use utils::*;

pub mod ast;
pub mod error;
#[cfg(test)]
mod tests;
mod tokens;
mod utils;

fn grouped(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map(
        tuple((start_group_symbol, alternative, cut(end_group_symbol))),
        |(open, node, close)| node.with_span(Span::combine(&open.1, &close.1)),
    )(i)
}

fn repeated(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map(
        tuple((start_repeat_symbol, alternative, cut(end_repeat_symbol))),
        |(open, expr, close)| Expression::Repeated {
            inner: Box::new(expr),
            span: Span::combine(&open.1, &close.1),
        },
    )(i)
}

fn optional(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map(
        tuple((start_option_symbol, alternative, cut(end_option_symbol))),
        |(open, node, close)| Expression::Optional {
            inner: Box::new(node),
            span: Span::combine(&open.1, &close.1),
        },
    )(i)
}

fn factor(i: Tokens) -> IResult<Tokens, Expression, Error> {
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
            (Some(count @ (0, _)), node) => {
                let span = Span::combine(&count.1, &node.span());
                Expression::Empty { span }
            }
            (Some(count), node) => {
                let span = Span::combine(&count.1, &node.span());
                Expression::Factor {
                    count: count.0,
                    primary: Box::new(node),
                    span,
                }
            }
            (None, node) => node,
        },
    )(i)
}

fn term(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map(
        pair(factor, opt(preceded(exception_symbol, cut(factor)))),
        |(primary, exception)| match exception {
            None => primary,
            Some(ex) => {
                let span = Span::combine(&primary.span(), &ex.span());
                Expression::Exception {
                    subject: Box::new(primary),
                    restriction: Box::new(ex),
                    span,
                }
            }
        },
    )(i)
}

fn sequence(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map(
        separated_list1(concatenation_symbol, term),
        |nodes| match nodes.len() {
            1 => nodes[0].clone(),
            _ => Expression::Sequence {
                first: Box::new(nodes[0].clone()),
                second: Box::new(nodes[1].clone()),
                rest: nodes[2..].to_vec(),
                span: Span::combine(&nodes[0].span(), &nodes[nodes.len() - 1].span()),
            },
        },
    )(i)
}

fn alternative(i: Tokens) -> IResult<Tokens, Expression, Error> {
    map_err(
        map(
            separated_list1(definition_separator, sequence),
            |nodes| match nodes.len() {
                1 => nodes[0].clone(),
                _ => Expression::Alternative {
                    first: Box::new(nodes[0].clone()),
                    second: Box::new(nodes[1].clone()),
                    rest: nodes[2..].to_vec(),
                    span: Span::combine(&nodes[0].span(), &nodes[nodes.len() - 1].span()),
                },
            },
        ),
        |e| match e {
            Error {
                kind: ErrorKind::Nom(nom::error::ErrorKind::SeparatedList),
                span,
            } => Error {
                kind: ErrorKind::DefinitionExpected,
                span,
            },
            e => e,
        },
    )(i)
}

fn production(i: Tokens) -> IResult<Tokens, (String, Production), Error> {
    map(
        non_eof(cut(pair(
            separated_pair(identifier, definition_symbol, alternative),
            terminator_symbol,
        ))),
        |((identifier, definitions), terminator)| {
            let span = Span::combine(&identifier.1, &terminator.1);
            (
                identifier.0,
                Production {
                    expression: definitions,
                    production_span: span,
                    identifier_span: identifier.1,
                },
            )
        },
    )(i)
}

fn syntax(i: Tokens) -> IResult<Tokens, Grammar, Error> {
    map(many1(production), |vec| {
        let span = Span::combine(
            &(vec[0].1).production_span,
            &(vec[vec.len() - 1].1).production_span,
        );
        let mut productions: HashMap<String, Vec<Production>> = HashMap::new();
        for (key, value) in vec.iter() {
            if let Some(x) = productions.get_mut(key) {
                (*x).push(value.clone());
            } else {
                productions.insert(key.clone(), vec![value.clone()]);
            }
        }
        Grammar { productions, span }
    })(i)
}

pub(super) fn parse<'a>(tokens: &'a [Token]) -> Result<Grammar, Error> {
    match syntax(Tokens::new(&tokens)) {
        Ok((_, grammar)) => Ok(grammar),
        Err(nom::Err::Failure(inner)) => Err(inner.into()),
        Err(nom::Err::Error(inner)) => Err(inner.into()),
        _ => unreachable!(),
    }
}

pub mod error;

use super::parser::{Expression, Grammar};
use super::span::{Span, Spanned, Spanning};

fn check_expr<'a>(
    input: &'a str,
    expression: &'a Expression,
    grammar: &'a Spanned<Grammar>,
) -> Result<(&'a str, String), ()> {
    match &expression {
        Expression::Alternative {
            first: box Spanned { node: first, .. },
            second: box Spanned { node: second, .. },
            rest,
        } => {
            match check_expr(input, first, grammar) {
                Ok((rest, output)) => return Ok((rest, output)),
                Err(()) => {}
            }
            match check_expr(input, second, grammar) {
                Ok((rest, output)) => return Ok((rest, output)),
                Err(()) => {}
            }
            for Spanned {
                node: expression, ..
            } in rest.iter()
            {
                match check_expr(input, expression, grammar) {
                    Ok((rest, output)) => return Ok((rest, output)),
                    Err(()) => {}
                }
            }
            Err(())
        }
        Expression::Sequence {
            first: box Spanned { node: first, .. },
            second: box Spanned { node: second, .. },
            rest,
        } => {
            let mut input = input;
            let mut output = String::new();
            let (inp, output_first) = check_expr(input, first, grammar)?;
            input = inp;
            output.push_str(&output_first);
            let (inp, output_second) = check_expr(input, second, grammar)?;
            input = inp;
            output.push_str(&output_second);
            for Spanned {
                node: expression, ..
            } in rest.iter()
            {
                let (inp, output_expr) = check_expr(input, expression, grammar)?;
                input = inp;
                output.push_str(&output_expr);
            }
            Ok((input, output))
        }
        Expression::Optional(box Spanned { node: inner, .. }) => {
            match check_expr(input, inner, grammar) {
                Ok((rest, output)) => Ok((rest, output)),
                Err(()) => Ok((input, String::new())),
            }
        }
        Expression::Repeated(box Spanned { node: inner, .. }) => {
            let mut input = input;
            let mut output = String::new();
            loop {
                match check_expr(input, inner, grammar) {
                    Ok((rest, out)) => {
                        input = rest;
                        output.push_str(&out);
                    }
                    Err(()) => return Ok((input, output)),
                }
            }
        }
        Expression::Factor {
            count: Spanned { node: count, .. },
            primary: box Spanned { node: primary, .. },
        } => {
            let mut input = input;
            let mut output = String::new();
            for _ in 0..*count {
                match check_expr(input, primary, grammar) {
                    Ok((rest, out)) => {
                        input = rest;
                        output.push_str(&out);
                    }
                    Err(()) => return Err(()),
                }
            }
            Ok((input, output))
        }
        Expression::Exception {
            subject: box Spanned { node: subject, .. },
            restriction: box Spanned {
                node: restriction, ..
            },
        } => {
            let (input, read_chars) = check_expr(input, subject, grammar)?;
            match check_expr(&read_chars, restriction, grammar) {
                Ok((_, matched_chars)) => {
                    if matched_chars == read_chars {
                        Err(())
                    } else {
                        Ok((input, read_chars))
                    }
                }
                Err(()) => Ok((input, read_chars)),
            }
        }
        Expression::Nonterminal(identifier) => check_prod(input, grammar, identifier),
        Expression::Terminal(content) => {
            if input.starts_with(content) {
                let len = content.len();
                Ok((&input[len..], content.clone()))
            } else {
                Err(())
            }
        }
        Expression::Special(_) => Err(()),
        Expression::Empty => Ok((input, String::new())),
    }
}

pub(super) fn check_prod<'a>(
    input: &'a str,
    grammar: &'a Spanned<Grammar>,
    initial_rule: &'a str,
) -> Result<(&'a str, String), ()> {
    let productions = &grammar.node.productions;
    for production in productions.iter() {
        let production = &production.node;
        let lhs = &production.lhs.node;
        let rhs = &production.rhs.node;
        if initial_rule == lhs {
            return check_expr(input, rhs, grammar);
        }
    }
    unreachable!()
}

pub(super) fn check<'a>(
    input: &'a str,
    grammar: &'a Spanned<Grammar>,
    initial_rule: &'a str,
) -> bool {
    match check_prod(input, grammar, initial_rule) {
        Ok((input, _)) => input.is_empty(),
        Err(()) => false,
    }
}

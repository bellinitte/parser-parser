use super::parser::{Expression, Grammar, Production};
use super::span::Spanned;

pub fn compile(Spanned { node: grammar, .. }: Spanned<Grammar>) -> base::Grammar {
    grammar
        .productions
        .into_iter()
        .map(
            |Spanned {
                 node:
                     Production {
                         lhs: Spanned { node: name, .. },
                         rhs: expr,
                     },
                 ..
             }: Spanned<Production>|
             -> (String, base::Expression) { (name, aux(expr)) },
        )
        .collect()
}

fn aux(
    Spanned {
        node: expression, ..
    }: Spanned<Expression>,
) -> base::Expression {
    match expression {
        Expression::Alternative {
            first: box first,
            second: box second,
            rest,
        } => base::Expression::Alternative {
            first: Box::new(aux(first)),
            second: Box::new(aux(second)),
            rest: rest.into_iter().map(aux).collect(),
        },
        Expression::Sequence {
            first: box first,
            second: box second,
            rest,
        } => base::Expression::Sequence {
            first: Box::new(aux(first)),
            second: Box::new(aux(second)),
            rest: rest.into_iter().map(aux).collect(),
        },
        Expression::Optional(box inner) => base::Expression::Optional(Box::new(aux(inner))),
        Expression::Repeated(box inner) => base::Expression::Repeated(Box::new(aux(inner))),
        Expression::Factor {
            count: Spanned { node: count, .. },
            primary: box primary,
        } => base::Expression::Factor {
            count,
            primary: Box::new(aux(primary)),
        },
        Expression::Exception {
            subject: box subject,
            restriction: box restriction,
        } => base::Expression::Exception {
            subject: Box::new(aux(subject)),
            restriction: Box::new(aux(restriction)),
        },
        Expression::Nonterminal(identifier) => base::Expression::Nonterminal(identifier),
        Expression::Terminal(content) => base::Expression::Terminal(content),
        Expression::Special(content) => base::Expression::Special(content),
        Expression::Empty => base::Expression::Empty,
    }
}

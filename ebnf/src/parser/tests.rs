use super::super::span::Location;
use super::{Error, Expression, Grammar, Production, Span, Spanned, Spanning, Token, Tokens};
use nom::{Err, Slice};
use quickcheck_macros::quickcheck;

#[macro_export]
macro_rules! ok_case {
    ($parser:expr, $input:expr, $offset:expr, $token:expr) => {
        let input = $input;
        let tokens = Tokens::new(input);
        assert_eq!(
            $parser(tokens.clone()),
            Ok((tokens.slice($offset..), $token))
        );
    };
}

#[macro_export]
macro_rules! error_case {
    ($parser:expr, $input_str:expr, $error:expr) => {
        assert_eq!($parser(Tokens::new($input_str)), Err(Err::Error($error)));
    };
}

#[macro_export]
macro_rules! failure_case {
    ($parser:expr, $input_str:expr, $error:expr) => {
        assert_eq!($parser(Tokens::new($input_str)), Err(Err::Failure($error)));
    };
}

#[test]
fn test_factors() {
    use super::factor;

    ok_case!(
        factor,
        &vec![Token::Terminal("terminal".to_owned()).spanning(Span::from(((1, 0), (11, 0))))],
        1,
        Expression::Terminal("terminal".to_owned()).spanning(Span::from(((1, 0), (11, 0))))
    );
    ok_case!(
        factor,
        &vec![Token::Nonterminal("nonterminal".to_owned()).spanning(Span::from(((0, 0), (13, 0))))],
        1,
        Expression::Nonterminal("nonterminal".to_owned()).spanning(Span::from(((0, 0), (13, 0))))
    );
    ok_case!(
        factor,
        &vec![Token::Special(" special ".to_owned()).spanning(Span::from(((2, 0), (13, 0))))],
        1,
        Expression::Special(" special ".to_owned()).spanning(Span::from(((2, 0), (13, 0))))
    );
    ok_case!(
        factor,
        &vec![],
        0,
        Expression::Empty.spanning(Span::from(((0, 0), (0, 0))))
    );
    ok_case!(
        factor,
        &vec![
            Token::Integer(2).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Repetition.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Terminal("terminal".to_owned()).spanning(Span::from(((4, 0), (14, 0))))
        ],
        3,
        Expression::Factor {
            count: 2.spanning(Span::from(((0, 0), (1, 0)))),
            primary: Box::new(
                Expression::Terminal("terminal".to_owned()).spanning(Span::from(((4, 0), (14, 0))))
            )
        }
        .spanning(Span::from(((0, 0), (14, 0))))
    );
    failure_case!(
        factor,
        &vec![
            Token::Integer(2).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Terminal("terminal".to_owned()).spanning(Span::from(((2, 0), (12, 0))))
        ],
        Error::RepetitionSymbolExpected.spanning(Span::from(((2, 0), (12, 0))))
    );
}

#[test]
fn test_terms() {
    use super::term;

    ok_case!(
        term,
        &vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0)))),
            Token::Exception.spanning(Span::from(((4, 0), (5, 0)))),
            Token::Terminal("test".to_owned()).spanning(Span::from(((6, 0), (12, 0))))
        ],
        3,
        Expression::Exception {
            subject: Box::new(
                Expression::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0))))
            ),
            restriction: Box::new(
                Expression::Terminal("test".to_owned()).spanning(Span::from(((6, 0), (12, 0))))
            ),
        }
        .spanning(Span::from(((0, 0), (12, 0))))
    );
    ok_case!(
        term,
        &vec![
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Exception.spanning(Span::from(((1, 0), (2, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((2, 0), (3, 0)))),
            Token::Exception.spanning(Span::from(((3, 0), (4, 0)))),
            Token::Nonterminal("c".to_owned()).spanning(Span::from(((4, 0), (5, 0))))
        ],
        3,
        Expression::Exception {
            subject: Box::new(
                Expression::Nonterminal("a".to_owned()).spanning(Span::from(((0, 0), (1, 0))))
            ),
            restriction: Box::new(
                Expression::Nonterminal("b".to_owned()).spanning(Span::from(((2, 0), (3, 0))))
            ),
        }
        .spanning(Span::from(((0, 0), (3, 0))))
    );
    ok_case!(
        term,
        &vec![],
        0,
        Expression::Empty.spanning(Span::from(((0, 0), (0, 0))))
    );
}

#[test]
fn test_sequences() {
    use super::sequence;

    ok_case!(
        sequence,
        &vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0)))),
            Token::Concatenation.spanning(Span::from(((3, 0), (4, 0)))),
            Token::Terminal("test".to_owned()).spanning(Span::from(((5, 0), (11, 0)))),
            Token::Concatenation.spanning(Span::from(((11, 0), (12, 0)))),
            Token::Nonterminal("bca".to_owned()).spanning(Span::from(((14, 0), (17, 0))))
        ],
        5,
        Expression::Sequence {
            first: Box::new(
                Expression::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0))))
            ),
            second: Box::new(
                Expression::Terminal("test".to_owned()).spanning(Span::from(((5, 0), (11, 0))))
            ),
            rest: vec![
                Expression::Nonterminal("bca".to_owned()).spanning(Span::from(((14, 0), (17, 0))))
            ]
        }
        .spanning(Span::from(((0, 0), (17, 0))))
    );
    ok_case!(
        sequence,
        &vec![],
        0,
        Expression::Empty.spanning(Span::from(((0, 0), (0, 0))))
    );
}

#[test]
fn test_alternatives() {
    use super::alternative;

    ok_case!(
        alternative,
        &vec![
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((1, 0), (2, 0)))),
            Token::Concatenation.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Terminal("b".to_owned()).spanning(Span::from(((4, 0), (7, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((8, 0), (9, 0)))),
            Token::Terminal("c".to_owned()).spanning(Span::from(((10, 0), (13, 0)))),
            Token::Concatenation.spanning(Span::from(((13, 0), (14, 0)))),
            Token::Nonterminal("d".to_owned()).spanning(Span::from(((15, 0), (16, 0))))
        ],
        7,
        Expression::Alternative {
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Nonterminal("a".to_owned())
                            .spanning(Span::from(((1, 0), (2, 0))))
                    ),
                    second: Box::new(
                        Expression::Terminal("b".to_owned()).spanning(Span::from(((4, 0), (7, 0))))
                    ),
                    rest: Vec::new(),
                }
                .spanning(Span::from(((1, 0), (7, 0))))
            ),
            second: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Terminal("c".to_owned())
                            .spanning(Span::from(((10, 0), (13, 0))))
                    ),
                    second: Box::new(
                        Expression::Nonterminal("d".to_owned())
                            .spanning(Span::from(((15, 0), (16, 0))))
                    ),
                    rest: Vec::new(),
                }
                .spanning(Span::from(((10, 0), (16, 0))))
            ),
            rest: Vec::new(),
        }
        .spanning(Span::from(((1, 0), (16, 0))))
    );
    ok_case!(
        alternative,
        &vec![],
        0,
        Expression::Empty.spanning(Span::from(((0, 0), (0, 0))))
    );
}

#[test]
fn test_grouped() {
    use super::grouped;

    ok_case!(
        grouped,
        &vec![
            Token::StartGroup.spanning(Span::from(((0, 0), (1, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((1, 0), (2, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((3, 0), (4, 0)))),
            Token::Nonterminal("c".to_owned()).spanning(Span::from(((5, 0), (6, 0)))),
            Token::EndGroup.spanning(Span::from(((6, 0), (7, 0))))
        ],
        5,
        Expression::Alternative {
            first: Box::new(
                Expression::Nonterminal("b".to_owned()).spanning(Span::from(((1, 0), (2, 0))))
            ),
            second: Box::new(
                Expression::Nonterminal("c".to_owned()).spanning(Span::from(((5, 0), (6, 0))))
            ),
            rest: Vec::new(),
        }
        .spanning(Span::from(((0, 0), (7, 0))))
    );
    ok_case!(
        grouped,
        &vec![
            Token::StartGroup.spanning(Span::from(((0, 0), (1, 0)))),
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((2, 0), (3, 0)))),
            Token::Concatenation.spanning(Span::from(((3, 0), (4, 0)))),
            Token::Terminal("b".to_owned()).spanning(Span::from(((5, 0), (8, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((23, 0), (24, 0)))),
            Token::Nonterminal("c".to_owned()).spanning(Span::from(((25, 0), (26, 0)))),
            Token::EndGroup.spanning(Span::from(((27, 0), (28, 0))))
        ],
        7,
        Expression::Alternative {
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Nonterminal("a".to_owned())
                            .spanning(Span::from(((2, 0), (3, 0))))
                    ),
                    second: Box::new(
                        Expression::Terminal("b".to_owned()).spanning(Span::from(((5, 0), (8, 0))))
                    ),
                    rest: Vec::new(),
                }
                .spanning(Span::from(((2, 0), (8, 0))))
            ),
            second: Box::new(
                Expression::Nonterminal("c".to_owned()).spanning(Span::from(((25, 0), (26, 0))))
            ),
            rest: Vec::new(),
        }
        .spanning(Span::from(((0, 0), (28, 0))))
    );
}

#[test]
fn test_repeated() {
    use super::repeated;

    ok_case!(
        repeated,
        &vec![
            Token::StartRepeat.spanning(Span::from(((0, 0), (1, 0)))),
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((1, 0), (4, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((10, 0), (11, 0)))),
            Token::Nonterminal("def".to_owned()).spanning(Span::from(((11, 0), (14, 0)))),
            Token::EndRepeat.spanning(Span::from(((15, 0), (16, 0))))
        ],
        5,
        Expression::Repeated(Box::new(
            Expression::Alternative {
                first: Box::new(
                    Expression::Nonterminal("abc".to_owned())
                        .spanning(Span::from(((1, 0), (4, 0))))
                ),
                second: Box::new(
                    Expression::Nonterminal("def".to_owned())
                        .spanning(Span::from(((11, 0), (14, 0))))
                ),
                rest: Vec::new(),
            }
            .spanning(Span::from(((1, 0), (14, 0))))
        ))
        .spanning(Span::from(((0, 0), (16, 0))))
    );
}

#[test]
fn test_optionals() {
    use super::optional;

    ok_case!(
        optional,
        &vec![
            Token::StartOption.spanning(Span::from(((0, 0), (1, 0)))),
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((2, 0), (5, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((5, 0), (6, 0)))),
            Token::Nonterminal("def".to_owned()).spanning(Span::from(((6, 0), (9, 0)))),
            Token::EndOption.spanning(Span::from(((19, 0), (20, 0))))
        ],
        5,
        Expression::Optional(Box::new(
            Expression::Alternative {
                first: Box::new(
                    Expression::Nonterminal("abc".to_owned())
                        .spanning(Span::from(((2, 0), (5, 0))))
                ),
                second: Box::new(
                    Expression::Nonterminal("def".to_owned())
                        .spanning(Span::from(((6, 0), (9, 0))))
                ),
                rest: Vec::new(),
            }
            .spanning(Span::from(((2, 0), (9, 0))))
        ))
        .spanning(Span::from(((0, 0), (20, 0))))
    );
}

#[test]
fn test_productions() {
    use super::production;

    ok_case!(
        production,
        &vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0)))),
            Token::Definition.spanning(Span::from(((4, 0), (5, 0)))),
            Token::Terminal("a".to_owned()).spanning(Span::from(((6, 0), (9, 0)))),
            Token::Concatenation.spanning(Span::from(((9, 0), (10, 0)))),
            Token::StartGroup.spanning(Span::from(((11, 0), (12, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((12, 0), (13, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((14, 0), (15, 0)))),
            Token::Terminal("c".to_owned()).spanning(Span::from(((16, 0), (19, 0)))),
            Token::EndGroup.spanning(Span::from(((30, 0), (31, 0)))),
            Token::Terminator.spanning(Span::from(((31, 0), (32, 0))))
        ],
        10,
        Production {
            lhs: "abc".to_owned().spanning(Span::from(((0, 0), (3, 0)))),
            rhs: Expression::Sequence {
                first: Box::new(
                    Expression::Terminal("a".to_owned()).spanning(Span::from(((6, 0), (9, 0))))
                ),
                second: Box::new(
                    Expression::Alternative {
                        first: Box::new(
                            Expression::Nonterminal("b".to_owned())
                                .spanning(Span::from(((12, 0), (13, 0))))
                        ),
                        second: Box::new(
                            Expression::Terminal("c".to_owned())
                                .spanning(Span::from(((16, 0), (19, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .spanning(Span::from(((11, 0), (31, 0))))
                ),
                rest: Vec::new(),
            }
            .spanning(Span::from(((6, 0), (31, 0))))
        }
        .spanning(Span::from(((0, 0), (32, 0))))
    );
    failure_case!(
        production,
        &vec![Token::Terminator.spanning(Span::from(((0, 0), (1, 0))))],
        Error::IdentifierExpected.spanning(Span::from(((0, 0), (1, 0))))
    );
    ok_case!(
        production,
        &vec![
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Definition.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((4, 0), (5, 0)))),
            Token::Terminator.spanning(Span::from(((5, 0), (6, 0)))),
            Token::Terminator.spanning(Span::from(((6, 0), (7, 0))))
        ],
        4,
        Production {
            lhs: "a".to_owned().spanning(Span::from(((0, 0), (1, 0)))),
            rhs: Expression::Nonterminal("b".to_owned()).spanning(Span::from(((4, 0), (5, 0))))
        }
        .spanning(Span::from(((0, 0), (6, 0))))
    );
    ok_case!(
        production,
        &vec![
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Definition.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Terminator.spanning(Span::from(((4, 0), (5, 0))))
        ],
        3,
        Production {
            lhs: "a".to_owned().spanning(Span::from(((0, 0), (1, 0)))),
            rhs: Expression::Empty.spanning(Span::from(((3, 0), (4, 0))))
        }
        .spanning(Span::from(((0, 0), (5, 0))))
    );
}

#[test]
fn test_syntaxes() {
    use super::syntax;

    error_case!(
        syntax,
        &vec![],
        Error::IdentifierExpected.spanning(Span::from(((0, 0), (0, 0))))
    );
    ok_case!(
        syntax,
        &vec![
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((0, 0), (1, 0)))),
            Token::Definition.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Terminal("d".to_owned()).spanning(Span::from(((4, 0), (6, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((8, 0), (9, 0)))),
            Token::StartRepeat.spanning(Span::from(((10, 0), (11, 0)))),
            Token::Integer(2).spanning(Span::from(((11, 0), (12, 0)))),
            Token::Repetition.spanning(Span::from(((13, 0), (14, 0)))),
            Token::Terminal("e".to_owned()).spanning(Span::from(((15, 0), (18, 0)))),
            Token::EndRepeat.spanning(Span::from(((18, 0), (19, 0)))),
            Token::Terminator.spanning(Span::from(((19, 0), (20, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((33, 0), (34, 0)))),
            Token::Definition.spanning(Span::from(((35, 0), (36, 0)))),
            Token::Terminal("a".to_owned()).spanning(Span::from(((37, 0), (40, 0)))),
            Token::Concatenation.spanning(Span::from(((40, 0), (41, 0)))),
            Token::StartGroup.spanning(Span::from(((42, 0), (43, 0)))),
            Token::Nonterminal("a".to_owned()).spanning(Span::from(((43, 0), (44, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((45, 0), (46, 0)))),
            Token::Terminal("c".to_owned()).spanning(Span::from(((47, 0), (50, 0)))),
            Token::EndGroup.spanning(Span::from(((50, 0), (51, 0)))),
            Token::Terminator.spanning(Span::from(((51, 0), (52, 0)))),
        ],
        20,
        Grammar {
            productions: vec![
                Production {
                    lhs: "a".to_owned().spanning(Span::from(((0, 0), (1, 0)))),
                    rhs: Expression::Alternative {
                        first: Box::new(
                            Expression::Terminal("d".to_owned())
                                .spanning(Span::from(((4, 0), (6, 0))))
                        ),
                        second: Box::new(
                            Expression::Repeated(Box::new(
                                Expression::Factor {
                                    count: 2.spanning(Span::from(((11, 0), (12, 0)))),
                                    primary: Box::new(
                                        Expression::Terminal("e".to_owned())
                                            .spanning(Span::from(((15, 0), (18, 0))))
                                    )
                                }
                                .spanning(Span::from(((11, 0), (18, 0))))
                            ))
                            .spanning(Span::from(((10, 0), (19, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .spanning(Span::from(((4, 0), (19, 0))))
                }
                .spanning(Span::from(((0, 0), (20, 0)))),
                Production {
                    lhs: "b".to_owned().spanning(Span::from(((33, 0), (34, 0)))),
                    rhs: Expression::Sequence {
                        first: Box::new(
                            Expression::Terminal("a".to_owned())
                                .spanning(Span::from(((37, 0), (40, 0))))
                        ),
                        second: Box::new(
                            Expression::Alternative {
                                first: Box::new(
                                    Expression::Nonterminal("a".to_owned())
                                        .spanning(Span::from(((43, 0), (44, 0))))
                                ),
                                second: Box::new(
                                    Expression::Terminal("c".to_owned())
                                        .spanning(Span::from(((47, 0), (50, 0))))
                                ),
                                rest: Vec::new(),
                            }
                            .spanning(Span::from(((42, 0), (51, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .spanning(Span::from(((37, 0), (51, 0))))
                }
                .spanning(Span::from(((33, 0), (52, 0))))
            ]
        }
        .spanning(Span::from(((0, 0), (52, 0))))
    );
}

use quickcheck::{Arbitrary, Gen};
use rand::seq::SliceRandom;

impl Arbitrary for Token {
    fn arbitrary<G: Gen>(g: &mut G) -> Token {
        let vals = &[
            Token::Nonterminal(String::arbitrary(g)),
            Token::Terminal(String::arbitrary(g)),
            Token::Special(String::arbitrary(g)),
            Token::Integer(usize::arbitrary(g)),
            Token::Concatenation,
            Token::Definition,
            Token::DefinitionSeparator,
            Token::EndGroup,
            Token::EndOption,
            Token::EndRepeat,
            Token::Exception,
            Token::Repetition,
            Token::StartGroup,
            Token::StartOption,
            Token::StartRepeat,
            Token::Terminator,
        ];
        vals.choose(g).unwrap().clone()
    }
}

impl Arbitrary for Location {
    fn arbitrary<G: Gen>(g: &mut G) -> Location {
        Location {
            column: usize::arbitrary(g),
            line: usize::arbitrary(g),
        }
    }
}

impl Arbitrary for Span {
    fn arbitrary<G: Gen>(g: &mut G) -> Span {
        Span {
            from: Location::arbitrary(g),
            to: Location::arbitrary(g),
        }
    }
}

impl<T: Arbitrary> Arbitrary for Spanned<T> {
    fn arbitrary<G: Gen>(g: &mut G) -> Spanned<T> {
        Spanned {
            node: T::arbitrary(g),
            span: Span::arbitrary(g),
        }
    }
}

#[quickcheck]
fn test_arbitrary_input(tokens: Vec<Spanned<Token>>) {
    use super::parse;
    let _ = parse(tokens.as_slice());
}

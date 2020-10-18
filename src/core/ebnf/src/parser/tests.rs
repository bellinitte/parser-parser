use super::{Error, ErrorKind, Expression, Grammar, Production, Span, Token, TokenKind, Tokens};
use nom::{Err, Slice};

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
        &vec![Token::new(
            TokenKind::Terminal("terminal".to_owned()),
            Span::from(((1, 0), (11, 0)))
        )],
        1,
        Expression::Terminal {
            content: "terminal".to_owned(),
            span: Span::from(((1, 0), (11, 0))),
        }
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Nonterminal("nonterminal".to_owned()),
            Span::from(((0, 0), (13, 0)))
        )],
        1,
        Expression::Nonterminal {
            identifier: "nonterminal".to_owned(),
            span: Span::from(((0, 0), (13, 0))),
        }
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Special(" special ".to_owned()),
            Span::from(((2, 0), (13, 0)))
        )],
        1,
        Expression::Special {
            content: " special ".to_owned(),
            span: Span::from(((2, 0), (13, 0)))
        }
    );
    ok_case!(
        factor,
        &vec![],
        0,
        Expression::Empty {
            span: Span::from(((0, 0), (0, 0)))
        }
    );
    ok_case!(
        factor,
        &vec![
            Token::new(TokenKind::Integer(2), Span::from(((0, 0), (1, 0)))),
            Token::new(TokenKind::Repetition, Span::from(((2, 0), (3, 0)))),
            Token::new(
                TokenKind::Terminal("terminal".to_owned()),
                Span::from(((4, 0), (14, 0)))
            )
        ],
        3,
        Expression::Factor {
            count: 2,
            primary: Box::new(Expression::Terminal {
                content: "terminal".to_owned(),
                span: Span::from(((4, 0), (14, 0)))
            }),
            span: Span::from(((0, 0), (14, 0)))
        }
    );
    failure_case!(
        factor,
        &vec![
            Token::new(TokenKind::Integer(2), Span::from(((0, 0), (1, 0)))),
            Token::new(
                TokenKind::Terminal("terminal".to_owned()),
                Span::from(((2, 0), (12, 0)))
            )
        ],
        Error {
            kind: ErrorKind::RepetitionSymbolExpected,
            span: Span::from(((2, 0), (12, 0)))
        }
    );
}

#[test]
fn test_terms() {
    use super::term;

    ok_case!(
        term,
        &vec![
            Token::new(
                TokenKind::Nonterminal("abc".to_owned()),
                Span::from(((0, 0), (3, 0)))
            ),
            Token::new(TokenKind::Exception, Span::from(((4, 0), (5, 0)))),
            Token::new(
                TokenKind::Terminal("test".to_owned()),
                Span::from(((6, 0), (12, 0)))
            )
        ],
        3,
        Expression::Exception {
            subject: Box::new(Expression::Nonterminal {
                identifier: "abc".to_owned(),
                span: Span::from(((0, 0), (3, 0)))
            }),
            restriction: Box::new(Expression::Terminal {
                content: "test".to_owned(),
                span: Span::from(((6, 0), (12, 0)))
            }),
            span: Span::from(((0, 0), (12, 0)))
        }
    );
    ok_case!(
        term,
        &vec![
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((0, 0), (1, 0)))
            ),
            Token::new(TokenKind::Exception, Span::from(((1, 0), (2, 0)))),
            Token::new(
                TokenKind::Nonterminal("b".to_owned()),
                Span::from(((2, 0), (3, 0)))
            ),
            Token::new(TokenKind::Exception, Span::from(((3, 0), (4, 0)))),
            Token::new(
                TokenKind::Nonterminal("c".to_owned()),
                Span::from(((4, 0), (5, 0)))
            )
        ],
        3,
        Expression::Exception {
            subject: Box::new(Expression::Nonterminal {
                identifier: "a".to_owned(),
                span: Span::from(((0, 0), (1, 0)))
            }),
            restriction: Box::new(Expression::Nonterminal {
                identifier: "b".to_owned(),
                span: Span::from(((2, 0), (3, 0)))
            }),
            span: Span::from(((0, 0), (3, 0)))
        }
    );
    ok_case!(
        term,
        &vec![],
        0,
        Expression::Empty {
            span: Span::from(((0, 0), (0, 0)))
        }
    );
}

#[test]
fn test_sequences() {
    use super::sequence;

    ok_case!(
        sequence,
        &vec![
            Token::new(
                TokenKind::Nonterminal("abc".to_owned()),
                Span::from(((0, 0), (3, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((3, 0), (4, 0)))),
            Token::new(
                TokenKind::Terminal("test".to_owned()),
                Span::from(((5, 0), (11, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((11, 0), (12, 0)))),
            Token::new(
                TokenKind::Nonterminal("bca".to_owned()),
                Span::from(((14, 0), (17, 0)))
            )
        ],
        5,
        Expression::Sequence {
            first: Box::new(Expression::Nonterminal {
                identifier: "abc".to_owned(),
                span: Span::from(((0, 0), (3, 0)))
            }),
            second: Box::new(Expression::Terminal {
                content: "test".to_owned(),
                span: Span::from(((5, 0), (11, 0)))
            }),
            rest: vec![Expression::Nonterminal {
                identifier: "bca".to_owned(),
                span: Span::from(((14, 0), (17, 0)))
            }],
            span: Span::from(((0, 0), (17, 0)))
        }
    );
    ok_case!(
        sequence,
        &vec![],
        0,
        Expression::Empty {
            span: Span::from(((0, 0), (0, 0)))
        }
    );
}

#[test]
fn test_alternatives() {
    use super::alternative;

    ok_case!(
        alternative,
        &vec![
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((1, 0), (2, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((2, 0), (3, 0)))),
            Token::new(
                TokenKind::Terminal("b".to_owned()),
                Span::from(((4, 0), (7, 0)))
            ),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((8, 0), (9, 0)))),
            Token::new(
                TokenKind::Terminal("c".to_owned()),
                Span::from(((10, 0), (13, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((13, 0), (14, 0)))),
            Token::new(
                TokenKind::Nonterminal("d".to_owned()),
                Span::from(((15, 0), (16, 0)))
            )
        ],
        7,
        Expression::Alternative {
            first: Box::new(Expression::Sequence {
                first: Box::new(Expression::Nonterminal {
                    identifier: "a".to_owned(),
                    span: Span::from(((1, 0), (2, 0)))
                }),
                second: Box::new(Expression::Terminal {
                    content: "b".to_owned(),
                    span: Span::from(((4, 0), (7, 0)))
                }),
                rest: Vec::new(),
                span: Span::from(((1, 0), (7, 0)))
            }),
            second: Box::new(Expression::Sequence {
                first: Box::new(Expression::Terminal {
                    content: "c".to_owned(),
                    span: Span::from(((10, 0), (13, 0)))
                }),
                second: Box::new(Expression::Nonterminal {
                    identifier: "d".to_owned(),
                    span: Span::from(((15, 0), (16, 0)))
                }),
                rest: Vec::new(),
                span: Span::from(((10, 0), (16, 0)))
            }),
            rest: Vec::new(),
            span: Span::from(((1, 0), (16, 0)))
        }
    );
    ok_case!(
        alternative,
        &vec![],
        0,
        Expression::Empty {
            span: Span::from(((0, 0), (0, 0)))
        }
    );
}

#[test]
fn test_grouped() {
    use super::grouped;

    ok_case!(
        grouped,
        &vec![
            Token::new(TokenKind::StartGroup, Span::from(((0, 0), (1, 0)))),
            Token::new(
                TokenKind::Nonterminal("b".to_owned()),
                Span::from(((1, 0), (2, 0)))
            ),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((3, 0), (4, 0)))),
            Token::new(
                TokenKind::Nonterminal("c".to_owned()),
                Span::from(((5, 0), (6, 0)))
            ),
            Token::new(TokenKind::EndGroup, Span::from(((6, 0), (7, 0))))
        ],
        5,
        Expression::Alternative {
            first: Box::new(Expression::Nonterminal {
                identifier: "b".to_owned(),
                span: Span::from(((1, 0), (2, 0)))
            }),
            second: Box::new(Expression::Nonterminal {
                identifier: "c".to_owned(),
                span: Span::from(((5, 0), (6, 0)))
            }),
            rest: Vec::new(),
            span: Span::from(((0, 0), (7, 0)))
        }
    );
    ok_case!(
        grouped,
        &vec![
            Token::new(TokenKind::StartGroup, Span::from(((0, 0), (1, 0)))),
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((2, 0), (3, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((3, 0), (4, 0)))),
            Token::new(
                TokenKind::Terminal("b".to_owned()),
                Span::from(((5, 0), (8, 0)))
            ),
            Token::new(
                TokenKind::DefinitionSeparator,
                Span::from(((23, 0), (24, 0)))
            ),
            Token::new(
                TokenKind::Nonterminal("c".to_owned()),
                Span::from(((25, 0), (26, 0)))
            ),
            Token::new(TokenKind::EndGroup, Span::from(((27, 0), (28, 0))))
        ],
        7,
        Expression::Alternative {
            first: Box::new(Expression::Sequence {
                first: Box::new(Expression::Nonterminal {
                    identifier: "a".to_owned(),
                    span: Span::from(((2, 0), (3, 0)))
                }),
                second: Box::new(Expression::Terminal {
                    content: "b".to_owned(),
                    span: Span::from(((5, 0), (8, 0)))
                }),
                rest: Vec::new(),
                span: Span::from(((2, 0), (8, 0)))
            }),
            second: Box::new(Expression::Nonterminal {
                identifier: "c".to_owned(),
                span: Span::from(((25, 0), (26, 0)))
            }),
            rest: Vec::new(),
            span: Span::from(((0, 0), (28, 0)))
        }
    );
}

#[test]
fn test_repeated() {
    use super::repeated;

    ok_case!(
        repeated,
        &vec![
            Token::new(TokenKind::StartRepeat, Span::from(((0, 0), (1, 0)))),
            Token::new(
                TokenKind::Nonterminal("abc".to_owned()),
                Span::from(((1, 0), (4, 0)))
            ),
            Token::new(
                TokenKind::DefinitionSeparator,
                Span::from(((10, 0), (11, 0)))
            ),
            Token::new(
                TokenKind::Nonterminal("def".to_owned()),
                Span::from(((11, 0), (14, 0)))
            ),
            Token::new(TokenKind::EndRepeat, Span::from(((15, 0), (16, 0))))
        ],
        5,
        Expression::Repeated {
            inner: Box::new(Expression::Alternative {
                first: Box::new(Expression::Nonterminal {
                    identifier: "abc".to_owned(),
                    span: Span::from(((1, 0), (4, 0)))
                }),
                second: Box::new(Expression::Nonterminal {
                    identifier: "def".to_owned(),
                    span: Span::from(((11, 0), (14, 0)))
                }),
                rest: Vec::new(),
                span: Span::from(((1, 0), (14, 0)))
            }),
            span: Span::from(((0, 0), (16, 0)))
        }
    );
}

#[test]
fn test_optionals() {
    use super::optional;

    ok_case!(
        optional,
        &vec![
            Token::new(TokenKind::StartOption, Span::from(((0, 0), (1, 0)))),
            Token::new(
                TokenKind::Nonterminal("abc".to_owned()),
                Span::from(((2, 0), (5, 0)))
            ),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((5, 0), (6, 0)))),
            Token::new(
                TokenKind::Nonterminal("def".to_owned()),
                Span::from(((6, 0), (9, 0)))
            ),
            Token::new(TokenKind::EndOption, Span::from(((19, 0), (20, 0))))
        ],
        5,
        Expression::Optional {
            inner: Box::new(Expression::Alternative {
                first: Box::new(Expression::Nonterminal {
                    identifier: "abc".to_owned(),
                    span: Span::from(((2, 0), (5, 0)))
                }),
                second: Box::new(Expression::Nonterminal {
                    identifier: "def".to_owned(),
                    span: Span::from(((6, 0), (9, 0)))
                }),
                rest: Vec::new(),
                span: Span::from(((2, 0), (9, 0)))
            }),
            span: Span::from(((0, 0), (20, 0)))
        }
    );
}

#[test]
fn test_productions() {
    use super::production;

    ok_case!(
        production,
        &vec![
            Token::new(
                TokenKind::Nonterminal("abc".to_owned()),
                Span::from(((0, 0), (3, 0)))
            ),
            Token::new(TokenKind::Definition, Span::from(((4, 0), (5, 0)))),
            Token::new(
                TokenKind::Terminal("a".to_owned()),
                Span::from(((6, 0), (9, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((9, 0), (10, 0)))),
            Token::new(TokenKind::StartGroup, Span::from(((11, 0), (12, 0)))),
            Token::new(
                TokenKind::Nonterminal("b".to_owned()),
                Span::from(((12, 0), (13, 0)))
            ),
            Token::new(
                TokenKind::DefinitionSeparator,
                Span::from(((14, 0), (15, 0)))
            ),
            Token::new(
                TokenKind::Terminal("c".to_owned()),
                Span::from(((16, 0), (19, 0)))
            ),
            Token::new(TokenKind::EndGroup, Span::from(((30, 0), (31, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((31, 0), (32, 0))))
        ],
        10,
        (
            "abc".to_owned(),
            Production {
                expression: Expression::Sequence {
                    first: Box::new(Expression::Terminal {
                        content: "a".to_owned(),
                        span: Span::from(((6, 0), (9, 0)))
                    }),
                    second: Box::new(Expression::Alternative {
                        first: Box::new(Expression::Nonterminal {
                            identifier: "b".to_owned(),
                            span: Span::from(((12, 0), (13, 0)))
                        }),
                        second: Box::new(Expression::Terminal {
                            content: "c".to_owned(),
                            span: Span::from(((16, 0), (19, 0)))
                        }),
                        rest: Vec::new(),
                        span: Span::from(((11, 0), (31, 0)))
                    }),
                    rest: Vec::new(),
                    span: Span::from(((6, 0), (31, 0)))
                },
                production_span: Span::from(((0, 0), (32, 0))),
                identifier_span: Span::from(((0, 0), (3, 0))),
            }
        )
    );
    failure_case!(
        production,
        &vec![Token::new(
            TokenKind::Terminator,
            Span::from(((0, 0), (1, 0)))
        )],
        Error {
            kind: ErrorKind::IdentifierExpected,
            span: Span::from(((0, 0), (1, 0)))
        }
    );
    ok_case!(
        production,
        &vec![
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((0, 0), (1, 0)))
            ),
            Token::new(TokenKind::Definition, Span::from(((2, 0), (3, 0)))),
            Token::new(
                TokenKind::Nonterminal("b".to_owned()),
                Span::from(((4, 0), (5, 0)))
            ),
            Token::new(TokenKind::Terminator, Span::from(((5, 0), (6, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((6, 0), (7, 0))))
        ],
        4,
        (
            "a".to_owned(),
            Production {
                expression: Expression::Nonterminal {
                    identifier: "b".to_owned(),
                    span: Span::from(((4, 0), (5, 0)))
                },
                identifier_span: Span::from(((0, 0), (1, 0))),
                production_span: Span::from(((0, 0), (6, 0)))
            }
        )
    );
    ok_case!(
        production,
        &vec![
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((0, 0), (1, 0)))
            ),
            Token::new(TokenKind::Definition, Span::from(((2, 0), (3, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((4, 0), (5, 0))))
        ],
        3,
        (
            "a".to_owned(),
            Production {
                expression: Expression::Empty {
                    span: Span::from(((3, 0), (4, 0)))
                },
                identifier_span: Span::from(((0, 0), (1, 0))),
                production_span: Span::from(((0, 0), (5, 0))),
            }
        )
    );
}

#[test]
fn test_syntaxes() {
    use super::syntax;

    error_case!(
        syntax,
        &vec![],
        Error {
            kind: ErrorKind::IdentifierExpected,
            span: Span::from(((0, 0), (0, 0)))
        }
    );
    ok_case!(
        syntax,
        &vec![
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((0, 0), (1, 0)))
            ),
            Token::new(TokenKind::Definition, Span::from(((2, 0), (3, 0)))),
            Token::new(
                TokenKind::Terminal("d".to_owned()),
                Span::from(((4, 0), (6, 0)))
            ),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((8, 0), (9, 0)))),
            Token::new(TokenKind::StartRepeat, Span::from(((10, 0), (11, 0)))),
            Token::new(TokenKind::Integer(2), Span::from(((11, 0), (12, 0)))),
            Token::new(TokenKind::Repetition, Span::from(((13, 0), (14, 0)))),
            Token::new(
                TokenKind::Terminal("e".to_owned()),
                Span::from(((15, 0), (18, 0)))
            ),
            Token::new(TokenKind::EndRepeat, Span::from(((18, 0), (19, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((19, 0), (20, 0)))),
            Token::new(
                TokenKind::Nonterminal("b".to_owned()),
                Span::from(((33, 0), (34, 0)))
            ),
            Token::new(TokenKind::Definition, Span::from(((35, 0), (36, 0)))),
            Token::new(
                TokenKind::Terminal("a".to_owned()),
                Span::from(((37, 0), (40, 0)))
            ),
            Token::new(TokenKind::Concatenation, Span::from(((40, 0), (41, 0)))),
            Token::new(TokenKind::StartGroup, Span::from(((42, 0), (43, 0)))),
            Token::new(
                TokenKind::Nonterminal("a".to_owned()),
                Span::from(((43, 0), (44, 0)))
            ),
            Token::new(
                TokenKind::DefinitionSeparator,
                Span::from(((45, 0), (46, 0)))
            ),
            Token::new(
                TokenKind::Terminal("c".to_owned()),
                Span::from(((47, 0), (50, 0)))
            ),
            Token::new(TokenKind::EndGroup, Span::from(((50, 0), (51, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((51, 0), (52, 0)))),
        ],
        20,
        Grammar {
            productions: vec![
                (
                    "a".to_owned(),
                    vec![Production {
                        expression: Expression::Alternative {
                            first: Box::new(Expression::Terminal {
                                content: "d".to_owned(),
                                span: Span::from(((4, 0), (6, 0)))
                            }),
                            second: Box::new(Expression::Repeated {
                                inner: Box::new(Expression::Factor {
                                    count: 2,
                                    primary: Box::new(Expression::Terminal {
                                        content: "e".to_owned(),
                                        span: Span::from(((15, 0), (18, 0)))
                                    }),
                                    span: Span::from(((11, 0), (18, 0)))
                                }),
                                span: Span::from(((10, 0), (19, 0)))
                            }),
                            rest: Vec::new(),
                            span: Span::from(((4, 0), (19, 0)))
                        },
                        identifier_span: Span::from(((0, 0), (1, 0))),
                        production_span: Span::from(((0, 0), (20, 0)))
                    }]
                ),
                (
                    "b".to_owned(),
                    vec![Production {
                        expression: Expression::Sequence {
                            first: Box::new(Expression::Terminal {
                                content: "a".to_owned(),
                                span: Span::from(((37, 0), (40, 0)))
                            }),
                            second: Box::new(Expression::Alternative {
                                first: Box::new(Expression::Nonterminal {
                                    identifier: "a".to_owned(),
                                    span: Span::from(((43, 0), (44, 0)))
                                }),
                                second: Box::new(Expression::Terminal {
                                    content: "c".to_owned(),
                                    span: Span::from(((47, 0), (50, 0)))
                                }),
                                rest: Vec::new(),
                                span: Span::from(((42, 0), (51, 0)))
                            }),
                            rest: Vec::new(),
                            span: Span::from(((37, 0), (51, 0)))
                        },
                        production_span: Span::from(((33, 0), (52, 0))),
                        identifier_span: Span::from(((33, 0), (34, 0)))
                    }]
                )
            ]
            .into_iter()
            .collect(),
            span: Span::from(((0, 0), (52, 0)))
        }
    );
}

use super::{
    Error, ErrorKind, Expression, Grammar, NodeAt, Production, Span, Token, TokenKind, Tokens,
};
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
        Expression::Terminal("terminal".to_owned()).node_at(Span::from(((1, 0), (11, 0))))
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Nonterminal("nonterminal".to_owned()),
            Span::from(((0, 0), (13, 0)))
        )],
        1,
        Expression::Nonterminal("nonterminal".to_owned()).node_at(Span::from(((0, 0), (13, 0))))
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Special(" special ".to_owned()),
            Span::from(((2, 0), (13, 0)))
        )],
        1,
        Expression::Special(" special ".to_owned()).node_at(Span::from(((2, 0), (13, 0))))
    );
    ok_case!(
        factor,
        &vec![],
        0,
        Expression::Empty.node_at(Span::from(((0, 0), (0, 0))))
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
            count: 2.node_at(Span::from(((0, 0), (1, 0)))),
            primary: Box::new(
                Expression::Terminal("terminal".to_owned()).node_at(Span::from(((4, 0), (14, 0))))
            )
        }
        .node_at(Span::from(((0, 0), (14, 0))))
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
            subject: Box::new(
                Expression::Nonterminal("abc".to_owned()).node_at(Span::from(((0, 0), (3, 0))))
            ),
            restriction: Box::new(
                Expression::Terminal("test".to_owned()).node_at(Span::from(((6, 0), (12, 0))))
            ),
        }
        .node_at(Span::from(((0, 0), (12, 0))))
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
            subject: Box::new(
                Expression::Nonterminal("a".to_owned()).node_at(Span::from(((0, 0), (1, 0))))
            ),
            restriction: Box::new(
                Expression::Nonterminal("b".to_owned()).node_at(Span::from(((2, 0), (3, 0))))
            ),
        }
        .node_at(Span::from(((0, 0), (3, 0))))
    );
    ok_case!(
        term,
        &vec![],
        0,
        Expression::Empty.node_at(Span::from(((0, 0), (0, 0))))
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
            first: Box::new(
                Expression::Nonterminal("abc".to_owned()).node_at(Span::from(((0, 0), (3, 0))))
            ),
            second: Box::new(
                Expression::Terminal("test".to_owned()).node_at(Span::from(((5, 0), (11, 0))))
            ),
            rest: vec![
                Expression::Nonterminal("bca".to_owned()).node_at(Span::from(((14, 0), (17, 0))))
            ]
        }
        .node_at(Span::from(((0, 0), (17, 0))))
    );
    ok_case!(
        sequence,
        &vec![],
        0,
        Expression::Empty.node_at(Span::from(((0, 0), (0, 0))))
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
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Nonterminal("a".to_owned())
                            .node_at(Span::from(((1, 0), (2, 0))))
                    ),
                    second: Box::new(
                        Expression::Terminal("b".to_owned()).node_at(Span::from(((4, 0), (7, 0))))
                    ),
                    rest: Vec::new(),
                }
                .node_at(Span::from(((1, 0), (7, 0))))
            ),
            second: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Terminal("c".to_owned())
                            .node_at(Span::from(((10, 0), (13, 0))))
                    ),
                    second: Box::new(
                        Expression::Nonterminal("d".to_owned())
                            .node_at(Span::from(((15, 0), (16, 0))))
                    ),
                    rest: Vec::new(),
                }
                .node_at(Span::from(((10, 0), (16, 0))))
            ),
            rest: Vec::new(),
        }
        .node_at(Span::from(((1, 0), (16, 0))))
    );
    ok_case!(
        alternative,
        &vec![],
        0,
        Expression::Empty.node_at(Span::from(((0, 0), (0, 0))))
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
            first: Box::new(
                Expression::Nonterminal("b".to_owned()).node_at(Span::from(((1, 0), (2, 0))))
            ),
            second: Box::new(
                Expression::Nonterminal("c".to_owned()).node_at(Span::from(((5, 0), (6, 0))))
            ),
            rest: Vec::new(),
        }
        .node_at(Span::from(((0, 0), (7, 0))))
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
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(
                        Expression::Nonterminal("a".to_owned())
                            .node_at(Span::from(((2, 0), (3, 0))))
                    ),
                    second: Box::new(
                        Expression::Terminal("b".to_owned()).node_at(Span::from(((5, 0), (8, 0))))
                    ),
                    rest: Vec::new(),
                }
                .node_at(Span::from(((2, 0), (8, 0))))
            ),
            second: Box::new(
                Expression::Nonterminal("c".to_owned()).node_at(Span::from(((25, 0), (26, 0))))
            ),
            rest: Vec::new(),
        }
        .node_at(Span::from(((0, 0), (28, 0))))
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
        Expression::Repeated(Box::new(
            Expression::Alternative {
                first: Box::new(
                    Expression::Nonterminal("abc".to_owned()).node_at(Span::from(((1, 0), (4, 0))))
                ),
                second: Box::new(
                    Expression::Nonterminal("def".to_owned())
                        .node_at(Span::from(((11, 0), (14, 0))))
                ),
                rest: Vec::new(),
            }
            .node_at(Span::from(((1, 0), (14, 0))))
        ))
        .node_at(Span::from(((0, 0), (16, 0))))
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
        Expression::Optional(Box::new(
            Expression::Alternative {
                first: Box::new(
                    Expression::Nonterminal("abc".to_owned()).node_at(Span::from(((2, 0), (5, 0))))
                ),
                second: Box::new(
                    Expression::Nonterminal("def".to_owned()).node_at(Span::from(((6, 0), (9, 0))))
                ),
                rest: Vec::new(),
            }
            .node_at(Span::from(((2, 0), (9, 0))))
        ))
        .node_at(Span::from(((0, 0), (20, 0))))
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
        Production {
            lhs: "abc".to_owned().node_at(Span::from(((0, 0), (3, 0)))),
            rhs: Expression::Sequence {
                first: Box::new(
                    Expression::Terminal("a".to_owned()).node_at(Span::from(((6, 0), (9, 0))))
                ),
                second: Box::new(
                    Expression::Alternative {
                        first: Box::new(
                            Expression::Nonterminal("b".to_owned())
                                .node_at(Span::from(((12, 0), (13, 0))))
                        ),
                        second: Box::new(
                            Expression::Terminal("c".to_owned())
                                .node_at(Span::from(((16, 0), (19, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .node_at(Span::from(((11, 0), (31, 0))))
                ),
                rest: Vec::new(),
            }
            .node_at(Span::from(((6, 0), (31, 0))))
        }
        .node_at(Span::from(((0, 0), (32, 0))))
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
        Production {
            lhs: "a".to_owned().node_at(Span::from(((0, 0), (1, 0)))),
            rhs: Expression::Nonterminal("b".to_owned()).node_at(Span::from(((4, 0), (5, 0))))
        }
        .node_at(Span::from(((0, 0), (6, 0))))
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
        Production {
            lhs: "a".to_owned().node_at(Span::from(((0, 0), (1, 0)))),
            rhs: Expression::Empty.node_at(Span::from(((3, 0), (4, 0))))
        }
        .node_at(Span::from(((0, 0), (5, 0))))
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
                Production {
                    lhs: "a".to_owned().node_at(Span::from(((0, 0), (1, 0)))),
                    rhs: Expression::Alternative {
                        first: Box::new(
                            Expression::Terminal("d".to_owned())
                                .node_at(Span::from(((4, 0), (6, 0))))
                        ),
                        second: Box::new(
                            Expression::Repeated(Box::new(
                                Expression::Factor {
                                    count: 2.node_at(Span::from(((11, 0), (12, 0)))),
                                    primary: Box::new(
                                        Expression::Terminal("e".to_owned())
                                            .node_at(Span::from(((15, 0), (18, 0))))
                                    )
                                }
                                .node_at(Span::from(((11, 0), (18, 0))))
                            ))
                            .node_at(Span::from(((10, 0), (19, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .node_at(Span::from(((4, 0), (19, 0))))
                }
                .node_at(Span::from(((0, 0), (20, 0)))),
                Production {
                    lhs: "b".to_owned().node_at(Span::from(((33, 0), (34, 0)))),
                    rhs: Expression::Sequence {
                        first: Box::new(
                            Expression::Terminal("a".to_owned())
                                .node_at(Span::from(((37, 0), (40, 0))))
                        ),
                        second: Box::new(
                            Expression::Alternative {
                                first: Box::new(
                                    Expression::Nonterminal("a".to_owned())
                                        .node_at(Span::from(((43, 0), (44, 0))))
                                ),
                                second: Box::new(
                                    Expression::Terminal("c".to_owned())
                                        .node_at(Span::from(((47, 0), (50, 0))))
                                ),
                                rest: Vec::new(),
                            }
                            .node_at(Span::from(((42, 0), (51, 0))))
                        ),
                        rest: Vec::new(),
                    }
                    .node_at(Span::from(((37, 0), (51, 0))))
                }
                .node_at(Span::from(((33, 0), (52, 0))))
            ]
        }
        .node_at(Span::from(((0, 0), (52, 0))))
    );
}

use super::{Expression, Grammar, NodeAt, Production, Token, TokenKind, Tokens};
use nom::Slice;

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
        assert_eq!($parser(Span::new($input_str)), Err(Err::Error($error)));
    };
}

#[macro_export]
macro_rules! failure_case {
    ($parser:expr, $input_str:expr, $error:expr) => {
        assert_eq!($parser(Span::new($input_str)), Err(Err::Failure($error)));
    };
}

#[test]
fn test_factors() {
    use super::factor;

    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Terminal("terminal".to_owned()),
            1..11
        )],
        1,
        Expression::Terminal("terminal".to_owned()).node_at(1..11)
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Nonterminal("nonterminal".to_owned()),
            0..13
        )],
        1,
        Expression::Nonterminal("nonterminal".to_owned()).node_at(0..13)
    );
    ok_case!(
        factor,
        &vec![Token::new(
            TokenKind::Special(" special ".to_owned()),
            2..13
        )],
        1,
        Expression::Special(" special ".to_owned()).node_at(2..13)
    );
    ok_case!(factor, &vec![], 0, Expression::Empty.node_at(0..1));
    ok_case!(
        factor,
        &vec![
            Token::new(TokenKind::Integer(2), 0..1),
            Token::new(TokenKind::Repetition, 2..3),
            Token::new(TokenKind::Terminal("terminal".to_owned()), 4..14)
        ],
        3,
        Expression::Factor {
            count: 2.node_at(0..1),
            primary: Box::new(Expression::Terminal("terminal".to_owned()).node_at(4..14))
        }
        .node_at(0..14)
    );
    ok_case!(
        factor,
        &vec![
            Token::new(TokenKind::Integer(2), 0..1),
            Token::new(TokenKind::Terminal("terminal".to_owned()), 2..12)
        ],
        0,
        Expression::Empty.node_at(0..0)
    );
}

#[test]
fn test_terms() {
    use super::term;

    ok_case!(
        term,
        &vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 0..3),
            Token::new(TokenKind::Exception, 4..5),
            Token::new(TokenKind::Terminal("test".to_owned()), 6..12)
        ],
        3,
        Expression::Exception {
            subject: Box::new(Expression::Nonterminal("abc".to_owned()).node_at(0..3)),
            restriction: Box::new(Expression::Terminal("test".to_owned()).node_at(6..12)),
        }
        .node_at(0..12)
    );
    ok_case!(
        term,
        &vec![
            Token::new(TokenKind::Nonterminal("a".to_owned()), 0..1),
            Token::new(TokenKind::Exception, 1..2),
            Token::new(TokenKind::Nonterminal("b".to_owned()), 2..3),
            Token::new(TokenKind::Exception, 3..4),
            Token::new(TokenKind::Nonterminal("c".to_owned()), 4..5)
        ],
        3,
        Expression::Exception {
            subject: Box::new(Expression::Nonterminal("a".to_owned()).node_at(0..1)),
            restriction: Box::new(Expression::Nonterminal("b".to_owned()).node_at(2..3)),
        }
        .node_at(0..3)
    );
}

#[test]
fn test_sequences() {
    use super::sequence;

    ok_case!(
        sequence,
        &vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 0..3),
            Token::new(TokenKind::Concatenation, 3..4),
            Token::new(TokenKind::Terminal("test".to_owned()), 5..11),
            Token::new(TokenKind::Concatenation, 11..12),
            Token::new(TokenKind::Nonterminal("bca".to_owned()), 14..17)
        ],
        5,
        Expression::Sequence {
            first: Box::new(Expression::Nonterminal("abc".to_owned()).node_at(0..3)),
            second: Box::new(Expression::Terminal("test".to_owned()).node_at(5..11)),
            rest: vec![Expression::Nonterminal("bca".to_owned()).node_at(14..17)]
        }
        .node_at(0..17)
    );
}

#[test]
fn test_alternatives() {
    use super::alternative;

    ok_case!(
        alternative,
        &vec![
            Token::new(TokenKind::Nonterminal("a".to_owned()), 1..2),
            Token::new(TokenKind::Concatenation, 2..3),
            Token::new(TokenKind::Terminal("b".to_owned()), 4..7),
            Token::new(TokenKind::DefinitionSeparator, 8..9),
            Token::new(TokenKind::Terminal("c".to_owned()), 10..13),
            Token::new(TokenKind::Concatenation, 13..14),
            Token::new(TokenKind::Nonterminal("d".to_owned()), 15..16)
        ],
        7,
        Expression::Alternative {
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(Expression::Nonterminal("a".to_owned()).node_at(1..2)),
                    second: Box::new(Expression::Terminal("b".to_owned()).node_at(4..7)),
                    rest: Vec::new(),
                }
                .node_at(1..7)
            ),
            second: Box::new(
                Expression::Sequence {
                    first: Box::new(Expression::Terminal("c".to_owned()).node_at(10..13)),
                    second: Box::new(Expression::Nonterminal("d".to_owned()).node_at(15..16)),
                    rest: Vec::new(),
                }
                .node_at(10..16)
            ),
            rest: Vec::new(),
        }
        .node_at(1..16)
    );
}

#[test]
fn test_grouped() {
    use super::grouped;

    ok_case!(
        grouped,
        &vec![
            Token::new(TokenKind::StartGroup, 0..1),
            Token::new(TokenKind::Nonterminal("b".to_owned()), 1..2),
            Token::new(TokenKind::DefinitionSeparator, 3..4),
            Token::new(TokenKind::Nonterminal("c".to_owned()), 5..6),
            Token::new(TokenKind::EndGroup, 6..7)
        ],
        5,
        Expression::Alternative {
            first: Box::new(Expression::Nonterminal("b".to_owned()).node_at(1..2)),
            second: Box::new(Expression::Nonterminal("c".to_owned()).node_at(5..6)),
            rest: Vec::new(),
        }
        .node_at(0..7)
    );
    ok_case!(
        grouped,
        &vec![
            Token::new(TokenKind::StartGroup, 0..1),
            Token::new(TokenKind::Nonterminal("a".to_owned()), 2..3),
            Token::new(TokenKind::Concatenation, 3..4),
            Token::new(TokenKind::Terminal("b".to_owned()), 5..8),
            Token::new(TokenKind::DefinitionSeparator, 23..24),
            Token::new(TokenKind::Nonterminal("c".to_owned()), 25..26),
            Token::new(TokenKind::EndGroup, 27..28)
        ],
        7,
        Expression::Alternative {
            first: Box::new(
                Expression::Sequence {
                    first: Box::new(Expression::Nonterminal("a".to_owned()).node_at(2..3)),
                    second: Box::new(Expression::Terminal("b".to_owned()).node_at(5..8)),
                    rest: Vec::new(),
                }
                .node_at(2..8)
            ),
            second: Box::new(Expression::Nonterminal("c".to_owned()).node_at(25..26)),
            rest: Vec::new(),
        }
        .node_at(0..28)
    );
}

#[test]
fn test_repeated() {
    use super::repeated;

    ok_case!(
        repeated,
        &vec![
            Token::new(TokenKind::StartRepeat, 0..1),
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 1..4),
            Token::new(TokenKind::DefinitionSeparator, 10..11),
            Token::new(TokenKind::Nonterminal("def".to_owned()), 11..14),
            Token::new(TokenKind::EndRepeat, 15..16)
        ],
        5,
        Expression::Repeated(Box::new(
            Expression::Alternative {
                first: Box::new(Expression::Nonterminal("abc".to_owned()).node_at(1..4)),
                second: Box::new(Expression::Nonterminal("def".to_owned()).node_at(11..14)),
                rest: Vec::new(),
            }
            .node_at(1..14)
        ))
        .node_at(0..16)
    );
}

#[test]
fn test_optionals() {
    use super::optional;

    ok_case!(
        optional,
        &vec![
            Token::new(TokenKind::StartOption, 0..1),
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 2..5),
            Token::new(TokenKind::DefinitionSeparator, 5..6),
            Token::new(TokenKind::Nonterminal("def".to_owned()), 6..9),
            Token::new(TokenKind::EndOption, 19..20)
        ],
        5,
        Expression::Optional(Box::new(
            Expression::Alternative {
                first: Box::new(Expression::Nonterminal("abc".to_owned()).node_at(2..5)),
                second: Box::new(Expression::Nonterminal("def".to_owned()).node_at(6..9)),
                rest: Vec::new(),
            }
            .node_at(2..9)
        ))
        .node_at(0..20)
    );
}

#[test]
fn test_productions() {
    use super::production;

    ok_case!(
        production,
        &vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 0..3),
            Token::new(TokenKind::Definition, 4..5),
            Token::new(TokenKind::Terminal("a".to_owned()), 6..9),
            Token::new(TokenKind::Concatenation, 9..10),
            Token::new(TokenKind::StartGroup, 11..12),
            Token::new(TokenKind::Nonterminal("b".to_owned()), 12..13),
            Token::new(TokenKind::DefinitionSeparator, 14..15),
            Token::new(TokenKind::Terminal("c".to_owned()), 16..19),
            Token::new(TokenKind::EndGroup, 30..31),
            Token::new(TokenKind::Terminator, 31..32)
        ],
        10,
        Production {
            lhs: "abc".to_owned().node_at(0..3),
            rhs: Expression::Sequence {
                first: Box::new(Expression::Terminal("a".to_owned()).node_at(6..9)),
                second: Box::new(
                    Expression::Alternative {
                        first: Box::new(Expression::Nonterminal("b".to_owned()).node_at(12..13)),
                        second: Box::new(Expression::Terminal("c".to_owned()).node_at(16..19)),
                        rest: Vec::new(),
                    }
                    .node_at(11..31)
                ),
                rest: Vec::new(),
            }
            .node_at(6..31)
        }
        .node_at(0..32)
    );
}

#[test]
fn test_syntaxes() {
    use super::syntax;

    ok_case!(
        syntax,
        &vec![
            Token::new(TokenKind::Nonterminal("a".to_owned()), 0..1),
            Token::new(TokenKind::Definition, 2..3),
            Token::new(TokenKind::Terminal("d".to_owned()), 4..6),
            Token::new(TokenKind::DefinitionSeparator, 8..9),
            Token::new(TokenKind::StartRepeat, 10..11),
            Token::new(TokenKind::Integer(2), 11..12),
            Token::new(TokenKind::Repetition, 13..14),
            Token::new(TokenKind::Terminal("e".to_owned()), 15..18),
            Token::new(TokenKind::EndRepeat, 18..19),
            Token::new(TokenKind::Terminator, 19..20),
            Token::new(TokenKind::Nonterminal("b".to_owned()), 33..34),
            Token::new(TokenKind::Definition, 35..36),
            Token::new(TokenKind::Terminal("a".to_owned()), 37..40),
            Token::new(TokenKind::Concatenation, 40..41),
            Token::new(TokenKind::StartGroup, 42..43),
            Token::new(TokenKind::Nonterminal("a".to_owned()), 43..44),
            Token::new(TokenKind::DefinitionSeparator, 45..46),
            Token::new(TokenKind::Terminal("c".to_owned()), 47..50),
            Token::new(TokenKind::EndGroup, 50..51),
            Token::new(TokenKind::Terminator, 51..52),
        ],
        20,
        Grammar {
            productions: vec![
                Production {
                    lhs: "a".to_owned().node_at(0..1),
                    rhs: Expression::Alternative {
                        first: Box::new(Expression::Terminal("d".to_owned()).node_at(4..6)),
                        second: Box::new(
                            Expression::Repeated(Box::new(
                                Expression::Factor {
                                    count: 2.node_at(11..12),
                                    primary: Box::new(
                                        Expression::Terminal("e".to_owned()).node_at(15..18)
                                    )
                                }
                                .node_at(11..18)
                            ))
                            .node_at(10..19)
                        ),
                        rest: Vec::new(),
                    }
                    .node_at(4..19)
                }
                .node_at(0..20),
                Production {
                    lhs: "b".to_owned().node_at(33..34),
                    rhs: Expression::Sequence {
                        first: Box::new(Expression::Terminal("a".to_owned()).node_at(37..40)),
                        second: Box::new(
                            Expression::Alternative {
                                first: Box::new(
                                    Expression::Nonterminal("a".to_owned()).node_at(43..44)
                                ),
                                second: Box::new(
                                    Expression::Terminal("c".to_owned()).node_at(47..50)
                                ),
                                rest: Vec::new(),
                            }
                            .node_at(42..51)
                        ),
                        rest: Vec::new(),
                    }
                    .node_at(37..51)
                }
                .node_at(33..52)
            ]
        }
        .node_at(0..52)
    );
}

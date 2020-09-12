use super::{Expression, Grammar, Production};
use nom::{
    error::ErrorKind,
    Err::{Error, Failure},
};

#[test]
fn test_optional_gaps() {
    use super::optional_gap;

    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("   \t\t  test  "),
        Ok(("test  ", ()))
    );
    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("   \r "),
        Ok(("", ()))
    );
    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("  \r\n\r\r"),
        Ok(("", ()))
    );
    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("\x0c\x0b"),
        Ok(("", ()))
    );
    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("test  "),
        Ok(("test  ", ()))
    );
    assert_eq!(
        optional_gap::<(&str, ErrorKind)>("  (* comment *) test  "),
        Ok(("test  ", ()))
    );
}

#[test]
fn test_integers() {
    use super::integer;

    assert_eq!(integer::<(&str, ErrorKind)>("123"), Ok(("", 123)));
    assert_eq!(integer::<(&str, ErrorKind)>("12 3"), Ok(("", 123)));
    assert_eq!(integer::<(&str, ErrorKind)>("12 a"), Ok((" a", 12)));
    assert_eq!(integer::<(&str, ErrorKind)>("012test"), Ok(("test", 12)));
    assert_eq!(
        integer::<(&str, ErrorKind)>("test"),
        Err(Error(("test", ErrorKind::Char)))
    );
    assert_eq!(integer::<(&str, ErrorKind)>("123  "), Ok(("  ", 123)));
    assert_eq!(integer::<(&str, ErrorKind)>("1 2  3 "), Ok((" ", 123)));
}

#[test]
fn test_identifiers() {
    use super::identifier;

    assert_eq!(
        identifier::<(&str, ErrorKind)>("abc12"),
        Ok(("", "abc12".to_owned()))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("12abc"),
        Err(Error(("12abc", ErrorKind::Char)))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("_test"),
        Err(Error(("_test", ErrorKind::Char)))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("test abc"),
        Ok(("", "testabc".to_owned()))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("藏京٣¾  abc"),
        Ok(("", "藏京٣¾abc".to_owned()))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("  test"),
        Err(Error(("  test", ErrorKind::Char)))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("  test  abc"),
        Err(Error(("  test  abc", ErrorKind::Char)))
    );
    assert_eq!(
        identifier::<(&str, ErrorKind)>("test  5 "),
        Ok((" ", "test5".to_owned()))
    );
}

#[test]
fn test_specials() {
    use super::special;

    assert_eq!(
        special::<(&str, ErrorKind)>("? anything really ?"),
        Ok(("", Expression::Special("anythingreally".to_owned())))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("?藏!? abc"),
        Ok((" abc", Expression::Special("藏!".to_owned())))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("? not closed"),
        Err(Failure(("", ErrorKind::Char)))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("not opened ?"),
        Err(Error(("not opened ?", ErrorKind::Char)))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("? this has\na newline ?"),
        Ok(("", Expression::Special("thishasanewline".to_owned())))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("??"),
        Ok(("", Expression::Special("".to_owned())))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("? test (* comment *) ?"),
        Ok(("", Expression::Special("test(*comment*)".to_owned())))
    );
    assert_eq!(
        special::<(&str, ErrorKind)>("  ? test ?  "),
        Err(Error(("  ? test ?  ", ErrorKind::Char)))
    );
}

#[test]
fn test_terminals() {
    use super::terminal;

    assert_eq!(
        terminal::<(&str, ErrorKind)>("'a string'"),
        Ok(("", Expression::Terminal("a string".to_owned())))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("\"some other string  \"abc"),
        Ok(("abc", Expression::Terminal("some other string  ".to_owned())))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("\"not closed"),
        Err(Failure(("", ErrorKind::Char)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("'not closed"),
        Err(Failure(("", ErrorKind::Char)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("not opened'"),
        Err(Error(("not opened'", ErrorKind::Char)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("'this has\na newline'abc"),
        Err(Failure(("\na newline'abc", ErrorKind::Char)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("\"this has\na newline\"abc"),
        Err(Failure(("\na newline\"abc", ErrorKind::Char)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("\"\""),
        Err(Failure(("\"", ErrorKind::TakeTill1)))
    );
    assert_eq!(
        terminal::<(&str, ErrorKind)>("  'a string'  "),
        Err(Error(("  'a string'  ", ErrorKind::Char)))
    );
}

#[test]
fn test_factors() {
    use super::factor;

    assert_eq!(
        factor::<(&str, ErrorKind)>("'terminal'"),
        Ok(("", Expression::Terminal("terminal".to_owned())))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>("nonterminal"),
        Ok(("", Expression::Nonterminal("nonterminal".to_owned())))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>("? special ?"),
        Ok(("", Expression::Special("special".to_owned())))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>(""),
        Ok(("", Expression::Empty))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>("2 * 'terminal'"),
        Ok((
            "",
            Expression::Factor {
                count: 2,
                primary: Box::new(Expression::Terminal("terminal".to_owned()))
            }
        ))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>(" 3* a "),
        Ok((
            "",
            Expression::Factor{
                count: 3,
                primary: Box::new(Expression::Nonterminal("a".to_owned()))
            }
        ))
    );
    assert_eq!(
        factor::<(&str, ErrorKind)>(" 3 b "),
        Ok(("3 b ", Expression::Empty))
    );
}

#[test]
fn test_comments() {
    use super::comment;

    assert_eq!(
        comment::<(&str, ErrorKind)>("(* comment *)"),
        Ok(("", ()))
    );
    assert_eq!(
        comment::<(&str, ErrorKind)>("(* (* nested *) *)  "),
        Ok(("  ", ()))
    );
    assert_eq!(
        comment::<(&str, ErrorKind)>("(*aa (* bb *) cc(*d*)*)fg"),
        Ok(("fg", ()))
    );
    assert_eq!(
        comment::<(&str, ErrorKind)>("(* not closed "),
        Err(Failure(("", ErrorKind::Char)))
    );
}

#[test]
fn test_terms() {
    use super::term;

    assert_eq!(
        term::<(&str, ErrorKind)>("abc - 'test'"),
        Ok((
            "",
            Expression::Exception {
                subject: Box::new(Expression::Nonterminal("abc".to_owned())),
                restriction: Box::new(Expression::Terminal("test".to_owned())),
            }
        ))
    );
    assert_eq!(
        term::<(&str, ErrorKind)>("a-b-c"),
        Ok((
            "-c",
            Expression::Exception {
                subject: Box::new(Expression::Nonterminal("a".to_owned())),
                restriction: Box::new(Expression::Nonterminal("b".to_owned())),
            }
        ))
    );
}

#[test]
fn test_sequences() {
    use super::sequence;

    assert_eq!(
        sequence::<(&str, ErrorKind)>("abc, 'test', bca"),
        Ok((
            "",
            Expression::Sequence {
                first: Box::new(Expression::Nonterminal("abc".to_owned())),
                second: Box::new(Expression::Terminal("test".to_owned())),
                rest: vec![Expression::Nonterminal("bca".to_owned())]
            }
        ))
    );
}

#[test]
fn test_alternatives() {
    use super::alternative;

    assert_eq!(
        alternative::<(&str, ErrorKind)>(" a, 'b' | 'c', d "),
        Ok((
            "",
            Expression::Alternative {
                first: Box::new(Expression::Sequence {
                    first: Box::new(Expression::Nonterminal("a".to_owned())),
                    second: Box::new(Expression::Terminal("b".to_owned())),
                    rest: Vec::new(),
                }),
                second: Box::new(Expression::Sequence {
                    first: Box::new(Expression::Terminal("c".to_owned())),
                    second: Box::new(Expression::Nonterminal("d".to_owned())),
                    rest: Vec::new(),
                }),
                rest: Vec::new(),
            }
        ))
    );
}

#[test]
fn test_grouped() {
    use super::grouped;

    assert_eq!(
        grouped::<(&str, ErrorKind)>("(b | c)"),
        Ok((
            "",
            Expression::Alternative {
                first: Box::new(Expression::Nonterminal("b".to_owned())),
                second: Box::new(Expression::Nonterminal("c".to_owned())),
                rest: Vec::new(),
            }
        ))
    );
    assert_eq!(
        grouped::<(&str, ErrorKind)>("( a, 'b' (* comment *) | c )"),
        Ok((
            "",
            Expression::Alternative {
                first: Box::new(Expression::Sequence {
                    first: Box::new(Expression::Nonterminal("a".to_owned())),
                    second: Box::new(Expression::Terminal("b".to_owned())),
                    rest: Vec::new(),
                }),
                second: Box::new(Expression::Nonterminal("c".to_owned())),
                rest: Vec::new(),
            }
        ))
    );
}

#[test]
fn test_repeated() {
    use super::repeated;

    assert_eq!(
        repeated::<(&str, ErrorKind)>("{abc (**) |def}"),
        Ok((
            "",
            Expression::Repeated(Box::new(Expression::Alternative {
                first: Box::new(Expression::Nonterminal("abc".to_owned())),
                second: Box::new(Expression::Nonterminal("def".to_owned())),
                rest: Vec::new(),
            }))
        ))
    );
}

#[test]
fn test_optionals() {
    use super::optional;

    assert_eq!(
        optional::<(&str, ErrorKind)>("[ abc|def (*test*) ]"),
        Ok((
            "",
            Expression::Optional(Box::new(Expression::Alternative {
                first: Box::new(Expression::Nonterminal("abc".to_owned())),
                second: Box::new(Expression::Nonterminal("def".to_owned())),
                rest: Vec::new(),
            }))
        ))
    );
}

#[test]
fn test_productions() {
    use super::production;

    assert_eq!(
        production::<(&str, ErrorKind)>("abc = 'a', (b | 'c' (* test *)); "),
        Ok((
            "",
            Production {
                lhs: "abc".to_owned(),
                rhs: Expression::Sequence {
                    first: Box::new(Expression::Terminal("a".to_owned())),
                    second: Box::new(Expression::Alternative {
                        first: Box::new(Expression::Nonterminal("b".to_owned())),
                        second: Box::new(Expression::Terminal("c".to_owned())),
                        rest: Vec::new(),
                    }),
                    rest: Vec::new(),
                }
            }
        ))
    );
}

#[test]
fn test_syntaxes() {
    use super::syntax;

    assert_eq!(
        syntax::<(&str, ErrorKind)>("a = 'd' | {2 * 'e'}; (* test *)\nb = 'a', (a | 'c');\n"),
        Ok((
            "",
            Grammar {
                productions: vec![
                    Production {
                        lhs: "a".to_owned(),
                        rhs: Expression::Alternative {
                            first: Box::new(Expression::Terminal("d".to_owned())),
                            second: Box::new(Expression::Repeated(Box::new(Expression::Factor {
                                count: 2,
                                primary: Box::new(Expression::Terminal("e".to_owned()))
                            }))),
                            rest: Vec::new(),
                        }
                    },
                    Production {
                        lhs: "b".to_owned(),
                        rhs: Expression::Sequence {
                            first: Box::new(Expression::Terminal("a".to_owned())),
                            second: Box::new(Expression::Alternative {
                                first: Box::new(Expression::Nonterminal("a".to_owned())),
                                second: Box::new(Expression::Terminal("c".to_owned())),
                                rest: Vec::new(),
                            }),
                            rest: Vec::new(),
                        }
                    }
                ]
            }
        ))
    );
}

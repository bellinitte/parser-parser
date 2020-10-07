use super::{lex, Error, ErrorKind, Symbol, Token, TokenKind, Span};

#[test]
fn test_concatenation() {
    assert_eq!(
        lex(","),
        Ok(vec![Token::new(TokenKind::Concatenation, Span::from(((0, 0), (1, 0))))])
    );
}

#[test]
fn test_definition_separators() {
    assert_eq!(
        lex("| /!"),
        Ok(vec![
            Token::new(TokenKind::DefinitionSeparator, Span::from(((0, 0), (1, 0)))),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((2, 0), (3, 0)))),
            Token::new(TokenKind::DefinitionSeparator, Span::from(((3, 0), (4, 0))))
        ])
    );
}

#[test]
fn test_definitions() {
    assert_eq!(
        lex("abc = b;"),
        Ok(vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), Span::from(((0, 0), (3, 0)))),
            Token::new(TokenKind::Definition, Span::from(((4, 0), (5, 0)))),
            Token::new(TokenKind::Nonterminal("b".to_owned()), Span::from(((6, 0), (7, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((7, 0), (8, 0))))
        ])
    );
}

#[test]
fn test_options() {
    assert_eq!(
        lex(" (/ [ /) ]"),
        Ok(vec![
            Token::new(TokenKind::StartOption, Span::from(((1, 0), (3, 0)))),
            Token::new(TokenKind::StartOption, Span::from(((4, 0), (5, 0)))),
            Token::new(TokenKind::EndOption, Span::from(((6, 0), (8, 0)))),
            Token::new(TokenKind::EndOption, Span::from(((9, 0), (10, 0))))
        ])
    );
    assert_eq!(
        lex(" (/) "),
        Err(Error {
            kind: ErrorKind::InvalidSymbol("(/)".to_owned()),
            span: Span::from(((1, 0), (4, 0))),
        })
    );
    assert_eq!(
        lex(" /"),
        Ok(vec![Token::new(TokenKind::DefinitionSeparator, Span::from(((1, 0), (2, 0)))),])
    );
}

#[test]
fn test_repeats() {
    assert_eq!(
        lex("(::) { } "),
        Ok(vec![
            Token::new(TokenKind::StartRepeat, Span::from(((0, 0), (2, 0)))),
            Token::new(TokenKind::EndRepeat, Span::from(((2, 0), (4, 0)))),
            Token::new(TokenKind::StartRepeat, Span::from(((5, 0), (6, 0)))),
            Token::new(TokenKind::EndRepeat, Span::from(((7, 0), (8, 0))))
        ])
    );
    assert_eq!(
        lex(" (:) "),
        Err(Error {
            kind: ErrorKind::InvalidSymbol("(:)".to_owned()),
            span: Span::from(((1, 0), (4, 0))),
        })
    );
}

#[test]
fn test_terminals() {
    assert_eq!(
        lex(" \"ab c \" "),
        Ok(vec![Token::new(
            TokenKind::Terminal("ab c ".to_owned()),
            Span::from(((1, 0), (8, 0)))
        )])
    );
    assert_eq!(
        lex("  '\"aba' "),
        Ok(vec![Token::new(
            TokenKind::Terminal("\"aba".to_owned()),
            Span::from(((2, 0), (8, 0)))
        )])
    );
    assert_eq!(
        lex(" ' a \""),
        Err(Error {
            kind: ErrorKind::UnterminatedTerminal,
            span: Span::from(((5, 0), (6, 0))),
        })
    );
    assert_eq!(
        lex("\"bbb'   "),
        Err(Error {
            kind: ErrorKind::UnterminatedTerminal,
            span: Span::from(((7, 0), (8, 0))),
        })
    );
    assert_eq!(
        lex("\"\""),
        Err(Error {
            kind: ErrorKind::EmptyTerminal,
            span: Span::from(((0, 0), (2, 0))),
        })
    );
    assert_eq!(
        lex("''"),
        Err(Error {
            kind: ErrorKind::EmptyTerminal,
            span: Span::from(((0, 0), (2, 0))),
        })
    );
    //     ok_case!(
    //         terminal,
    //         "'a string'",
    //         10,
    //         Expression::Terminal("a string".to_owned()).token_at(Span::from(((0, 0), (10, 0))))
    //     );
    //     ok_case!(
    //         terminal,
    //         "\"some other string  \"abc",
    //         21,
    //         Expression::Terminal("some other string  ".to_owned()).token_at(Span::from(((0, 0), (21, 0))))
    //     );
    //     failure_case!(terminal, "\"not closed", Error::Internal(ErrorKind::Char));
    //     failure_case!(terminal, "'not closed", Error::Internal(ErrorKind::Char));
    //     error_case!(terminal, "not opened'", Error::Internal(ErrorKind::Char));
    //     failure_case!(
    //         terminal,
    //         "'this has\na newline'abc",
    //         Error::Internal(ErrorKind::Char)
    //     );
    //     failure_case!(
    //         terminal,
    //         "\"this has\na newline\"abc",
    //         Error::Internal(ErrorKind::Char)
    //     );
    //     failure_case!(terminal, "\"\"", Error::Internal(ErrorKind::TakeTill1));
    //     error_case!(terminal, "  'a string'  ", Error::Internal(ErrorKind::Char));
}

#[test]
fn test_specials() {
    assert_eq!(
        lex(" ? test ?"),
        Ok(vec![Token::new(
            TokenKind::Special(" test ".to_owned()),
            Span::from(((1, 0), (9, 0)))
        )])
    );
    assert_eq!(
        lex("?a\nbc?  "),
        Ok(vec![Token::new(
            TokenKind::Special("a\nbc".to_owned()),
            Span::from(((0, 0), (3, 1)))
        )])
    );
    assert_eq!(
        lex(" ?bbb  "),
        Err(Error {
            kind: ErrorKind::UnterminatedSpecial,
            span: Span::from(((6, 0), (7, 0))),
        })
    );
    assert_eq!(
        lex("??"),
        Ok(vec![Token::new(TokenKind::Special("".to_owned()), Span::from(((0, 0), (2, 0))))])
    );
    //     ok_case!(
    //         special,
    //         "? anything really ?",
    //         19,
    //         Expression::Special("anythingreally".to_owned()).token_at(Span::from(((0, 0), (19, 0))))
    //     );
    //     ok_case!(
    //         special,
    //         "?藏!? abc",
    //         6,
    //         Expression::Special("藏!".to_owned()).token_at(Span::from(((0, 0), (6, 0))))
    //     );
    //     failure_case!(special, "? not closed", Error::UnterminatedSpecial);
    //     error_case!(special, "not opened ?", Error::Internal(ErrorKind::Char));
    //     ok_case!(
    //         special,
    //         "? this has\na newline ?",
    //         22,
    //         Expression::Special("thishasanewline".to_owned()).token_at(Span::from(((0, 0), (22, 0))))
    //     );
    //     ok_case!(
    //         special,
    //         "??",
    //         2,
    //         Expression::Special("".to_owned()).token_at(Span::from(((0, 0), (2, 0))))
    //     );
    //     ok_case!(
    //         special,
    //         "? test (* comment *) ?",
    //         22,
    //         Expression::Special("test(*comment*)".to_owned()).token_at(Span::from(((0, 0), (22, 0))))
    //     );
    //     error_case!(special, "  ? test ?  ", Error::Internal(ErrorKind::Char));
}

#[test]
fn test_integers() {
    assert_eq!(
        lex(" 123 "),
        Ok(vec![Token::new(TokenKind::Integer(123), Span::from(((1, 0), (4, 0))))])
    );
    assert_eq!(
        lex(" 1 2  3  "),
        Ok(vec![Token::new(TokenKind::Integer(123), Span::from(((1, 0), (7, 0))))])
    );
    assert_eq!(
        lex(" 01234 56"),
        Ok(vec![Token::new(TokenKind::Integer(123456), Span::from(((1, 0), (9, 0))))])
    );
    assert_eq!(
        lex(" 0 "),
        Ok(vec![Token::new(TokenKind::Integer(0), Span::from(((1, 0), (2, 0))))])
    );
    // ok_case!(integer, "123", 3, 123.token_at(0..3));
    // ok_case!(integer, "12 3", 4, 123.token_at(0..4));
    // ok_case!(integer, "12 a", 2, 12.token_at(0..2));
    // ok_case!(integer, "012test", 3, 12.token_at(0..3));
    // error_case!(integer, "test", Error::Internal(ErrorKind::Char));
    // ok_case!(integer, "123  ", 3, 123.token_at(0..3));
    // ok_case!(integer, "1 2  3 ", 6, 123.token_at(0..6));
}

#[test]
fn test_nonterminals() {
    assert_eq!(
        lex(" abc "),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc".to_owned()),
            Span::from(((1, 0), (4, 0)))
        )])
    );
    assert_eq!(
        lex("a  bc "),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc".to_owned()),
            Span::from(((0, 0), (5, 0)))
        )])
    );
    assert_eq!(
        lex("abc12 3 "),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc123".to_owned()),
            Span::from(((0, 0), (7, 0)))
        )])
    );
    assert_eq!(
        lex(" x "),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("x".to_owned()),
            Span::from(((1, 0), (2, 0)))
        )])
    );
    //     ok_case!(identifier, "abc12", 5, "abc12".to_owned().token_at(0..5));
    //     error_case!(identifier, "12abc", Error::Internal(ErrorKind::Char));
    //     error_case!(identifier, "_test", Error::Internal(ErrorKind::Char));
    //     ok_case!(
    //         identifier,
    //         "test abc",
    //         8,
    //         "testabc".to_owned().token_at(0..8)
    //     );
    //     ok_case!(
    //         identifier,
    //         "藏京٣¾  abc",
    //         15,
    //         "藏京٣¾abc".to_owned().token_at(0..15)
    //     );
    //     error_case!(identifier, "  test", Error::Internal(ErrorKind::Char));
    //     error_case!(identifier, "  test  abc", Error::Internal(ErrorKind::Char));
    //     ok_case!(identifier, "test  5 ", 7, "test5".to_owned().token_at(0..7));
}

#[test]
fn test_invalid_symbols() {
    assert_eq!(
        lex(" + "),
        Err(Error {
            kind: ErrorKind::InvalidSymbol('+'.to_string()),
            span: Span::from(((1, 0), (2, 0))),
        })
    );
}

#[test]
fn test_whitespace() {
    assert_eq!(
        lex("  , \n,"),
        Ok(vec![
            Token::new(TokenKind::Concatenation, Span::from(((2, 0), (3, 0)))),
            Token::new(TokenKind::Concatenation, Span::from(((0, 1), (1, 1))))
        ])
    );
}

#[test]
fn test_comments() {
    assert_eq!(lex(" (* test *) "), Ok(vec![]));
    assert_eq!(
        lex(" (* test * "),
        Err(Error {
            kind: ErrorKind::UnterminatedComment,
            span: Span::from(((10, 0), (11, 0))),
        })
    );
    assert_eq!(
        lex(" (* ("),
        Err(Error {
            kind: ErrorKind::UnterminatedComment,
            span: Span::from(((4, 0), (5, 0))),
        })
    );
    assert_eq!(
        lex(", (*, *) , "),
        Ok(vec![
            Token::new(TokenKind::Concatenation, Span::from(((0, 0), (1, 0)))),
            Token::new(TokenKind::Concatenation, Span::from(((9, 0), (10, 0))))
        ])
    );
    assert_eq!(
        lex(" ,(*, (* ,*) ,*) , ,"),
        Ok(vec![
            Token::new(TokenKind::Concatenation, Span::from(((1, 0), (2, 0)))),
            Token::new(TokenKind::Concatenation, Span::from(((17, 0), (18, 0)))),
            Token::new(TokenKind::Concatenation, Span::from(((19, 0), (20, 0)))),
        ])
    );
    assert_eq!(
        lex(" (* (* *) "),
        Err(Error {
            kind: ErrorKind::UnterminatedComment,
            span: Span::from(((9, 0), (10, 0))),
        })
    );
    assert_eq!(
        lex(" (*) "),
        Err(Error {
            kind: ErrorKind::InvalidSymbol("(*)".to_owned()),
            span: Span::from(((1, 0), (4, 0))),
        })
    );
}

#[test]
fn test_multiline() {
    assert_eq!(
        lex(" abc \n = 'def' "),
        Ok(vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), Span::from(((1, 0), (4, 0)))),
            Token::new(TokenKind::Definition, Span::from(((1, 1), (2, 1)))),
            Token::new(TokenKind::Terminal("def".to_owned()), Span::from(((3, 1), (8, 1))))
        ])
    );
}

#[test]
fn test_combinations() {}

#[test]
fn test_multiple_unicode_code_points() {
    assert_eq!(
        lex("aéf = abc;"),
        Ok(vec![
            Token::new(TokenKind::Nonterminal("aéf".to_owned()), Span::from(((0, 0), (4, 0)))),
            Token::new(TokenKind::Definition, Span::from(((5, 0), (6, 0)))),
            Token::new(TokenKind::Nonterminal("abc".to_owned()), Span::from(((7, 0), (10, 0)))),
            Token::new(TokenKind::Terminator, Span::from(((10, 0), (11, 0)))),
        ])
    );
}

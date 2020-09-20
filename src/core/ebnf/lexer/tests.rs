use super::{lex, Error, ErrorKind, Symbol, Token, TokenKind};

fn regular<'a>(s: &'a str) -> Vec<Symbol> {
    (0..).zip(s.chars()).collect()
}

#[test]
fn test_concatenation() {
    assert_eq!(
        lex(&regular(",")),
        Ok(vec![Token::new(TokenKind::Concatenation, 0..1)])
    );
}

#[test]
fn test_definition_separators() {
    assert_eq!(
        lex(&regular("| /!")),
        Ok(vec![
            Token::new(TokenKind::DefinitionSeparator, 0..1),
            Token::new(TokenKind::DefinitionSeparator, 2..3),
            Token::new(TokenKind::DefinitionSeparator, 3..4)
        ])
    );
}

#[test]
fn test_definitions() {
    assert_eq!(
        lex(&regular("abc = b;")),
        Ok(vec![
            Token::new(TokenKind::Nonterminal("abc".to_owned()), 0..3),
            Token::new(TokenKind::Definition, 4..5),
            Token::new(TokenKind::Nonterminal("b".to_owned()), 6..7),
            Token::new(TokenKind::Terminator, 7..8)
        ])
    );
}

#[test]
fn test_options() {
    assert_eq!(
        lex(&regular(" (/ [ /) ]")),
        Ok(vec![
            Token::new(TokenKind::StartOption, 1..3),
            Token::new(TokenKind::StartOption, 4..5),
            Token::new(TokenKind::EndOption, 6..8),
            Token::new(TokenKind::EndOption, 9..10)
        ])
    );
}

#[test]
fn test_repeats() {
    assert_eq!(
        lex(&regular("(::) { } ")),
        Ok(vec![
            Token::new(TokenKind::StartRepeat, 0..2),
            Token::new(TokenKind::EndRepeat, 2..4),
            Token::new(TokenKind::StartRepeat, 5..6),
            Token::new(TokenKind::EndRepeat, 7..8)
        ])
    );
}

#[test]
fn test_terminals() {
    assert_eq!(
        lex(&regular(" \"ab c \" ")),
        Ok(vec![Token::new(
            TokenKind::Terminal("ab c ".to_owned()),
            1..8
        )])
    );
    assert_eq!(
        lex(&regular("  '\"aba' ")),
        Ok(vec![Token::new(
            TokenKind::Terminal("\"aba".to_owned()),
            2..8
        )])
    );
    assert_eq!(
        lex(&regular(" ' a \"")),
        Err(Error {
            kind: ErrorKind::UnterminatedTerminal,
            position: 5..6,
        })
    );
    assert_eq!(
        lex(&regular("\"bbb'   ")),
        Err(Error {
            kind: ErrorKind::UnterminatedTerminal,
            position: 7..8,
        })
    );
    assert_eq!(
        lex(&regular("\"\"")),
        Err(Error {
            kind: ErrorKind::EmptyTerminal,
            position: 0..2,
        })
    );
    assert_eq!(
        lex(&regular("''")),
        Err(Error {
            kind: ErrorKind::EmptyTerminal,
            position: 0..2,
        })
    );
    //     ok_case!(
    //         terminal,
    //         "'a string'",
    //         10,
    //         Expression::Terminal("a string".to_owned()).token_at(0..10)
    //     );
    //     ok_case!(
    //         terminal,
    //         "\"some other string  \"abc",
    //         21,
    //         Expression::Terminal("some other string  ".to_owned()).token_at(0..21)
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
        lex(&regular(" ? test ?")),
        Ok(vec![Token::new(
            TokenKind::Special(" test ".to_owned()),
            1..9
        )])
    );
    assert_eq!(
        lex(&regular("?a\nbc?  ")),
        Ok(vec![Token::new(
            TokenKind::Special("a\nbc".to_owned()),
            0..6
        )])
    );
    assert_eq!(
        lex(&regular(" ?bbb  ")),
        Err(Error {
            kind: ErrorKind::UnterminatedSpecial,
            position: 6..7,
        })
    );
    assert_eq!(
        lex(&regular("??")),
        Ok(vec![Token::new(TokenKind::Special("".to_owned()), 0..2)])
    );
    //     ok_case!(
    //         special,
    //         "? anything really ?",
    //         19,
    //         Expression::Special("anythingreally".to_owned()).token_at(0..19)
    //     );
    //     ok_case!(
    //         special,
    //         "?藏!? abc",
    //         6,
    //         Expression::Special("藏!".to_owned()).token_at(0..6)
    //     );
    //     failure_case!(special, "? not closed", Error::UnterminatedSpecial);
    //     error_case!(special, "not opened ?", Error::Internal(ErrorKind::Char));
    //     ok_case!(
    //         special,
    //         "? this has\na newline ?",
    //         22,
    //         Expression::Special("thishasanewline".to_owned()).token_at(0..22)
    //     );
    //     ok_case!(
    //         special,
    //         "??",
    //         2,
    //         Expression::Special("".to_owned()).token_at(0..2)
    //     );
    //     ok_case!(
    //         special,
    //         "? test (* comment *) ?",
    //         22,
    //         Expression::Special("test(*comment*)".to_owned()).token_at(0..22)
    //     );
    //     error_case!(special, "  ? test ?  ", Error::Internal(ErrorKind::Char));
}

#[test]
fn test_integers() {
    assert_eq!(
        lex(&regular(" 123 ")),
        Ok(vec![Token::new(TokenKind::Integer(123), 1..4)])
    );
    assert_eq!(
        lex(&regular(" 1 2  3  ")),
        Ok(vec![Token::new(TokenKind::Integer(123), 1..7)])
    );
    assert_eq!(
        lex(&regular(" 01234 56")),
        Ok(vec![Token::new(TokenKind::Integer(123456), 1..9)])
    );
    assert_eq!(
        lex(&regular(" 0 ")),
        Ok(vec![Token::new(TokenKind::Integer(0), 1..2)])
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
        lex(&regular(" abc ")),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc".to_owned()),
            1..4
        )])
    );
    assert_eq!(
        lex(&regular("a  bc ")),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc".to_owned()),
            0..5
        )])
    );
    assert_eq!(
        lex(&regular("abc12 3 ")),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("abc123".to_owned()),
            0..7
        )])
    );
    assert_eq!(
        lex(&regular(" x ")),
        Ok(vec![Token::new(
            TokenKind::Nonterminal("x".to_owned()),
            1..2
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
        lex(&regular(" + ")),
        Err(Error {
            kind: ErrorKind::InvalidSymbol('+'),
            position: 1..2,
        })
    );
}

#[test]
fn test_whitespace() {
    assert_eq!(
        lex(&regular("  , \n,")),
        Ok(vec![
            Token::new(TokenKind::Concatenation, 2..3),
            Token::new(TokenKind::Concatenation, 5..6)
        ])
    );
}

#[test]
fn test_comments() {
    assert_eq!(
        lex(&regular(", (*, *) , ")),
        Ok(vec![
            Token::new(TokenKind::Concatenation, 0..1),
            Token::new(TokenKind::Concatenation, 9..10)
        ])
    );
    assert_eq!(
        lex(&regular(" ,(*, (* ,*) ,*) , ,")),
        Ok(vec![
            Token::new(TokenKind::Concatenation, 1..2),
            Token::new(TokenKind::Concatenation, 17..18),
            Token::new(TokenKind::Concatenation, 19..20),
        ])
    );
    assert_eq!(
        lex(&regular(" (* (* *) ")),
        Err(Error {
            kind: ErrorKind::UnterminatedComment,
            position: 9..10,
        })
    );
    assert_eq!(
        lex(&regular(" (*) ")),
        Err(Error {
            kind: ErrorKind::AmbiguousSymbol,
            position: 1..4,
        })
    );
}

#[test]
fn test_combinations() {}

#[test]
fn test_unicode() {}

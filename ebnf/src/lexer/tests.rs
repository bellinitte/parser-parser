use super::{lex, scan, Error, Span, Spanning, Token};
use quickcheck_macros::quickcheck;

#[test]
fn test_scan_control_characters() {
    use std::str;

    assert_eq!(
        scan(str::from_utf8(&[0x0a, 0x0d]).unwrap()),
        Ok(vec![
            "\n".spanning(Span::from(((0, 0), (0, 1)))),
            "\r".spanning(Span::from(((0, 1), (0, 2))))
        ])
    );
}

#[test]
fn test_scan_multiline() {
    assert_eq!(
        scan(" abc \n = def "),
        Ok(vec![
            " ".spanning(Span::from(((0, 0), (1, 0)))),
            "a".spanning(Span::from(((1, 0), (2, 0)))),
            "b".spanning(Span::from(((2, 0), (3, 0)))),
            "c".spanning(Span::from(((3, 0), (4, 0)))),
            " ".spanning(Span::from(((4, 0), (5, 0)))),
            "\n".spanning(Span::from(((5, 0), (0, 1)))),
            " ".spanning(Span::from(((0, 1), (1, 1)))),
            "=".spanning(Span::from(((1, 1), (2, 1)))),
            " ".spanning(Span::from(((2, 1), (3, 1)))),
            "d".spanning(Span::from(((3, 1), (4, 1)))),
            "e".spanning(Span::from(((4, 1), (5, 1)))),
            "f".spanning(Span::from(((5, 1), (6, 1)))),
            " ".spanning(Span::from(((6, 1), (7, 1)))),
        ])
    );
}

#[test]
fn test_scan_multiple_unicode_code_points() {
    assert_eq!(
        scan("aéf = abc;"),
        Ok(vec![
            "a".spanning(Span::from(((0, 0), (1, 0)))),
            "e\u{301}".spanning(Span::from(((1, 0), (3, 0)))),
            "f".spanning(Span::from(((3, 0), (4, 0)))),
            " ".spanning(Span::from(((4, 0), (5, 0)))),
            "=".spanning(Span::from(((5, 0), (6, 0)))),
            " ".spanning(Span::from(((6, 0), (7, 0)))),
            "a".spanning(Span::from(((7, 0), (8, 0)))),
            "b".spanning(Span::from(((8, 0), (9, 0)))),
            "c".spanning(Span::from(((9, 0), (10, 0)))),
            ";".spanning(Span::from(((10, 0), (11, 0)))),
        ])
    );
}

#[test]
fn test_concatenation() {
    assert_eq!(
        lex(","),
        Ok(vec![
            Token::Concatenation.spanning(Span::from(((0, 0), (1, 0))))
        ])
    );
}

#[test]
fn test_definition_separators() {
    assert_eq!(
        lex("| /!"),
        Ok(vec![
            Token::DefinitionSeparator.spanning(Span::from(((0, 0), (1, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((2, 0), (3, 0)))),
            Token::DefinitionSeparator.spanning(Span::from(((3, 0), (4, 0))))
        ])
    );
}

#[test]
fn test_definitions() {
    assert_eq!(
        lex("abc = b;"),
        Ok(vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (3, 0)))),
            Token::Definition.spanning(Span::from(((4, 0), (5, 0)))),
            Token::Nonterminal("b".to_owned()).spanning(Span::from(((6, 0), (7, 0)))),
            Token::Terminator.spanning(Span::from(((7, 0), (8, 0))))
        ])
    );
}

#[test]
fn test_options() {
    assert_eq!(
        lex(" (/ [ /) ]"),
        Ok(vec![
            Token::StartOption.spanning(Span::from(((1, 0), (3, 0)))),
            Token::StartOption.spanning(Span::from(((4, 0), (5, 0)))),
            Token::EndOption.spanning(Span::from(((6, 0), (8, 0)))),
            Token::EndOption.spanning(Span::from(((9, 0), (10, 0))))
        ])
    );
    assert_eq!(
        lex(" (/) "),
        Err(Error::InvalidSymbol("(/)".to_owned()).spanning(Span::from(((1, 0), (4, 0)))))
    );
    assert_eq!(
        lex(" /"),
        Ok(vec![
            Token::DefinitionSeparator.spanning(Span::from(((1, 0), (2, 0)))),
        ])
    );
}

#[test]
fn test_repeats() {
    assert_eq!(
        lex("(::) { } "),
        Ok(vec![
            Token::StartRepeat.spanning(Span::from(((0, 0), (2, 0)))),
            Token::EndRepeat.spanning(Span::from(((2, 0), (4, 0)))),
            Token::StartRepeat.spanning(Span::from(((5, 0), (6, 0)))),
            Token::EndRepeat.spanning(Span::from(((7, 0), (8, 0))))
        ])
    );
    assert_eq!(
        lex(" (:) "),
        Err(Error::InvalidSymbol("(:)".to_owned()).spanning(Span::from(((1, 0), (4, 0)))))
    );
}

#[test]
fn test_terminals() {
    assert_eq!(
        lex(" \"ab c \" "),
        Ok(vec![
            Token::Terminal("ab c ".to_owned()).spanning(Span::from(((1, 0), (8, 0))))
        ])
    );
    assert_eq!(
        lex("  '\"aba' "),
        Ok(vec![
            Token::Terminal("\"aba".to_owned()).spanning(Span::from(((2, 0), (8, 0))))
        ])
    );
    assert_eq!(
        lex(" ' a \""),
        Err(Error::UnterminatedTerminal.spanning(Span::from(((5, 0), (6, 0)))))
    );
    assert_eq!(
        lex("\"bbb'   "),
        Err(Error::UnterminatedTerminal.spanning(Span::from(((7, 0), (8, 0)))))
    );
    assert_eq!(
        lex("\"\""),
        Err(Error::EmptyTerminal.spanning(Span::from(((0, 0), (2, 0)))))
    );
    assert_eq!(
        lex("''"),
        Err(Error::EmptyTerminal.spanning(Span::from(((0, 0), (2, 0)))))
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
        Ok(vec![
            Token::Special(" test ".to_owned()).spanning(Span::from(((1, 0), (9, 0))))
        ])
    );
    assert_eq!(
        lex("?a\nbc?  "),
        Ok(vec![
            Token::Special("a\nbc".to_owned()).spanning(Span::from(((0, 0), (3, 1))))
        ])
    );
    assert_eq!(
        lex(" ?bbb  "),
        Err(Error::UnterminatedSpecial.spanning(Span::from(((6, 0), (7, 0)))))
    );
    assert_eq!(
        lex("??"),
        Ok(vec![
            Token::Special("".to_owned()).spanning(Span::from(((0, 0), (2, 0))))
        ])
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
        Ok(vec![
            Token::Integer(123).spanning(Span::from(((1, 0), (4, 0))))
        ])
    );
    assert_eq!(
        lex(" 1 2  3  "),
        Ok(vec![
            Token::Integer(123).spanning(Span::from(((1, 0), (7, 0))))
        ])
    );
    assert_eq!(
        lex(" 01234 56"),
        Ok(vec![
            Token::Integer(123456).spanning(Span::from(((1, 0), (9, 0))))
        ])
    );
    assert_eq!(
        lex(" 0 "),
        Ok(vec![
            Token::Integer(0).spanning(Span::from(((1, 0), (2, 0))))
        ])
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
        Ok(vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((1, 0), (4, 0))))
        ])
    );
    assert_eq!(
        lex("a  bc "),
        Ok(vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((0, 0), (5, 0))))
        ])
    );
    assert_eq!(
        lex("abc12 3 "),
        Ok(vec![
            Token::Nonterminal("abc123".to_owned()).spanning(Span::from(((0, 0), (7, 0))))
        ])
    );
    assert_eq!(
        lex(" x "),
        Ok(vec![
            Token::Nonterminal("x".to_owned()).spanning(Span::from(((1, 0), (2, 0))))
        ])
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
        Err(Error::InvalidSymbol('+'.to_string()).spanning(Span::from(((1, 0), (2, 0)))))
    );
}

#[test]
fn test_whitespace() {
    assert_eq!(
        lex("  , \n,"),
        Ok(vec![
            Token::Concatenation.spanning(Span::from(((2, 0), (3, 0)))),
            Token::Concatenation.spanning(Span::from(((0, 1), (1, 1))))
        ])
    );
}

#[test]
fn test_comments() {
    assert_eq!(lex(" (* test *) "), Ok(vec![]));
    assert_eq!(
        lex(" (* test * "),
        Err(Error::UnterminatedComment.spanning(Span::from(((10, 0), (11, 0)))))
    );
    assert_eq!(
        lex(" (* ("),
        Err(Error::UnterminatedComment.spanning(Span::from(((4, 0), (5, 0)))))
    );
    assert_eq!(
        lex(", (*, *) , "),
        Ok(vec![
            Token::Concatenation.spanning(Span::from(((0, 0), (1, 0)))),
            Token::Concatenation.spanning(Span::from(((9, 0), (10, 0))))
        ])
    );
    assert_eq!(
        lex(" ,(*, (* ,*) ,*) , ,"),
        Ok(vec![
            Token::Concatenation.spanning(Span::from(((1, 0), (2, 0)))),
            Token::Concatenation.spanning(Span::from(((17, 0), (18, 0)))),
            Token::Concatenation.spanning(Span::from(((19, 0), (20, 0)))),
        ])
    );
    assert_eq!(
        lex(" (* (* *) "),
        Err(Error::UnterminatedComment.spanning(Span::from(((9, 0), (10, 0)))))
    );
    assert_eq!(
        lex(" (*) "),
        Err(Error::InvalidSymbol("(*)".to_owned()).spanning(Span::from(((1, 0), (4, 0)))))
    );
}

#[test]
fn test_multiline() {
    assert_eq!(
        lex(" abc \n = 'def' "),
        Ok(vec![
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((1, 0), (4, 0)))),
            Token::Definition.spanning(Span::from(((1, 1), (2, 1)))),
            Token::Terminal("def".to_owned()).spanning(Span::from(((3, 1), (8, 1))))
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
            Token::Nonterminal("aéf".to_owned()).spanning(Span::from(((0, 0), (4, 0)))),
            Token::Definition.spanning(Span::from(((5, 0), (6, 0)))),
            Token::Nonterminal("abc".to_owned()).spanning(Span::from(((7, 0), (10, 0)))),
            Token::Terminator.spanning(Span::from(((10, 0), (11, 0)))),
        ])
    );
}

#[quickcheck]
fn test_arbitrary_input(input: String) {
    let _ = lex(&input);
}

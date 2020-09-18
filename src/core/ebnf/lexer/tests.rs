use super::super::scanner::Symbol;
use super::lex;
use super::{Error, Token, TokenKind};

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
    assert_eq!(lex(&regular(" ' a \"")), Err(Error::UnterminatedTerminal));
    assert_eq!(lex(&regular("\"bbb'   ")), Err(Error::UnterminatedTerminal));
    assert_eq!(lex(&regular("\"\"")), Err(Error::EmptyTerminal));
    assert_eq!(lex(&regular("''")), Err(Error::EmptyTerminal));
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
    assert_eq!(lex(&regular(" ?bbb  ")), Err(Error::UnterminatedSpecial));
    assert_eq!(
        lex(&regular("??")),
        Ok(vec![Token::new(TokenKind::Special("".to_owned()), 0..2)])
    );
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
}

#[test]
fn test_invalid_symbols() {
    assert_eq!(lex(&regular(" + ")), Err(Error::InvalidSymbol('+')));
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
    assert_eq!(lex(&regular(" (* (* *) ")), Err(Error::UnterminatedComment));
    assert_eq!(lex(&regular(" (*) ")), Err(Error::AmbiguousSymbol));
}

#[test]
fn test_combinations() {}

#[test]
fn test_unicode() {}

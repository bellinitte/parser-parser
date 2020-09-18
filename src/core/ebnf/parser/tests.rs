// use super::super::ast::TokenAt;
// use super::Error;
// use super::{Expression, Grammar, Production};
// use super::{Position, Span, Token};
// use nom::{error::ErrorKind, Err, Slice};

// #[macro_export]
// macro_rules! ok_case {
//     ($parser:expr, $input_str:expr, $offset:expr, $token:expr) => {
//         let input = Span::new($input_str);
//         assert_eq!($parser(input), Ok((input.slice($offset..), $token)));
//     };
// }

// #[macro_export]
// macro_rules! error_case {
//     ($parser:expr, $input_str:expr, $error:expr) => {
//         assert_eq!($parser(Span::new($input_str)), Err(Err::Error($error)));
//     };
// }

// #[macro_export]
// macro_rules! failure_case {
//     ($parser:expr, $input_str:expr, $error:expr) => {
//         assert_eq!($parser(Span::new($input_str)), Err(Err::Failure($error)));
//     };
// }

// #[test]
// fn test_optional_gaps() {
//     use super::optional_gap;

//     ok_case!(optional_gap, "   \t\t  test  ", 7, ());
//     ok_case!(optional_gap, "   \r ", 5, ());
//     ok_case!(optional_gap, "  \r\n\r\r", 6, ());
//     ok_case!(optional_gap, "\x0c\x0b", 2, ());
//     ok_case!(optional_gap, "test  ", 0, ());
//     ok_case!(optional_gap, "  (* comment *) test  ", 16, ());
// }

// #[test]
// fn test_integers() {
//     use super::integer;

//     ok_case!(integer, "123", 3, 123.token_at(0..3));
//     ok_case!(integer, "12 3", 4, 123.token_at(0..4));
//     ok_case!(integer, "12 a", 2, 12.token_at(0..2));
//     ok_case!(integer, "012test", 3, 12.token_at(0..3));
//     error_case!(integer, "test", Error::Internal(ErrorKind::Char));
//     ok_case!(integer, "123  ", 3, 123.token_at(0..3));
//     ok_case!(integer, "1 2  3 ", 6, 123.token_at(0..6));
// }

// #[test]
// fn test_identifiers() {
//     use super::identifier;

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
// }

// #[test]
// fn test_specials() {
//     use super::special;

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
// }

// #[test]
// fn test_terminals() {
//     use super::terminal;

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
// }

// #[test]
// fn test_factors() {
//     use super::factor;

//     ok_case!(
//         factor,
//         "'terminal'",
//         10,
//         Expression::Terminal("terminal".to_owned()).token_at(0..10)
//     );
//     ok_case!(
//         factor,
//         "nonterminal",
//         11,
//         Expression::Nonterminal("nonterminal".to_owned()).token_at(0..11)
//     );
//     ok_case!(
//         factor,
//         "? special ?",
//         11,
//         Expression::Special("special".to_owned()).token_at(0..11)
//     );
//     ok_case!(factor, "", 0, Expression::Empty.token_at(0..0));
    // ok_case!(
    //     factor,
    //     "2 * 'terminal'",
    //     14,
    //     Expression::Factor {
    //         count: 2.token_at(0..1),
    //         primary: Box::new(Expression::Terminal("terminal".to_owned()).token_at(4..14))
    //     }
    //     .token_at(0..14)
    // );
    // ok_case!(
    //     factor,
    //     " 3* a ",
    //     14,
    //     Expression::Factor {
    //         count: 3.token_at(1..2),
    //         primary: Box::new(Expression::Nonterminal("a".to_owned()).token_at(4..5))
    //     }
    //     .token_at(1..5)
    // );
    // ok_case!(factor, " 3 b ", 1, Expression::Empty.token_at(1..1));
// }

// #[test]
// fn test_comments() {
//     use super::comment;

//     assert_eq!(comment("(* comment *)"), Ok(("", ())));
//     assert_eq!(comment("(* (* nested *) *)  "), Ok(("  ", ())));
//     assert_eq!(comment("(*aa (* bb *) cc(*d*)*)fg"), Ok(("fg", ())));
//     assert_eq!(
//         comment("(* not closed "),
//         Err(Failure(("", ErrorKind::Char)))
//     );
// }

// #[test]
// fn test_terms() {
//     use super::term;

//     assert_eq!(
//         term("abc - 'test'"),
//         Ok((
//             "",
//             Expression::Exception {
//                 subject: Box::new(Expression::Nonterminal("abc".to_owned())),
//                 restriction: Box::new(Expression::Terminal("test".to_owned())),
//             }
//         ))
//     );
//     assert_eq!(
//         term("a-b-c"),
//         Ok((
//             "-c",
//             Expression::Exception {
//                 subject: Box::new(Expression::Nonterminal("a".to_owned())),
//                 restriction: Box::new(Expression::Nonterminal("b".to_owned())),
//             }
//         ))
//     );
// }

// #[test]
// fn test_sequences() {
//     use super::sequence;

//     assert_eq!(
//         sequence("abc, 'test', bca"),
//         Ok((
//             "",
//             Expression::Sequence {
//                 first: Box::new(Expression::Nonterminal("abc".to_owned())),
//                 second: Box::new(Expression::Terminal("test".to_owned())),
//                 rest: vec![Expression::Nonterminal("bca".to_owned())]
//             }
//         ))
//     );
// }

// #[test]
// fn test_alternatives() {
//     use super::alternative;

//     assert_eq!(
//         alternative(" a, 'b' | 'c', d "),
//         Ok((
//             "",
//             Expression::Alternative {
//                 first: Box::new(Expression::Sequence {
//                     first: Box::new(Expression::Nonterminal("a".to_owned())),
//                     second: Box::new(Expression::Terminal("b".to_owned())),
//                     rest: Vec::new(),
//                 }),
//                 second: Box::new(Expression::Sequence {
//                     first: Box::new(Expression::Terminal("c".to_owned())),
//                     second: Box::new(Expression::Nonterminal("d".to_owned())),
//                     rest: Vec::new(),
//                 }),
//                 rest: Vec::new(),
//             }
//         ))
//     );
// }

// #[test]
// fn test_grouped() {
//     use super::grouped;

//     assert_eq!(
//         grouped("(b | c)"),
//         Ok((
//             "",
//             Expression::Alternative {
//                 first: Box::new(Expression::Nonterminal("b".to_owned())),
//                 second: Box::new(Expression::Nonterminal("c".to_owned())),
//                 rest: Vec::new(),
//             }
//         ))
//     );
//     assert_eq!(
//         grouped("( a, 'b' (* comment *) | c )"),
//         Ok((
//             "",
//             Expression::Alternative {
//                 first: Box::new(Expression::Sequence {
//                     first: Box::new(Expression::Nonterminal("a".to_owned())),
//                     second: Box::new(Expression::Terminal("b".to_owned())),
//                     rest: Vec::new(),
//                 }),
//                 second: Box::new(Expression::Nonterminal("c".to_owned())),
//                 rest: Vec::new(),
//             }
//         ))
//     );
// }

// #[test]
// fn test_repeated() {
//     use super::repeated;

//     assert_eq!(
//         repeated("{abc (**) |def}"),
//         Ok((
//             "",
//             Expression::Repeated(Box::new(Expression::Alternative {
//                 first: Box::new(Expression::Nonterminal("abc".to_owned())),
//                 second: Box::new(Expression::Nonterminal("def".to_owned())),
//                 rest: Vec::new(),
//             }))
//         ))
//     );
// }

// #[test]
// fn test_optionals() {
//     use super::optional;

//     assert_eq!(
//         optional("[ abc|def (*test*) ]"),
//         Ok((
//             "",
//             Expression::Optional(Box::new(Expression::Alternative {
//                 first: Box::new(Expression::Nonterminal("abc".to_owned())),
//                 second: Box::new(Expression::Nonterminal("def".to_owned())),
//                 rest: Vec::new(),
//             }))
//         ))
//     );
// }

// #[test]
// fn test_productions() {
//     use super::production;

//     assert_eq!(
//         production("abc = 'a', (b | 'c' (* test *)); "),
//         Ok((
//             "",
//             Production {
//                 lhs: "abc".to_owned(),
//                 rhs: Expression::Sequence {
//                     first: Box::new(Expression::Terminal("a".to_owned())),
//                     second: Box::new(Expression::Alternative {
//                         first: Box::new(Expression::Nonterminal("b".to_owned())),
//                         second: Box::new(Expression::Terminal("c".to_owned())),
//                         rest: Vec::new(),
//                     }),
//                     rest: Vec::new(),
//                 }
//             }
//         ))
//     );
// }

// #[test]
// fn test_syntaxes() {
//     use super::syntax;

//     assert_eq!(
//         syntax("a = 'd' | {2 * 'e'}; (* test *)\nb = 'a', (a | 'c');\n"),
//         Ok((
//             "",
//             Grammar {
//                 productions: vec![
//                     Production {
//                         lhs: "a".to_owned(),
//                         rhs: Expression::Alternative {
//                             first: Box::new(Expression::Terminal("d".to_owned())),
//                             second: Box::new(Expression::Repeated(Box::new(Expression::Factor {
//                                 count: 2,
//                                 primary: Box::new(Expression::Terminal("e".to_owned()))
//                             }))),
//                             rest: Vec::new(),
//                         }
//                     },
//                     Production {
//                         lhs: "b".to_owned(),
//                         rhs: Expression::Sequence {
//                             first: Box::new(Expression::Terminal("a".to_owned())),
//                             second: Box::new(Expression::Alternative {
//                                 first: Box::new(Expression::Nonterminal("a".to_owned())),
//                                 second: Box::new(Expression::Terminal("c".to_owned())),
//                                 rest: Vec::new(),
//                             }),
//                             rest: Vec::new(),
//                         }
//                     }
//                 ]
//             }
//         ))
//     );
// }

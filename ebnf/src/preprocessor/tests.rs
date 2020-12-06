// use super::{preprocess, Error, Expression, Grammar, Production, Span, Spanning};

// #[test]
// fn test_indirect_left_recursion() {
//     assert_eq!(
//         preprocess(
//             Grammar {
//                 productions: vec![
//                     Production {
//                         lhs: "a".to_owned().spanning(Span::from(((0, 0), (1, 0)))),
//                         rhs: Expression::Nonterminal("b".to_owned())
//                             .spanning(Span::from(((4, 0), (5, 0))))
//                     }
//                     .spanning(Span::from(((0, 0), (6, 0)))),
//                     Production {
//                         lhs: "b".to_owned().spanning(Span::from(((0, 1), (1, 1)))),
//                         rhs: Expression::Nonterminal("a".to_owned())
//                             .spanning(Span::from(((4, 1), (5, 1))))
//                     }
//                     .spanning(Span::from(((0, 1), (6, 1))))
//                 ]
//             }
//             .spanning(Span::from(((0, 0), (6, 1))))
//         ),
//         Err(
//             Error::LeftRecursion(vec!["b".to_owned(), "a".to_owned(), "b".to_owned()])
//                 .spanning(Span::from(((4, 0), (5, 0))))
//         )
//     );
// }

// TODO decide what to do about the fact that a terminal can contain a newline vs a space.

use nom::{
    error::{ErrorKind, ParseError},
    IResult, InputTakeAtPosition,
};

pub mod error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    productions: Vec<Production>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    lhs: String,
    rhs: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    Alternative {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
    },
    Sequence {
        first: Box<Expression>,
        second: Box<Expression>,
        rest: Vec<Expression>,
    },
    Optional(Box<Expression>),
    Repeated(Box<Expression>),
    Factor {
        count: usize,
        primary: Box<Expression>
    },
    Exception {
        subject: Box<Expression>,
        restriction: Box<Expression>
    },
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
}

/// Parses a non-zero sequence of decimal digits and returns a usize represented by that sequence.
///
/// # Example
///
/// ```rust
/// assert_eq!(integer::<(&str, ErrorKind)>("123"), Ok(("", 123)));
/// ```
pub fn integer<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, usize, E> {
    let mut chars = i.chars();
    let mut offset = 0;
    let mut integer: usize = 0;

    match chars.next() {
        Some(c) if c.is_digit(10) => {
            integer = c.to_digit(10).unwrap() as usize;
            offset += c.len_utf8();
        },
        _ => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Char)))?,
    };

    let mut last_whitespace_offset = 0;

    loop {
        match chars.next() {
            Some(c) if c.is_digit(10) => {
                integer *= 10;
                integer += c.to_digit(10).unwrap() as usize;
                offset += last_whitespace_offset;
                last_whitespace_offset = 0;
                offset += c.len_utf8();
            },
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            },
            _ => break,
        };
    }

    Ok((&i[offset..], integer))
}

/// Parses a meta identifier from ISO/IEC 14977, which is a letter followed by a possibly empty
/// sequence of letters or digits.
/// ```ebnf
/// meta identifier
///   = letter, {meta identifier character};
/// meta identifier character
///   = letter
///   | decimal digit;
/// ```
///
/// The parser is extended to support UTF-8 characters, where a letter is any alphabetic character,
/// and a digit is any numeric character, which are specified
/// in the [Unicode Character Database](https://www.unicode.org/reports/tr44/).
///
/// # Example
///
/// ```rust
/// assert_eq!(
///     meta_identifier::<(&str, ErrorKind)>("test2 abc"),
///     Ok((" abc", "test2"))
/// );
/// ```
pub fn identifier<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, String, E> {
    let mut chars = i.chars();
    let mut offset = 0;
    let mut identifier: String = String::new();

    match chars.next() {
        Some(c) if c.is_alphabetic() => {
            identifier.push(c);
            offset += c.len_utf8();
        },
        _ => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Char)))?,
    };

    let mut last_whitespace_offset = 0;

    loop {
        match chars.next() {
            Some(c) if c.is_alphanumeric() => {
                identifier.push(c);
                offset += last_whitespace_offset;
                last_whitespace_offset = 0;
                offset += c.len_utf8();
            },
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            },
            _ => break,
        };
    }

    Ok((&i[offset..], identifier))
}

/// Parses a '{gap separator}' from ISO/IEC 14977, which is a possibly empty sequence of
/// 'gap separators' defined as
/// ```ebnf
/// gap separator
///   = space character
///   | horizontal_tabulation character
///   | new line
///   | vertical tabulation character
///   | form feed;
/// ```
/// where a new line is a line feed surrounded by possibly empty sequences of carriage returns.
///
/// # Example
///
/// ```rust
/// assert_eq!(
///     gap_separation::<(&str, ErrorKind)>("   \t\t  test  "),
///     Ok(("test  ", ()))
/// );
/// ```
fn optional_gap<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    use nom::combinator::opt;

    let (i, _) = i.split_at_position_complete(|c| !c.is_whitespace())?;
    let mut j = i;

    loop {
        let i = match opt(comment)(j)? {
            (i, Some(_)) => {
                i
            },
            (i, None) => {
                return Ok((i, ()));
            },
        };
        let (i, _) = i.split_at_position_complete(|c| !c.is_whitespace())?;
        j = i;
    }
}

/// Parses a special sequence from ISO/IEC 14977, which is any sequence
/// of terminal characters, which starts and ends with a question mark symbol.
/// ```ebnf
/// special sequence symbol = '?';
/// special sequence
///   = special sequence symbol,
///     {special sequence character},
///     special sequence symbol;
/// special sequence character
///   = terminal character - special sequence symbol;
/// ```
///
/// # Example
///
/// ```rust
/// assert_eq!(
///     special_sequence::<(&str, ErrorKind)>("? anything really ?"),
///     Ok(("", " anything really "))
/// );
/// ```
pub fn special<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    let mut chars = i.chars();
    let mut offset = 0;
    let mut sequence: String = String::new();

    match chars.next() {
        Some(c) if c == '?' => {
            offset += c.len_utf8();
        },
        _ => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Char)))?,
    };

    let mut last_whitespace_offset = 0;

    loop {
        match chars.next() {
            Some(c) if c == '?' => {
                offset += last_whitespace_offset;
                offset += c.len_utf8();
                let term = Expression::Special(sequence);
                return Ok((&i[offset..], term));
            },
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            },
            Some(c) if !c.is_control() => {
                sequence.push(c);
                offset += last_whitespace_offset;
                last_whitespace_offset = 0;
                offset += c.len_utf8();
            },
            _ => Err(nom::Err::Failure(E::from_error_kind(&i[offset..], ErrorKind::Char)))?,
        };
    }
}

/// Parses a terminal string from ISO/IEC 14977, which is a non-zero sequence of
/// terminal characters surrounded by single or double quotes.
/// ```ebnf
/// terminal string
///   = first quote symbol, first terminal character,
///     {first terminal character},
///     first quote symbol
///   | second quote symbol, second terminal character,
///     {second terminal character},
///     second quote symbol;
/// first terminal character
///   = terminal character - first quote symbol;
/// second terminal character
///   = terminal character - second quote symbol;
/// ```
///
/// # Example
///
///
pub fn terminal<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{
        branch::alt,
        bytes::complete::take_till1,
        character::complete::char,
        combinator::cut,
        combinator::map,
        sequence::{preceded, terminated},
    };

    map(
        alt((
            preceded(
                char('\''),
                cut(terminated(
                    take_till1(|c: char| c.is_control() || c == '\''),
                    char('\''),
                )),
            ),
            preceded(
                char('"'),
                cut(terminated(
                    take_till1(|c: char| c.is_control() || c == '"'),
                    char('"'),
                )),
            ),
        )),
        |s: &str| Expression::Terminal(s.to_owned()),
    )(i)
}

pub fn nonterminal<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::combinator::map;

    map(
        identifier,
        |s| Expression::Nonterminal(s),
    )(i)
}

pub fn grouped<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{character::complete::char, sequence::delimited};

    delimited(char('('), alternative, char(')'))(i)
}

pub fn repeated<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited};
    
    map(
        delimited(
            alt((tag("{"), tag("(:"))),
            alternative,
            alt((tag("}"), tag(":)"))),
        ),
        |e| Expression::Repeated(Box::new(e))
    )(i)
}

pub fn optional<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited};
    
    map(
        delimited(
            alt((tag("["), tag("(/"))),
            alternative,
            alt((tag("]"), tag("/)"))),
        ),
        |e| Expression::Optional(Box::new(e))
    )(i)
}

pub fn factor<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{branch::alt, character::complete::char, combinator::{map, opt}};

    let (i, count) = opt(|i: &'a str| -> IResult<&'a str, usize, E> {
        let (i, _) = optional_gap(i)?;
        let (i, integer) = integer(i)?;
        let (i, _) = optional_gap(i)?;
        let (i, _) = char('*')(i)?;
        Ok((i, integer))
    })(i)?;

    let (i, _) = optional_gap(i)?;

    let (i, expression) = map(
        alt((
            optional,
            repeated,
            grouped,
            nonterminal,
            terminal,
            special,
            empty,
        )),
        move |e| match count {
            Some(0) => Expression::Empty,
            Some(i) if i > 1 => Expression::Factor {
                count: i,
                primary: Box::new(e),
            },
            _ => e,
        }
    )(i)?;

    let (i, _) = optional_gap(i)?;

    Ok((i, expression))
}

pub fn empty<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Expression, E> {
    Ok((i, Expression::Empty))
}

pub fn term<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Expression, E> {
    use nom::{character::complete::char, combinator::opt, sequence::preceded};

    let (i, primary) = factor(i)?;
    // TODO a syntactic-factor that could be replaced by a syntactic-factor containing no meta-identifiers
    let (i, exception) = opt(preceded(char('-'), factor))(i)?;
    Ok(match exception {
        None => (i, primary),
        Some(ex) => (i, Expression::Exception {
            subject: Box::new(primary),
            restriction: Box::new(ex)
        }),
    })
}

pub fn sequence<'a, E: ParseError<&'a str> + 'a>(
    i: &'a str,
) -> IResult<&'a str, Expression, E> {
    use nom::{character::complete::char, combinator::map, multi::separated_list1};

    map(
        separated_list1(char(','), term),
        |expressions: Vec<Expression>| {
            assert!(expressions.len() > 0);
            match expressions.len() {
                1 => expressions[0].clone(),
                _ => Expression::Sequence {
                    first: Box::new(expressions[0].clone()),
                    second: Box::new(expressions[1].clone()),
                    rest: expressions[2..].to_vec(),
                },
            }
        },
    )(i)
}

pub fn alternative<'a, E: ParseError<&'a str> + 'a>(
    i: &'a str,
) -> IResult<&'a str, Expression, E> {
    use nom::{character::complete::one_of, combinator::map, multi::separated_list1};

    map(
        separated_list1(one_of("|/!"), sequence),
        |expressions: Vec<Expression>| {
            assert!(expressions.len() > 0);
            match expressions.len() {
                1 => expressions[0].clone(),
                _ => Expression::Alternative {
                    first: Box::new(expressions[0].clone()),
                    second: Box::new(expressions[1].clone()),
                    rest: expressions[2..].to_vec(),
                },
            }
        },
    )(i)
}

pub fn production<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Production, E> {
    use nom::character::complete::{char, one_of};

    let (i, _) = optional_gap(i)?;
    let (i, identifier) = identifier(i)?;
    let (i, _) = optional_gap(i)?;
    let (i, _) = char('=')(i)?;
    let (i, definitions) = alternative(i)?;
    let (i, _) = one_of(";.")(i)?;
    let (i, _) = optional_gap(i)?;
    Ok((
        i,
        Production {
            lhs: identifier,
            rhs: definitions,
        },
    ))
}

pub fn syntax<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Grammar, E> {
    use nom::{combinator::map, multi::many1};

    map(many1(production), |p: Vec<Production>| -> Grammar {
        Grammar { productions: p }
    })(i)
}

pub fn comment<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (), E> {
    use nom::{
        bytes::complete::tag,
        combinator::opt,
    };

    let (i, _) = tag("(*")(i)?;
    let mut j = i;
    
    loop {
        let (i, _) = opt(comment)(j)?;
        if let (i, Some(_)) = opt(tag("*)"))(i)? {
            return Ok((i, ()));
        };
        let i = match i.chars().next() {
            Some(c) if !c.is_control() => {
                &i[c.len_utf8()..]
            },
            Some(_) => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Char)))?,
            None => Err(nom::Err::Failure(E::from_error_kind(i, ErrorKind::Char)))?,
        };
        j = i;
    }
}

#[cfg(test)]
mod tests {
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
}

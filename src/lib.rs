use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use nom::{
    error::{ErrorKind, ParseError},
    AsChar, FindToken, IResult, InputTake, InputTakeAtPosition,
};

pub mod error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Grammar {
    productions: Vec<Production>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Production {
    pub lhs: String,
    rhs: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Expression(Vec<Term>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Optional(Vec<Expression>),
    Repeated(Vec<Expression>),
    Grouped(Vec<Expression>),
    Factor(usize, Box<Term>),
    Exception(Box<Term>, Box<Term>),
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Empty,
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
pub fn gap_separation<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    let newline: bool = i.find_token('\n');
    match i.split_at_position_complete(|item| {
        let c = item.clone().as_char();
        !(c == ' '
            || c == '\t'
            || c == '\x0b'
            || c == '\x0c'
            || (newline && (c == '\n' || c == '\r'))) // We are doing a newline check for every
                                                      // character, TODO split this into
                                                      // two separate closures
    }) {
        Ok((rest, _)) => Ok((rest, ())),
        Err(e) => Err(e),
    }
}

/// Parses a non-zero sequence of decimal digits and returns a usize represented by that sequence.
///
/// # Example
///
/// ```rust
/// assert_eq!(integer::<(&str, ErrorKind)>("123"), Ok(("", 123)));
/// ```
pub fn integer<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, usize, E> {
    use nom::{character::complete::digit1, combinator::map_res};

    skipped(map_res(digit1, |s: &str| s.parse()))(i)
}

pub fn is_terminal_character(i: char) -> bool {
    !i.is_control()
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
pub fn meta_identifier<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Term, E> {
    let (i, _) = skip(i)?;

    skipped(|i| {
        let (rest, offset) = match i.chars().next().map(|c| {
            let b = c.is_alphabetic();
            (c, b)
        }) {
            Some((c, true)) => Ok((&i[c.len_utf8()..], c.len_utf8())),
            _ => Err(nom::Err::Error(E::from_error_kind(i, ErrorKind::Char))),
        }?;

        match rest.find(|c: char| !c.is_alphanumeric()) {
            Some(pos) => Ok(i.take_split(pos + offset)),
            None => Ok(i.take_split(i.len())),
        }
        .map(|(i, ident)| -> (&str, Term) { (i, Term::Nonterminal(ident.to_owned())) })
    })(i)
}

fn skip<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    use nom::combinator::opt;

    let (i, _) = gap_separation(i)?;
    let (i, _) = opt(bracketed_textual_comment)(i)?;
    let (i, _) = gap_separation(i)?;
    Ok((i, ()))
}

pub fn skipped<'a, O, E: ParseError<&'a str>, F>(
    mut f: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E> + 'a,
{
    move |i: &'a str| {
        let (i, _) = skip(i)?;
        let (i, res) = f(i)?;
        let (i, _) = skip(i)?;
        Ok((i, res))
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
pub fn special_sequence<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{
        bytes::complete::take_till,
        character::complete::char,
        combinator::{cut, map},
        sequence::{preceded, terminated},
    };

    skipped(map(
        preceded(
            char('?'),
            cut(terminated(
                take_till(|c| !is_terminal_character(c) || c == '?'),
                char('?'),
            )),
        ),
        |s: &str| -> Term { Term::Special(s.to_owned()) },
    ))(i)
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
pub fn terminal_string<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{
        branch::alt,
        bytes::complete::take_till1,
        character::complete::char,
        combinator::cut,
        combinator::map,
        sequence::{preceded, terminated},
    };

    skipped(map(
        alt((
            preceded(
                char('\''),
                cut(terminated(
                    take_till1(|c| !is_terminal_character(c) || c == '\''),
                    char('\''),
                )),
            ),
            preceded(
                char('"'),
                cut(terminated(
                    take_till1(|c| !is_terminal_character(c) || c == '"'),
                    char('"'),
                )),
            ),
        )),
        |s: &str| -> Term { Term::Terminal(s.to_owned()) },
    ))(i)
}

pub fn grouped_sequence<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{character::complete::char, combinator::map, sequence::delimited};

    skipped(map(
        delimited(char('('), definitions_list, char(')')),
        |e: Vec<Expression>| -> Term { Term::Grouped(e) },
    ))(i)
}

pub fn repeated_sequence<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited};

    skipped(map(
        delimited(
            alt((tag("{"), tag("(:"))),
            definitions_list,
            alt((tag("}"), tag(":)"))),
        ),
        |e: Vec<Expression>| -> Term { Term::Repeated(e) },
    ))(i)
}

pub fn optional_sequence<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::delimited};

    skipped(map(
        delimited(
            alt((tag("["), tag("(/"))),
            definitions_list,
            alt((tag("]"), tag("/)"))),
        ),
        |e: Vec<Expression>| -> Term { Term::Optional(e) },
    ))(i)
}

pub fn syntactic_primary<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::branch::alt;

    alt((
        optional_sequence,
        repeated_sequence,
        grouped_sequence,
        meta_identifier,
        terminal_string,
        special_sequence,
        empty_sequence,
    ))(i)
}

pub fn empty_sequence<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, Term, E> {
    Ok((i, Term::Empty))
}

pub fn syntactic_factor<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{character::complete::char, combinator::opt, sequence::terminated};

    let (i, number): (&str, Option<usize>) = opt(terminated(integer, char('*')))(i)?;
    let (i, primary) = syntactic_primary(i)?;
    Ok(match number {
        None => (i, primary),
        Some(n) => (i, Term::Factor(n, Box::new(primary))),
    })
}

pub fn syntactic_term<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Term, E> {
    use nom::{character::complete::char, combinator::opt, sequence::preceded};

    let (i, factor) = syntactic_factor(i)?;
    // TODO a syntactic-factor that could be replaced by a syntactic-factor containing no meta-identifiers
    let (i, exception) = opt(preceded(char('-'), syntactic_factor))(i)?;
    Ok(match exception {
        None => (i, factor),
        Some(ex) => (i, Term::Exception(Box::new(factor), Box::new(ex))),
    })
}

pub fn single_definition<'a, E: ParseError<&'a str> + 'a>(
    i: &'a str,
) -> IResult<&'a str, Expression, E> {
    use nom::{character::complete::char, combinator::map, multi::separated_list1};

    map(
        separated_list1(char(','), syntactic_term),
        |t: Vec<Term>| -> Expression { Expression(t) },
    )(i)
}

pub fn definitions_list<'a, E: ParseError<&'a str> + 'a>(
    i: &'a str,
) -> IResult<&'a str, Vec<Expression>, E> {
    use nom::{character::complete::one_of, multi::separated_list1};

    separated_list1(one_of("|/!"), single_definition)(i)
}

pub fn syntax_rule<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Production, E> {
    use nom::character::complete::{char, one_of};

    let (i, identifier) = meta_identifier(i)?;
    let (i, _) = char('=')(i)?;
    let (i, definitions) = definitions_list(i)?;
    let (i, _) = one_of(";.")(i)?;
    let (i, _) = skip(i)?;
    Ok((
        i,
        Production {
            lhs: match identifier {
                Term::Nonterminal(ident) => ident,
                _ => unreachable!(),
            },
            rhs: definitions,
        },
    ))
}

pub fn syntax<'a, E: ParseError<&'a str> + 'a>(i: &'a str) -> IResult<&'a str, Grammar, E> {
    use nom::{combinator::map, multi::many1};

    map(many1(syntax_rule), |p: Vec<Production>| -> Grammar {
        Grammar { productions: p }
    })(i)
}

pub fn bracketed_textual_comment<'a, E: ParseError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (), E> {
    use nom::{
        bytes::complete::{tag, take_until},
        combinator::{cut, map},
        sequence::{preceded, terminated},
    };

    // TODO allow only terminal_characters inside the comment (along with the whitespace)
    map(
        preceded(tag("(*"), cut(terminated(take_until("*)"), tag("*)")))),
        |_| (),
    )(i)
}

#[wasm_bindgen]
pub fn parse(input: JsString) -> JsString {
    let i: String = input.into();
    format!("{:?}", syntax::<()>(&i).ok().map(|(_, grammar)| grammar)).into()
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if the code ever panics.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(test)]
mod tests {
    use super::{Expression, Grammar, Production, Term};
    use nom::{
        error::ErrorKind,
        Err::{Error, Failure},
    };

    #[test]
    fn test_terminal_characters() {
        use super::is_terminal_character;

        for i in 0..32 {
            assert!(!is_terminal_character(std::char::from_u32(i).unwrap()));
        }
        for i in 32..127 {
            assert!(is_terminal_character(std::char::from_u32(i).unwrap()));
        }
        for i in 127..160 {
            assert!(!is_terminal_character(std::char::from_u32(i).unwrap()));
        }
        for i in 160..4096 {
            assert!(is_terminal_character(std::char::from_u32(i).unwrap()));
        }
    }

    #[test]
    fn test_gap_separations() {
        use super::gap_separation;

        assert_eq!(
            gap_separation::<(&str, ErrorKind)>("   \t\t  test  "),
            Ok(("test  ", ()))
        );
        assert_eq!(
            gap_separation::<(&str, ErrorKind)>("   \r "),
            Ok(("\r ", ()))
        );
        assert_eq!(
            gap_separation::<(&str, ErrorKind)>("  \r\n\r\r"),
            Ok(("", ()))
        );
        assert_eq!(
            gap_separation::<(&str, ErrorKind)>("\x0c\x0b"),
            Ok(("", ()))
        );
        assert_eq!(
            gap_separation::<(&str, ErrorKind)>("test  "),
            Ok(("test  ", ()))
        );
    }

    #[test]
    fn test_integers() {
        use super::integer;

        assert_eq!(integer::<(&str, ErrorKind)>("123"), Ok(("", 123)));
        assert_eq!(integer::<(&str, ErrorKind)>("012test"), Ok(("test", 12)));
        assert_eq!(
            integer::<(&str, ErrorKind)>("test"),
            Err(Error(("test", ErrorKind::Digit)))
        );
        // Skips
        assert_eq!(integer::<(&str, ErrorKind)>("  123 "), Ok(("", 123)));
        assert_eq!(integer::<(&str, ErrorKind)>("(**)56 (**)"), Ok(("", 56)));
    }

    #[test]
    fn test_meta_identifiers() {
        use super::meta_identifier;

        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("abc12"),
            Ok(("", Term::Nonterminal("abc12".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("12abc"),
            Err(Error(("12abc", ErrorKind::Char)))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("_test"),
            Err(Error(("_test", ErrorKind::Char)))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("test abc"),
            Ok(("abc", Term::Nonterminal("test".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("藏京٣¾  abc"),
            Ok(("abc", Term::Nonterminal("藏京٣¾".to_owned())))
        );
        // Skips
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("  test"),
            Ok(("", Term::Nonterminal("test".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("  test  abc"),
            Ok(("abc", Term::Nonterminal("test".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("abc(* comment *)"),
            Ok(("", Term::Nonterminal("abc".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("(**)a(**)b"),
            Ok(("b", Term::Nonterminal("a".to_owned())))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("  (**) a (**)  b"),
            Ok(("b", Term::Nonterminal("a".to_owned())))
        );
    }

    #[test]
    fn test_special_sequences() {
        use super::special_sequence;

        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("? anything really ?"),
            Ok(("", Term::Special(" anything really ".to_owned())))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("?藏!? abc"),
            Ok(("abc", Term::Special("藏!".to_owned())))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("? not closed"),
            Err(Failure(("", ErrorKind::Char)))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("not opened ?"),
            Err(Error(("not opened ?", ErrorKind::Char)))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("? this has\na newline ?"),
            Err(Failure(("\na newline ?", ErrorKind::Char)))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("??"),
            Ok(("", Term::Special("".to_owned())))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("? test (* comment *) ?"),
            Ok(("", Term::Special(" test (* comment *) ".to_owned())))
        );
        // Skips
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("  ? test ?  "),
            Ok(("", Term::Special(" test ".to_owned())))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>(" (**) ? test ?  (**) "),
            Ok(("", Term::Special(" test ".to_owned())))
        );
    }

    #[test]
    fn test_terminal_string() {
        use super::terminal_string;

        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("'a string'"),
            Ok(("", Term::Terminal("a string".to_owned())))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("\"some other string  \"abc"),
            Ok(("abc", Term::Terminal("some other string  ".to_owned())))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("\"not closed"),
            Err(Failure(("", ErrorKind::Char)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("'not closed"),
            Err(Failure(("", ErrorKind::Char)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("not opened'"),
            Err(Error(("not opened'", ErrorKind::Char)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("'this has\na newline'abc"),
            Err(Failure(("\na newline'abc", ErrorKind::Char)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("\"this has\na newline\"abc"),
            Err(Failure(("\na newline\"abc", ErrorKind::Char)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("\"\""),
            Err(Failure(("\"", ErrorKind::TakeTill1)))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("  'a string'  "),
            Ok(("", Term::Terminal("a string".to_owned())))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>(" (**) 'a string' (**) "),
            Ok(("", Term::Terminal("a string".to_owned())))
        );
    }

    #[test]
    fn test_syntactic_primary() {
        use super::syntactic_primary;

        assert_eq!(
            syntactic_primary::<(&str, ErrorKind)>("'terminal'"),
            Ok(("", Term::Terminal("terminal".to_owned())))
        );
        assert_eq!(
            syntactic_primary::<(&str, ErrorKind)>("nonterminal"),
            Ok(("", Term::Nonterminal("nonterminal".to_owned())))
        );
        assert_eq!(
            syntactic_primary::<(&str, ErrorKind)>("? special ?"),
            Ok(("", Term::Special(" special ".to_owned())))
        );
        assert_eq!(
            syntactic_primary::<(&str, ErrorKind)>(""),
            Ok(("", Term::Empty))
        );
    }

    #[test]
    fn test_syntactic_factor() {
        use super::syntactic_factor;

        assert_eq!(
            syntactic_factor::<(&str, ErrorKind)>("2 * 'terminal'"),
            Ok((
                "",
                Term::Factor(2, Box::new(Term::Terminal("terminal".to_owned())))
            ))
        );
        assert_eq!(
            syntactic_factor::<(&str, ErrorKind)>(" 3* a "),
            Ok((
                "",
                Term::Factor(3, Box::new(Term::Nonterminal("a".to_owned())))
            ))
        );
        assert_eq!(
            syntactic_factor::<(&str, ErrorKind)>(" 3 b "),
            Ok((" 3 b ", Term::Empty))
        );
    }

    #[test]
    fn test_bracketed_textual_comment() {
        use super::bracketed_textual_comment;

        assert_eq!(
            bracketed_textual_comment::<(&str, ErrorKind)>("(* comment *)"),
            Ok(("", ()))
        );
    }

    #[test]
    fn test_syntactic_term() {
        use super::syntactic_term;

        assert_eq!(
            syntactic_term::<(&str, ErrorKind)>("abc - 'test'"),
            Ok((
                "",
                Term::Exception(
                    Box::new(Term::Nonterminal("abc".to_owned())),
                    Box::new(Term::Terminal("test".to_owned()))
                )
            ))
        );
        assert_eq!(
            syntactic_term::<(&str, ErrorKind)>("a-b-c"),
            Ok((
                "-c",
                Term::Exception(
                    Box::new(Term::Nonterminal("a".to_owned())),
                    Box::new(Term::Nonterminal("b".to_owned()))
                )
            ))
        );
    }

    #[test]
    fn test_single_definition() {
        use super::single_definition;

        assert_eq!(
            single_definition::<(&str, ErrorKind)>("abc, 'test', bca"),
            Ok((
                "",
                Expression(vec![
                    Term::Nonterminal("abc".to_owned()),
                    Term::Terminal("test".to_owned()),
                    Term::Nonterminal("bca".to_owned())
                ])
            ))
        );
    }

    #[test]
    fn test_definitions_list() {
        use super::definitions_list;

        assert_eq!(
            definitions_list::<(&str, ErrorKind)>(" a, 'b' | 'c', d "),
            Ok((
                "",
                vec![
                    Expression(vec![
                        Term::Nonterminal("a".to_owned()),
                        Term::Terminal("b".to_owned())
                    ]),
                    Expression(vec![
                        Term::Terminal("c".to_owned()),
                        Term::Nonterminal("d".to_owned())
                    ])
                ]
            ))
        );
    }

    #[test]
    fn test_grouped_sequence() {
        use super::grouped_sequence;

        assert_eq!(
            grouped_sequence::<(&str, ErrorKind)>("(b | c)"),
            Ok((
                "",
                Term::Grouped(vec![
                    Expression(vec![Term::Nonterminal("b".to_owned())]),
                    Expression(vec![Term::Nonterminal("c".to_owned())])
                ])
            ))
        );
        assert_eq!(
            grouped_sequence::<(&str, ErrorKind)>("( a, 'b' (* comment *) | c )"),
            Ok((
                "",
                Term::Grouped(vec![
                    Expression(vec![
                        Term::Nonterminal("a".to_owned()),
                        Term::Terminal("b".to_owned())
                    ]),
                    Expression(vec![Term::Nonterminal("c".to_owned())])
                ])
            ))
        );
    }

    #[test]
    fn test_repeated_sequence() {
        use super::repeated_sequence;

        assert_eq!(
            repeated_sequence::<(&str, ErrorKind)>("{abc (**) |def}"),
            Ok((
                "",
                Term::Repeated(vec![
                    Expression(vec![Term::Nonterminal("abc".to_owned())]),
                    Expression(vec![Term::Nonterminal("def".to_owned())])
                ])
            ))
        );
    }

    #[test]
    fn test_optional_sequence() {
        use super::optional_sequence;

        assert_eq!(
            optional_sequence::<(&str, ErrorKind)>("[ abc|def (*test*) ]"),
            Ok((
                "",
                Term::Optional(vec![
                    Expression(vec![Term::Nonterminal("abc".to_owned())]),
                    Expression(vec![Term::Nonterminal("def".to_owned())])
                ])
            ))
        );
    }

    #[test]
    fn test_syntax_rule() {
        use super::syntax_rule;

        assert_eq!(
            syntax_rule::<(&str, ErrorKind)>("abc = 'a', (b | 'c' (* test *)); "),
            Ok((
                "",
                Production {
                    lhs: "abc".to_owned(),
                    rhs: vec![Expression(vec![
                        Term::Terminal("a".to_owned()),
                        Term::Grouped(vec![
                            Expression(vec![Term::Nonterminal("b".to_owned())]),
                            Expression(vec![Term::Terminal("c".to_owned())])
                        ])
                    ])]
                }
            ))
        );
    }

    #[test]
    fn test_syntax() {
        use super::syntax;

        assert_eq!(
            syntax::<(&str, ErrorKind)>("a = 'd' | {2 * 'e'}; (* test *)\nb = 'a', (a | 'c');\n"),
            Ok((
                "",
                Grammar {
                    productions: vec![
                        Production {
                            lhs: "a".to_owned(),
                            rhs: vec![
                                Expression(vec![Term::Terminal("d".to_owned())]),
                                Expression(vec![Term::Repeated(vec![Expression(vec![
                                    Term::Factor(2, Box::new(Term::Terminal("e".to_owned())))
                                ])])])
                            ]
                        },
                        Production {
                            lhs: "b".to_owned(),
                            rhs: vec![Expression(vec![
                                Term::Terminal("a".to_owned()),
                                Term::Grouped(vec![
                                    Expression(vec![Term::Nonterminal("a".to_owned())]),
                                    Expression(vec![Term::Terminal("c".to_owned())])
                                ])
                            ])]
                        }
                    ]
                }
            ))
        );
    }
}

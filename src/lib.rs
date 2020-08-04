use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use nom::{
    error::{ErrorKind, ParseError},
    AsChar, FindToken, IResult, InputTake, InputTakeAtPosition,
};

pub mod error;

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
pub fn integer<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, usize, E> {
    use nom::{character::complete::digit1, combinator::map_res};

    map_res(digit1, |s: &str| s.parse())(i)
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
pub fn meta_identifier<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
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
pub fn special_sequence<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    use nom::{
        bytes::complete::take_till,
        character::complete::char,
        combinator::cut,
        sequence::{preceded, terminated},
    };

    preceded(
        char('?'),
        cut(terminated(
            take_till(|c| !is_terminal_character(c) || c == '?'),
            char('?'),
        )),
    )(i)
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
pub fn terminal_string<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    use nom::{
        branch::alt,
        bytes::complete::take_till1,
        character::complete::char,
        combinator::cut,
        sequence::{preceded, terminated},
    };

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
    ))(i)
}

#[no_mangle]
#[wasm_bindgen(js_name = getMessage)]
pub fn get_message() -> String {
    "world".to_owned()
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
    }

    #[test]
    fn test_meta_identifiers() {
        use super::meta_identifier;

        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("abc12"),
            Ok(("", "abc12"))
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
            Ok((" abc", "test"))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("  test"),
            Err(Error(("  test", ErrorKind::Char)))
        );
        assert_eq!(
            meta_identifier::<(&str, ErrorKind)>("藏京٣¾  abc"),
            Ok(("  abc", "藏京٣¾"))
        );
    }

    #[test]
    fn test_special_sequences() {
        use super::special_sequence;

        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("? anything really ?"),
            Ok(("", " anything really "))
        );
        assert_eq!(
            special_sequence::<(&str, ErrorKind)>("?藏!? abc"),
            Ok((" abc", "藏!"))
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
        assert_eq!(special_sequence::<(&str, ErrorKind)>("??"), Ok(("", "")));
    }

    #[test]
    fn test_terminal_string() {
        use super::terminal_string;

        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("'a string'"),
            Ok(("", "a string"))
        );
        assert_eq!(
            terminal_string::<(&str, ErrorKind)>("\"some other string  \"abc"),
            Ok(("abc", "some other string  "))
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
    }
}

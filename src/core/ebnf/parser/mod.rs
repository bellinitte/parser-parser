// TODO decide what to do about the fact that a terminal can contain a newline vs a space.

use super::ast::{Expression, Grammar, Position, Production, Token};
use super::error::Result;
use error::Error;
use nom::{error::ErrorKind, IResult, InputTakeAtPosition};
use nom::{InputIter, Slice};
use span::Span;

pub mod error;
mod span;
#[cfg(test)]
mod tests;

/// Parses a non-zero sequence of decimal digits and returns a usize represented by that sequence.
///
/// # Example
///
/// ```rust
/// assert_eq!(integer::<(&str, ErrorKind)>("123"), Ok(("", 123)));
/// ```
fn integer(i: Span) -> IResult<Span, Token<usize>, Error> {
    let mut chars = i.iter_elements();
    let mut offset = 0;
    let mut integer: usize = 0;
    let start_offset = i.offset();

    match chars.next() {
        Some(c) if c.is_digit(10) => {
            integer = c.to_digit(10).unwrap() as usize;
            offset += c.len_utf8();
        }
        _ => Err(nom::Err::Error(Error::Internal(ErrorKind::Char)))?,
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
            }
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            }
            _ => break,
        };
    }

    Ok((
        i.slice(offset..),
        Token::new(integer, Position::new(start_offset..start_offset + offset)),
    ))
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
fn identifier(i: Span) -> IResult<Span, Token<String>, Error> {
    let mut chars = i.iter_elements();
    let mut offset = 0;
    let mut identifier: String = String::new();
    let start_offset = i.offset();

    match chars.next() {
        Some(c) if c.is_alphabetic() => {
            identifier.push(c);
            offset += c.len_utf8();
        }
        _ => Err(nom::Err::Error(Error::Internal(ErrorKind::Char)))?,
    };

    let mut last_whitespace_offset = 0;

    loop {
        match chars.next() {
            Some(c) if c.is_alphanumeric() => {
                identifier.push(c);
                offset += last_whitespace_offset;
                last_whitespace_offset = 0;
                offset += c.len_utf8();
            }
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            }
            _ => break,
        };
    }

    Ok((
        i.slice(offset..),
        Token::new(
            identifier,
            Position::new(start_offset..start_offset + offset),
        ),
    ))
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
fn optional_gap(i: Span) -> IResult<Span, (), Error> {
    use nom::combinator::opt;

    let (i, _) = i.split_at_position_complete(|c| !c.is_whitespace())?;
    let mut j = i;

    loop {
        let i = match opt(comment)(j)? {
            (i, Some(_)) => i,
            (i, None) => {
                return Ok((i, ()));
            }
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
fn special(i: Span) -> IResult<Span, Token<Expression>, Error> {
    let mut chars = i.iter_elements();
    let mut offset = 0;
    let mut sequence: String = String::new();
    let start_offset = i.offset();

    match chars.next() {
        Some(c) if c == '?' => {
            offset += c.len_utf8();
        }
        _ => Err(nom::Err::Error(Error::Internal(ErrorKind::Char)))?,
    };

    let mut last_whitespace_offset = 0;

    loop {
        match chars.next() {
            Some(c) if c == '?' => {
                offset += last_whitespace_offset;
                offset += c.len_utf8();
                let term = Token::new(
                    Expression::Special(sequence),
                    Position::new(start_offset..start_offset + offset),
                );
                return Ok((i.slice(offset..), term));
            }
            Some(c) if c.is_whitespace() => {
                last_whitespace_offset += c.len_utf8();
            }
            Some(c) if !c.is_control() => {
                sequence.push(c);
                offset += last_whitespace_offset;
                last_whitespace_offset = 0;
                offset += c.len_utf8();
            }
            _ => Err(nom::Err::Failure(Error::UnterminatedSpecial))?,
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
fn terminal(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{
        branch::alt,
        bytes::complete::take_till1,
        character::complete::char,
        combinator::cut,
        sequence::{preceded, terminated},
    };

    let start_offset = i.offset();
    let (i, s) = alt((
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
    ))(i)?;
    let end_offset = i.offset();

    Ok((
        i,
        Token::new(
            Expression::Terminal(s.fragment().to_owned()),
            Position::new(start_offset..end_offset),
        ),
    ))
}

fn nonterminal(i: Span) -> IResult<Span, Token<Expression>, Error> {
    let (i, token) = identifier(i)?;
    Ok((
        i,
        Token::new(Expression::Nonterminal(token.inner), token.position),
    ))
}

fn grouped(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{character::complete::char, sequence::delimited};

    let start_offset = i.offset();
    let (i, token) = delimited(char('('), alternative, char(')'))(i)?;
    let end_offset = i.offset();

    Ok((
        i,
        Token::new(token.inner, Position::new(start_offset..end_offset)),
    ))
}

fn repeated(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{branch::alt, bytes::complete::tag, sequence::delimited};

    let start_offset = i.offset();
    let (i, token) = delimited(
        alt((tag("{"), tag("(:"))),
        alternative,
        alt((tag("}"), tag(":)"))),
    )(i)?;
    let end_offset = i.offset();

    Ok((
        i,
        Token::new(
            Expression::Repeated(Box::new(token)),
            Position::new(start_offset..end_offset),
        ),
    ))
}

fn optional(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{branch::alt, bytes::complete::tag, sequence::delimited};

    let start_offset = i.offset();
    let (i, token) = delimited(
        alt((tag("["), tag("(/"))),
        alternative,
        alt((tag("]"), tag("/)"))),
    )(i)?;
    let end_offset = i.offset();

    Ok((
        i,
        Token::new(
            Expression::Optional(Box::new(token)),
            Position::new(start_offset..end_offset),
        ),
    ))
}

fn factor<'a>(i: Span<'a>) -> IResult<Span<'a>, Token<Expression>, Error> {
    use nom::{branch::alt, character::complete::char, combinator::opt};

    let pre_offset = i.offset();
    let (i, integer_token) = opt(|i: Span<'a>| -> IResult<Span<'a>, Token<usize>, Error> {
        let (i, _) = optional_gap(i)?;
        let (i, integer) = integer(i)?;
        let (i, _) = optional_gap(i)?;
        let (i, _) = char('*')(i)?;
        Ok((i, integer))
    })(i)?;

    let (i, _) = optional_gap(i)?;

    let start_offset = i.offset();
    let (i, token) = alt((
        optional,
        repeated,
        grouped,
        nonterminal,
        terminal,
        special,
        empty,
    ))(i)?;
    let end_offset = i.offset();

    let expression = match integer_token {
        Some(Token { inner: 0, .. }) => {
            Token::new(Expression::Empty, Position::new(pre_offset..end_offset))
        }
        Some(Token { inner: i, .. }) if i > 1 => {
            let factor = Expression::Factor {
                count: integer_token.unwrap(),
                primary: Box::new(token),
            };
            Token::new(factor, Position::new(start_offset..end_offset))
        }
        _ => token,
    };

    let (i, _) = optional_gap(i)?;

    Ok((i, expression))
}

fn empty(i: Span) -> IResult<Span, Token<Expression>, Error> {
    let offset = i.offset();
    Ok((
        i,
        Token::new(Expression::Empty, Position::new(offset..offset)),
    ))
}

fn term(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{character::complete::char, combinator::opt, sequence::preceded};

    let start_offset = i.offset();
    let (i, primary) = factor(i)?;
    // TODO a syntactic-factor that could be replaced by a syntactic-factor containing no meta-identifiers
    let (i, exception) = opt(preceded(char('-'), factor))(i)?;
    let end_offset = i.offset();

    Ok(match exception {
        None => (i, primary),
        Some(ex) => (
            i,
            Token::new(
                Expression::Exception {
                    subject: Box::new(primary),
                    restriction: Box::new(ex),
                },
                Position::new(start_offset..end_offset),
            ),
        ),
    })
}

fn sequence(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{character::complete::char, multi::separated_list1};

    let start_offset = i.offset();
    let (i, tokens) = separated_list1(char(','), term)(i)?;
    let end_offset = i.offset();

    assert!(tokens.len() > 0);

    let expression = match tokens.len() {
        1 => tokens[0].clone(),
        _ => Token::new(
            Expression::Sequence {
                first: Box::new(tokens[0].clone()),
                second: Box::new(tokens[1].clone()),
                rest: tokens[2..].to_vec(),
            },
            // TODO modify the range to be between the beginning of the first token and end
            // of the last token
            Position::new(start_offset..end_offset),
        ),
    };

    Ok((i, expression))
}

fn alternative(i: Span) -> IResult<Span, Token<Expression>, Error> {
    use nom::{character::complete::one_of, multi::separated_list1};

    let start_offset = i.offset();
    let (i, tokens) = separated_list1(one_of("|/!"), sequence)(i)?;
    let end_offset = i.offset();

    assert!(tokens.len() > 0);

    let expression = match tokens.len() {
        1 => tokens[0].clone(),
        _ => Token::new(
            Expression::Alternative {
                first: Box::new(tokens[0].clone()),
                second: Box::new(tokens[1].clone()),
                rest: tokens[2..].to_vec(),
            },
            Position::new(start_offset..end_offset),
        ),
    };

    Ok((i, expression))
}

fn production(i: Span) -> IResult<Span, Token<Production>, Error> {
    use nom::character::complete::{char, one_of};

    let (i, _) = optional_gap(i)?;
    let start_offset = i.offset();
    let (i, identifier) = identifier(i)?;
    let (i, _) = optional_gap(i)?;
    let (i, _) = char('=')(i)?;
    let (i, definitions) = alternative(i)?;
    let (i, _) = one_of(";.")(i)?;
    let end_offset = i.offset();
    let (i, _) = optional_gap(i)?;

    Ok((
        i,
        Token::new(
            Production {
                lhs: identifier,
                rhs: definitions,
            },
            Position::new(start_offset..end_offset),
        ),
    ))
}

fn syntax(i: Span) -> IResult<Span, Token<Grammar>, Error> {
    use nom::multi::many1;

    let start_offset = i.offset();
    let (i, tokens) = many1(production)(i)?;
    let end_offset = i.offset();

    Ok((
        i,
        Token::new(
            Grammar {
                productions: tokens,
            },
            Position::new(start_offset..end_offset),
        ),
    ))
}

fn comment(i: Span) -> IResult<Span, (), Error> {
    use nom::{bytes::complete::tag, combinator::opt};

    let (i, _) = tag("(*")(i)?;
    let mut j = i;

    loop {
        let (i, _) = opt(comment)(j)?;
        if let (i, Some(_)) = opt(tag("*)"))(i)? {
            return Ok((i, ()));
        };
        let i = match i.iter_elements().next() {
            Some(c) if !c.is_control() => i.slice(c.len_utf8()..),
            Some(_) => Err(nom::Err::Error(Error::Internal(ErrorKind::Char)))?,
            None => Err(nom::Err::Failure(Error::UnterminatedComment))?,
        };
        j = i;
    }
}

pub(super) fn parse<'a>(input: &'a str) -> Result<Grammar> {
    match syntax(Span::new(input)) {
        Ok((_, grammar)) => Ok(grammar.inner),
        Err(nom::Err::Failure(inner)) => Err(inner.into()),
        Err(nom::Err::Error(inner)) => Err(inner.into()),
        _ => unreachable!(),
    }
}

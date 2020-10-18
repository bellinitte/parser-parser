use super::{Error, ErrorKind, Expression, Span, Token, TokenKind, Tokens};
use nom::{error::ParseError, Err, IResult, InputIter, InputLength, Parser, Slice};

#[macro_export]
macro_rules! literal {
    ($name:ident, $kind:pat, $error:expr) => {
        pub fn $name(i: Tokens) -> IResult<Tokens, (TokenKind, Span), Error> {
            match i.iter_elements().next() {
                Some(Token {
                    kind: kind @ $kind,
                    span,
                }) => Ok((i.slice(1..), (kind, span))),
                Some(Token { span, .. }) => Err(Err::Error(Error { kind: $error, span })),
                None => Err(Err::Error(Error {
                    kind: $error,
                    span: i.last_span(),
                })),
            }
        }
    };
}

literal!(
    concatenation_symbol,
    TokenKind::Concatenation,
    ErrorKind::ConcatenationSymbolExpected
);
literal!(
    definition_symbol,
    TokenKind::Definition,
    ErrorKind::DefinitionSymbolExpected
);
literal!(
    definition_separator,
    TokenKind::DefinitionSeparator,
    ErrorKind::DefinitionSeparatorSymbolExpected
);
literal!(
    end_group_symbol,
    TokenKind::EndGroup,
    ErrorKind::EndGroupSymbolExpected
);
literal!(
    end_option_symbol,
    TokenKind::EndOption,
    ErrorKind::EndOptionSymbolExpected
);
literal!(
    end_repeat_symbol,
    TokenKind::EndRepeat,
    ErrorKind::EndRepeatSymbolExpected
);
literal!(
    exception_symbol,
    TokenKind::Exception,
    ErrorKind::ExceptionSymbolExpected
);
literal!(
    repetition_symbol,
    TokenKind::Repetition,
    ErrorKind::RepetitionSymbolExpected
);
literal!(
    start_group_symbol,
    TokenKind::StartGroup,
    ErrorKind::StartGroupSymbolExpected
);
literal!(
    start_option_symbol,
    TokenKind::StartOption,
    ErrorKind::StartOptionSymbolExpected
);
literal!(
    start_repeat_symbol,
    TokenKind::StartRepeat,
    ErrorKind::StartRepeatSymbolExpected
);
literal!(
    terminator_symbol,
    TokenKind::Terminator,
    ErrorKind::TerminatorSymbolExpected
);

pub fn identifier(i: Tokens) -> IResult<Tokens, (String, Span), Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), (s, span))),
        Some(Token { span, .. }) => Err(Err::Error(Error {
            kind: ErrorKind::IdentifierExpected,
            span,
        })),
        None => Err(Err::Error(Error {
            kind: ErrorKind::IdentifierExpected,
            span: i.last_span(),
        })),
    }
}

pub fn nonterminal(i: Tokens) -> IResult<Tokens, Expression, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((
            i.slice(1..),
            Expression::Nonterminal {
                identifier: s,
                span,
            },
        )),
        Some(Token { span, .. }) => Err(Err::Error(Error {
            kind: ErrorKind::NonterminalExpected,
            span,
        })),
        None => Err(Err::Error(Error {
            kind: ErrorKind::NonterminalExpected,
            span: i.last_span(),
        })),
    }
}

pub fn terminal(i: Tokens) -> IResult<Tokens, Expression, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Terminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Terminal { content: s, span })),
        Some(Token { span, .. }) => Err(Err::Error(Error {
            kind: ErrorKind::TerminalExpected,
            span,
        })),
        None => Err(Err::Error(Error {
            kind: ErrorKind::TerminalExpected,
            span: i.last_span(),
        })),
    }
}

pub fn special(i: Tokens) -> IResult<Tokens, Expression, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Special(s),
            span,
        }) => Ok((i.slice(1..), Expression::Special { content: s, span })),
        Some(Token { span, .. }) => Err(Err::Error(Error {
            kind: ErrorKind::SpecialExpected,
            span,
        })),
        None => Err(Err::Error(Error {
            kind: ErrorKind::SpecialExpected,
            span: i.last_span(),
        })),
    }
}

pub fn integer(i: Tokens) -> IResult<Tokens, (usize, Span), Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Integer(s),
            span,
        }) => Ok((i.slice(1..), (s, span))),
        Some(Token { span, .. }) => Err(Err::Error(Error {
            kind: ErrorKind::IntegerExpected,
            span,
        })),
        None => Err(Err::Error(Error {
            kind: ErrorKind::IntegerExpected,
            span: i.last_span(),
        })),
    }
}

pub fn empty(i: Tokens) -> IResult<Tokens, Expression, Error> {
    let span = match i.iter_elements().next() {
        Some(token) => Span::between(&i.last_span(), &token.span),
        None => i.last_span(),
    };
    Ok((i, Expression::Empty { span }))
}

pub fn non_eof<'a, O, F>(mut f: F) -> impl FnMut(Tokens<'a>) -> IResult<Tokens<'a>, O, Error>
where
    F: Parser<Tokens<'a>, O, Error>,
{
    move |input: Tokens| {
        if input.input_len() == 0 {
            Err(Err::Error(Error {
                kind: ErrorKind::IdentifierExpected,
                span: input.last_span(),
            }))
        } else {
            let (input, res) = f.parse(input)?;
            Ok((input, res))
        }
    }
}

pub fn separated_list1<I, O, O2, E, F, G>(
    mut sep: G,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: Clone + PartialEq,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    move |i: I| {
        let mut res = Vec::new();
        let mut i = i.clone();

        // Parse the first element
        match f.parse(i.clone()) {
            Err(e) => return Err(e),
            Ok((i1, o)) => {
                res.push(o);
                i = i1;
            }
        }

        loop {
            match sep.parse(i.clone()) {
                Err(Err::Error(_)) => return Ok((i, res)),
                Err(e) => return Err(e),
                Ok((i1, _)) => {
                    if i1 == i {
                        return Err(Err::Error(E::from_error_kind(
                            i1,
                            nom::error::ErrorKind::SeparatedList,
                        )));
                    }

                    match f.parse(i1.clone()) {
                        Err(Err::Error(_)) => return Ok((i, res)),
                        Err(e) => return Err(e),
                        Ok((i2, o)) => {
                            if i2 == i {
                                return Err(Err::Error(E::from_error_kind(
                                    i2,
                                    nom::error::ErrorKind::SeparatedList,
                                )));
                            }

                            res.push(o);
                            i = i2;
                        }
                    }
                }
            }
        }
    }
}

pub fn map_err<I, O, E1, E2, F, G>(mut first: F, second: G) -> impl FnMut(I) -> IResult<I, O, E2>
where
    F: Parser<I, O, E1>,
    G: Fn(E1) -> E2,
{
    move |input: I| match first.parse(input) {
        Ok(t) => Ok(t),
        Err(nom::Err::Incomplete(n)) => Err(nom::Err::Incomplete(n)),
        Err(nom::Err::Error(e)) => Err(nom::Err::Error(second(e))),
        Err(nom::Err::Failure(e)) => Err(nom::Err::Failure(second(e))),
    }
}

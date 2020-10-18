use super::{Error, Expression, Span, Spanned, Spanning, Token, Tokens};
use nom::{error::ParseError, Err, IResult, InputIter, InputLength, Parser, Slice};

#[macro_export]
macro_rules! literal {
    ($name:ident, $kind:pat, $error:expr) => {
        pub fn $name(i: Tokens) -> IResult<Tokens, Spanned<Token>, Spanned<Error>> {
            match i.iter_elements().next() {
                Some(Spanned {
                    node: kind @ $kind,
                    span,
                }) => Ok((i.slice(1..), kind.spanning(span))),
                Some(Spanned { span, .. }) => Err(Err::Error($error.spanning(span))),
                None => Err(Err::Error($error.spanning(i.last_span()))),
            }
        }
    };
}

literal!(
    concatenation_symbol,
    Token::Concatenation,
    Error::ConcatenationSymbolExpected
);
literal!(
    definition_symbol,
    Token::Definition,
    Error::DefinitionSymbolExpected
);
literal!(
    definition_separator,
    Token::DefinitionSeparator,
    Error::DefinitionSeparatorSymbolExpected
);
literal!(
    end_group_symbol,
    Token::EndGroup,
    Error::EndGroupSymbolExpected
);
literal!(
    end_option_symbol,
    Token::EndOption,
    Error::EndOptionSymbolExpected
);
literal!(
    end_repeat_symbol,
    Token::EndRepeat,
    Error::EndRepeatSymbolExpected
);
literal!(
    exception_symbol,
    Token::Exception,
    Error::ExceptionSymbolExpected
);
literal!(
    repetition_symbol,
    Token::Repetition,
    Error::RepetitionSymbolExpected
);
literal!(
    start_group_symbol,
    Token::StartGroup,
    Error::StartGroupSymbolExpected
);
literal!(
    start_option_symbol,
    Token::StartOption,
    Error::StartOptionSymbolExpected
);
literal!(
    start_repeat_symbol,
    Token::StartRepeat,
    Error::StartRepeatSymbolExpected
);
literal!(
    terminator_symbol,
    Token::Terminator,
    Error::TerminatorSymbolExpected
);

pub fn identifier(i: Tokens) -> IResult<Tokens, Spanned<String>, Spanned<Error>> {
    match i.iter_elements().next() {
        Some(Spanned {
            node: Token::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), s.spanning(span))),
        Some(Spanned { span, .. }) => Err(Err::Error(Error::IdentifierExpected.spanning(span))),
        None => Err(Err::Error(
            Error::IdentifierExpected.spanning(i.last_span()),
        )),
    }
}

pub fn nonterminal(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    match i.iter_elements().next() {
        Some(Spanned {
            node: Token::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Nonterminal(s).spanning(span))),
        Some(Spanned { span, .. }) => Err(Err::Error(Error::NonterminalExpected.spanning(span))),
        None => Err(Err::Error(
            Error::NonterminalExpected.spanning(i.last_span()),
        )),
    }
}

pub fn terminal(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    match i.iter_elements().next() {
        Some(Spanned {
            node: Token::Terminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Terminal(s).spanning(span))),
        Some(Spanned { span, .. }) => Err(Err::Error(Error::TerminalExpected.spanning(span))),
        None => Err(Err::Error(Error::TerminalExpected.spanning(i.last_span()))),
    }
}

pub fn special(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    match i.iter_elements().next() {
        Some(Spanned {
            node: Token::Special(s),
            span,
        }) => Ok((i.slice(1..), Expression::Special(s).spanning(span))),
        Some(Spanned { span, .. }) => Err(Err::Error(Error::SpecialExpected.spanning(span))),
        None => Err(Err::Error(Error::SpecialExpected.spanning(i.last_span()))),
    }
}

pub fn integer(i: Tokens) -> IResult<Tokens, Spanned<usize>, Spanned<Error>> {
    match i.iter_elements().next() {
        Some(Spanned {
            node: Token::Integer(s),
            span,
        }) => Ok((i.slice(1..), s.spanning(span))),
        Some(Spanned { span, .. }) => Err(Err::Error(Error::IntegerExpected.spanning(span))),
        None => Err(Err::Error(Error::IntegerExpected.spanning(i.last_span()))),
    }
}

pub fn empty(i: Tokens) -> IResult<Tokens, Spanned<Expression>, Spanned<Error>> {
    let span = match i.iter_elements().next() {
        Some(token) => Span::between(&i.last_span(), &token.span),
        None => i.last_span(),
    };
    Ok((i, Expression::Empty.spanning(span)))
}

pub fn non_eof<'a, O, F>(
    mut f: F,
) -> impl FnMut(Tokens<'a>) -> IResult<Tokens<'a>, O, Spanned<Error>>
where
    F: Parser<Tokens<'a>, O, Spanned<Error>>,
{
    move |input: Tokens| {
        if input.input_len() == 0 {
            Err(Err::Error(
                Error::IdentifierExpected.spanning(input.last_span()),
            ))
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

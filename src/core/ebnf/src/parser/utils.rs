use super::{Error, ErrorKind, Expression, Node, NodeAt, Span, Token, TokenKind, Tokens};
use nom::{Err, IResult, InputIter, Parser, Slice};

#[macro_export]
macro_rules! literal {
    ($name:ident, $kind:pat, $error:expr) => {
        pub fn $name(i: Tokens) -> IResult<Tokens, Node<TokenKind>, Error> {
            match i.iter_elements().next() {
                Some(Token {
                    kind: kind @ $kind,
                    span,
                }) => Ok((i.slice(1..), kind.node_at(span))),
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

pub fn identifier(i: Tokens) -> IResult<Tokens, Node<String>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), s.node_at(span))),
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

pub fn nonterminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Nonterminal(s).node_at(span))),
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

pub fn terminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Terminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Terminal(s).node_at(span))),
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

pub fn special(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Special(s),
            span,
        }) => Ok((i.slice(1..), Expression::Special(s).node_at(span))),
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

pub fn integer(i: Tokens) -> IResult<Tokens, Node<usize>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Integer(s),
            span,
        }) => Ok((i.slice(1..), s.node_at(span))),
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

pub fn empty(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    let span = match i.iter_elements().next() {
        Some(token) => Span::between(&i.last_span(), &token.span),
        None => i.last_span(),
    };
    Ok((i, Expression::Empty.node_at(span)))
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

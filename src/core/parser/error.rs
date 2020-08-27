use nom::error::{ContextError, ErrorKind, ParseError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParserParserError<I> {
    pub errors: Vec<(I, ParserParserErrorKind)>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ParserParserErrorKind {
    Context(&'static str),
    Nom(ErrorKind),
}

impl<I> ParseError<I> for ParserParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        ParserParserError {
            errors: vec![(input, ParserParserErrorKind::Nom(kind))],
        }
    }

    fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
        other.errors.push((input, ParserParserErrorKind::Nom(kind)));
        other
    }
}

impl<I> ContextError<I> for ParserParserError<I> {
    fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
        other
            .errors
            .push((input, ParserParserErrorKind::Context(ctx)));
        other
    }
}

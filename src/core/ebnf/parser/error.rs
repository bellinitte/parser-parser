use nom::error::{ErrorKind, ParseError};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    IdentifierExpected,
    NonterminalExpected,
    TerminalExpected,
    SpecialExpected,
    IntegerExpected,
    ConcatenationExpected,
    DefinitionExpected,
    DefinitionSeparatorExpected,
    EndGroupExpected,
    EndOptionExpected,
    EndRepeatExpected,
    ExceptionExpected,
    RepetitionExpected,
    StartGroupExpected,
    StartOptionExpected,
    StartRepeatExpected,
    TerminatorExpected,
    Unknown,
}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, _: ErrorKind) -> Self {
        Error::Unknown
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IdentifierExpected => write!(f, "identifier expected"),
            Error::NonterminalExpected => write!(f, "nonterminal expected"),
            Error::TerminalExpected => write!(f, "terminal expected"),
            Error::SpecialExpected => write!(f, "special sequence expected"),
            Error::IntegerExpected => write!(f, "integer expected"),
            Error::ConcatenationExpected => write!(f, "concatenation symbol expected"),
            Error::DefinitionExpected => write!(f, "definition symbol expected"),
            Error::DefinitionSeparatorExpected => write!(f, "definition separator symbol expected"),
            Error::EndGroupExpected => write!(f, "end group symbol expected"),
            Error::EndOptionExpected => write!(f, "end option symbol expected"),
            Error::EndRepeatExpected => write!(f, "end repeat symbol expected"),
            Error::ExceptionExpected => write!(f, "exception symbol expected"),
            Error::RepetitionExpected => write!(f, "repetition symbol expected"),
            Error::StartGroupExpected => write!(f, "start group symbol expected"),
            Error::StartOptionExpected => write!(f, "start option symbol expected"),
            Error::StartRepeatExpected => write!(f, "start repeat symbol expected"),
            Error::TerminatorExpected => write!(f, "terminator symbol expected"),
            Error::Unknown => write!(f, "unknown error"),
        }
    }
}

impl std::error::Error for Error {}

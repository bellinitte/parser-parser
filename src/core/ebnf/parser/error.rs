use nom::error::{ErrorKind, ParseError};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    DefinitionExpected,
    IdentifierExpected,
    NonterminalExpected,
    TerminalExpected,
    SpecialExpected,
    IntegerExpected,
    ConcatenationSymbolExpected,
    DefinitionSymbolExpected,
    DefinitionSeparatorSymbolExpected,
    EndGroupSymbolExpected,
    EndOptionSymbolExpected,
    EndRepeatSymbolExpected,
    ExceptionSymbolExpected,
    RepetitionSymbolExpected,
    StartGroupSymbolExpected,
    StartOptionSymbolExpected,
    StartRepeatSymbolExpected,
    TerminatorSymbolExpected,
    Nom(ErrorKind),
}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, e: ErrorKind) -> Self {
        Error::Nom(e)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::DefinitionExpected => write!(f, "definition expected"),
            Error::IdentifierExpected => write!(f, "identifier expected"),
            Error::NonterminalExpected => write!(f, "nonterminal expected"),
            Error::TerminalExpected => write!(f, "terminal expected"),
            Error::SpecialExpected => write!(f, "special sequence expected"),
            Error::IntegerExpected => write!(f, "integer expected"),
            Error::ConcatenationSymbolExpected => write!(f, "concatenation symbol expected"),
            Error::DefinitionSymbolExpected => write!(f, "definition symbol expected"),
            Error::DefinitionSeparatorSymbolExpected => {
                write!(f, "definition separator symbol expected")
            }
            Error::EndGroupSymbolExpected => write!(f, "end group symbol expected"),
            Error::EndOptionSymbolExpected => write!(f, "end option symbol expected"),
            Error::EndRepeatSymbolExpected => write!(f, "end repeat symbol expected"),
            Error::ExceptionSymbolExpected => write!(f, "exception symbol expected"),
            Error::RepetitionSymbolExpected => write!(f, "repetition symbol expected"),
            Error::StartGroupSymbolExpected => write!(f, "start group symbol expected"),
            Error::StartOptionSymbolExpected => write!(f, "start option symbol expected"),
            Error::StartRepeatSymbolExpected => write!(f, "start repeat symbol expected"),
            Error::TerminatorSymbolExpected => write!(f, "terminator symbol expected"),
            Error::Nom(_) => write!(f, "internal error"),
        }
    }
}

impl std::error::Error for Error {}

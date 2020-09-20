use super::Tokens;
use nom::InputIter;
use std::{fmt, ops::Range};

#[derive(Debug, PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Range<usize>,
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
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
    Nom(nom::error::ErrorKind),
}

impl<'a> nom::error::ParseError<Tokens<'a>> for Error {
    fn from_error_kind(input: Tokens<'a>, e: nom::error::ErrorKind) -> Self {
        let position = match input.iter_elements().next() {
            Some(token) => token.span,
            None => input.offset()..input.offset() + 1,
        };
        Error {
            kind: ErrorKind::Nom(e),
            position,
        }
    }

    fn append(_: Tokens<'a>, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::DefinitionExpected => write!(f, "definition expected"),
            ErrorKind::IdentifierExpected => write!(f, "identifier expected"),
            ErrorKind::NonterminalExpected => write!(f, "nonterminal expected"),
            ErrorKind::TerminalExpected => write!(f, "terminal expected"),
            ErrorKind::SpecialExpected => write!(f, "special sequence expected"),
            ErrorKind::IntegerExpected => write!(f, "integer expected"),
            ErrorKind::ConcatenationSymbolExpected => write!(f, "concatenation symbol expected"),
            ErrorKind::DefinitionSymbolExpected => write!(f, "definition symbol expected"),
            ErrorKind::DefinitionSeparatorSymbolExpected => {
                write!(f, "definition separator symbol expected")
            }
            ErrorKind::EndGroupSymbolExpected => write!(f, "end group symbol expected"),
            ErrorKind::EndOptionSymbolExpected => write!(f, "end option symbol expected"),
            ErrorKind::EndRepeatSymbolExpected => write!(f, "end repeat symbol expected"),
            ErrorKind::ExceptionSymbolExpected => write!(f, "exception symbol expected"),
            ErrorKind::RepetitionSymbolExpected => write!(f, "repetition symbol expected"),
            ErrorKind::StartGroupSymbolExpected => write!(f, "start group symbol expected"),
            ErrorKind::StartOptionSymbolExpected => write!(f, "start option symbol expected"),
            ErrorKind::StartRepeatSymbolExpected => write!(f, "start repeat symbol expected"),
            ErrorKind::TerminatorSymbolExpected => write!(f, "terminator symbol expected"),
            ErrorKind::Nom(_) => write!(f, "internal error"),
        }
    }
}

impl std::error::Error for Error {}

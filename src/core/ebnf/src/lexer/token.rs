use std::{fmt, ops::Range};
use super::{Location, Span};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol<'a> {
    pub grapheme: &'a str,
    pub span: Span,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Token {
        Token { kind, span }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Nonterminal(String),
    Terminal(String),
    Special(String),
    Integer(usize),
    Concatenation,
    Definition,
    DefinitionSeparator,
    EndGroup,
    EndOption,
    EndRepeat,
    Exception,
    Repetition,
    StartGroup,
    StartOption,
    StartRepeat,
    Terminator,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Nonterminal(s) => write!(f, "nonterminal '{}'", s),
            TokenKind::Terminal(s) => write!(f, "terminal '{}'", s),
            TokenKind::Special(s) => write!(f, "special sequence '?{}?'", s),
            TokenKind::Integer(i) => write!(f, "integer '{}'", i),
            TokenKind::Concatenation => write!(f, "cocatenation symbol"),
            TokenKind::Definition => write!(f, "definition symbol"),
            TokenKind::DefinitionSeparator => write!(f, "definition separator symbol"),
            TokenKind::EndGroup => write!(f, "end group symbol"),
            TokenKind::EndOption => write!(f, "end option symbol"),
            TokenKind::EndRepeat => write!(f, "end repeat symbol"),
            TokenKind::Exception => write!(f, "exception symbol"),
            TokenKind::Repetition => write!(f, "repetition symbol"),
            TokenKind::StartGroup => write!(f, "start group symbol"),
            TokenKind::StartOption => write!(f, "start option symbol"),
            TokenKind::StartRepeat => write!(f, "start repeat symbol"),
            TokenKind::Terminator => write!(f, "terminator symbol"),
        }
    }
}

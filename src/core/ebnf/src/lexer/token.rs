use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Nonterminal(s) => write!(f, "nonterminal '{}'", s),
            Token::Terminal(s) => write!(f, "terminal '{}'", s),
            Token::Special(s) => write!(f, "special sequence '?{}?'", s),
            Token::Integer(i) => write!(f, "integer '{}'", i),
            Token::Concatenation => write!(f, "cocatenation symbol"),
            Token::Definition => write!(f, "definition symbol"),
            Token::DefinitionSeparator => write!(f, "definition separator symbol"),
            Token::EndGroup => write!(f, "end group symbol"),
            Token::EndOption => write!(f, "end option symbol"),
            Token::EndRepeat => write!(f, "end repeat symbol"),
            Token::Exception => write!(f, "exception symbol"),
            Token::Repetition => write!(f, "repetition symbol"),
            Token::StartGroup => write!(f, "start group symbol"),
            Token::StartOption => write!(f, "start option symbol"),
            Token::StartRepeat => write!(f, "start repeat symbol"),
            Token::Terminator => write!(f, "terminator symbol"),
        }
    }
}

impl_spanning!(Token);

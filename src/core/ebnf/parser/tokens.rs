use super::super::lexer::{Token, TokenKind};
use nom::{
    Compare, CompareResult, FindSubstring, FindToken, InputIter, InputLength, InputTake, Slice,
    UnspecializedInput,
};
use std::iter::{Enumerate, Map};
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct Tokens<'a> {
    inner: &'a [Token],
    offset: usize, // used in the empty expression
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token]) -> Tokens<'a> {
        Tokens {
            inner: tokens,
            offset: 0,
        }
    }
}

impl<'a> PartialEq for Tokens<'a> {
    fn eq(&self, other: &Tokens) -> bool {
        self.inner == other.inner
    }
}

impl<'a> Eq for Tokens<'a> {}

impl<'a> InputLength for Tokens<'a> {
    fn input_len(&self) -> usize {
        self.inner.len()
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = Token;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Map<Iter<'a, Self::Item>, fn(&Token) -> Token>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.inner.iter().map(|t: &Token| t.clone())
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.inner.iter().position(|t| predicate(t.clone()))
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        if self.inner.len() >= count {
            Some(count)
        } else {
            None
        }
    }
}

impl<'a> InputTake for Tokens<'a>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        Tokens {
            inner: &self.inner[0..count],
            offset: self.offset,
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.inner.split_at(count);
        let offset = if count > 0 {
            self.inner[count - 1].span.end
        } else {
            self.offset
        };
        (
            Tokens {
                inner: suffix,
                offset: self.offset,
            },
            Tokens {
                inner: prefix,
                offset,
            },
        )
    }
}

impl<'a> UnspecializedInput for Tokens<'a> {}

impl<'a> Compare<Tokens<'a>> for Tokens<'a> {
    #[inline(always)]
    fn compare(&self, other: Tokens<'a>) -> CompareResult {
        let pos = self
            .inner
            .iter()
            .zip(other.inner.iter())
            .position(|(a, b)| a != b);

        match pos {
            Some(_) => CompareResult::Error,
            None => {
                if self.inner.len() >= other.inner.len() {
                    CompareResult::Ok
                } else {
                    CompareResult::Incomplete
                }
            }
        }
    }

    #[inline(always)]
    fn compare_no_case(&self, other: Tokens<'a>) -> CompareResult {
        self.compare(other)
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    fn slice(&self, range: Range<usize>) -> Tokens<'a> {
        let offset = if range.start > 0 {
            self.inner[range.start - 1].span.end
        } else {
            self.offset
        };
        Tokens {
            inner: &self.inner[range],
            offset,
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Tokens<'a> {
        Tokens {
            inner: &self.inner[range],
            offset: self.offset,
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Tokens<'a> {
        let offset = if range.start > 0 {
            self.inner[range.start - 1].span.end
        } else {
            self.offset
        };
        Tokens {
            inner: &self.inner[range],
            offset,
        }
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, range: RangeFull) -> Tokens<'a> {
        Tokens {
            inner: &self.inner[range],
            offset: self.offset,
        }
    }
}

impl<'a> FindToken<Token> for Tokens<'a> {
    fn find_token(&self, token: Token) -> bool {
        self.inner.contains(&token)
    }
}

impl<'a> FindSubstring<&'a [Token]> for Tokens<'a> {
    #[inline]
    fn find_substring(&self, substr: &'a [Token]) -> Option<usize> {
        let substr_len = substr.len();

        if substr_len == 0 {
            Some(0)
        } else if substr_len == 1 {
            self.inner.iter().position(|t| t == &substr[0])
        } else if substr_len > self.inner.len() {
            None
        } else {
            let max = self.inner.len() - substr_len;
            let mut offset = 0;
            let mut haystack = &self.inner[..];

            while let Some(position) = haystack.iter().position(|t| t == &substr[0]) {
                offset += position;

                if offset > max {
                    return None;
                }

                if &haystack[position..position + substr_len] == substr {
                    return Some(offset);
                }

                haystack = &haystack[position + 1..];
                offset += 1;
            }

            None
        }
    }
}

use super::Error;
use super::NodeAt;
use super::{Expression, Node};
use nom::Err;
use nom::IResult;

#[macro_export]
macro_rules! literal {
    ($name:ident, $kind:pat, $error:expr) => {
        pub fn $name(i: Tokens) -> IResult<Tokens, Node<TokenKind>, Error> {
            match i.iter_elements().next() {
                Some(Token {
                    kind: kind @ $kind,
                    span,
                }) => Ok((i.slice(1..), kind.node_at(span))),
                _ => Err(Err::Error($error)),
            }
        }
    };
}

literal!(concatenation, TokenKind::Concatenation, Error::ConcatenationExpected);
literal!(definition, TokenKind::Definition, Error::DefinitionExpected);
literal!(definition_separator, TokenKind::DefinitionSeparator, Error::DefinitionSeparatorExpected);
literal!(end_group, TokenKind::EndGroup, Error::EndGroupExpected);
literal!(end_option, TokenKind::EndOption, Error::EndOptionExpected);
literal!(end_repeat, TokenKind::EndRepeat, Error::EndRepeatExpected);
literal!(exception, TokenKind::Exception, Error::ExceptionExpected);
literal!(repetition, TokenKind::Repetition, Error::RepetitionExpected);
literal!(start_group, TokenKind::StartGroup, Error::StartGroupExpected);
literal!(start_option, TokenKind::StartOption, Error::StartOptionExpected);
literal!(start_repeat, TokenKind::StartRepeat, Error::StartRepeatExpected);
literal!(terminator, TokenKind::Terminator, Error::TerminatorExpected);

pub fn identifier(i: Tokens) -> IResult<Tokens, Node<String>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), s.node_at(span))),
        _ => Err(Err::Error(Error::IdentifierExpected)),
    }
}

pub fn nonterminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Nonterminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Nonterminal(s).node_at(span))),
        _ => Err(Err::Error(Error::NonterminalExpected)),
    }
}

pub fn terminal(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Terminal(s),
            span,
        }) => Ok((i.slice(1..), Expression::Terminal(s).node_at(span))),
        _ => Err(Err::Error(Error::TerminalExpected)),
    }
}

pub fn special(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Special(s),
            span,
        }) => Ok((i.slice(1..), Expression::Special(s).node_at(span))),
        _ => Err(Err::Error(Error::SpecialExpected)),
    }
}

pub fn integer(i: Tokens) -> IResult<Tokens, Node<usize>, Error> {
    match i.iter_elements().next() {
        Some(Token {
            kind: TokenKind::Integer(s),
            span,
        }) => Ok((i.slice(1..), s.node_at(span))),
        _ => Err(Err::Error(Error::IntegerExpected)),
    }
}

pub fn empty(i: Tokens) -> IResult<Tokens, Node<Expression>, Error> {
    let start = i.offset;
    let end = match i.iter_elements().next() {
        Some(token) => token.span.start,
        None => start,
    };
    Ok((i, Expression::Empty.node_at(start..end)))
}

use super::Token;
use nom::{
    Compare, CompareResult, FindSubstring, FindToken, InputIter, InputLength, InputTake, Slice,
    UnspecializedInput,
};
use std::{
    iter::{Enumerate, Map},
    ops::{Range, RangeFrom, RangeFull, RangeTo},
    slice::Iter,
};

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

    pub fn offset(&self) -> usize {
        self.offset
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

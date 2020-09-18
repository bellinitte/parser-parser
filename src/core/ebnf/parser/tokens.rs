use super::super::lexer::Token;
use nom::{
    AsBytes, Compare, CompareResult, FindSubstring, FindToken, InputIter, InputLength, InputTake,
    Offset, ParseTo, Slice, UnspecializedInput,
};
use std::iter::{Enumerate, Map};
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::slice::Iter;

#[derive(Clone)]
pub struct Tokens<'a> {
    inner: &'a [Token],
}

impl<'a> Tokens<'a> {
    pub fn new(tokens: &'a [Token]) -> Tokens<'a> {
        Tokens { inner: tokens }
    }
}

// impl<'a> From<&'a str> for Span<'a> {
//     fn from(i: &'a str) -> Span<'a> {
//         Span::new(i)
//     }
// }

impl<'a> PartialEq for Tokens<'a> {
    fn eq(&self, other: &Tokens) -> bool {
        self.inner == other.inner
    }
}

impl<'a> Eq for Tokens<'a> {}

// impl<'a> AsBytes for Span<'a> {
//     fn as_bytes(&self) -> &[u8] {
//         self.fragment.as_bytes()
//     }
// }

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
        }
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.inner.split_at(count);
        (Tokens { inner: suffix }, Tokens { inner: prefix })
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
        Tokens {
            inner: &self.inner[range],
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Tokens<'a> {
        Tokens {
            inner: &self.inner[range],
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Tokens<'a> {
        Tokens {
            inner: &self.inner[range],
        }
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    fn slice(&self, range: RangeFull) -> Tokens<'a> {
        Tokens {
            inner: &self.inner[range],
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

// impl<'a, R: FromStr> ParseTo<R> for Tokens<'a> {
//     #[inline]
//     fn parse_to(&self) -> Option<R> {
//         self.fragment.parse_to()
//     }
// }

// impl<'a> Offset for Tokens<'a> {
//     fn offset(&self, second: &Self) -> usize {
//         let fst = self.inner[0].range.start;
//         let snd = second.offset;

//         snd - fst
//     }
// }

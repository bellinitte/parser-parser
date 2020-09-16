use nom::{
    AsBytes, Compare, CompareResult, FindSubstring, FindToken, InputIter, InputLength, InputTake,
    Offset, ParseTo, Slice, UnspecializedInput,
};
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::str::{CharIndices, Chars, FromStr};

#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    pub offset: usize,
    pub fragment: &'a str,
}

// impl<'a> core::ops::Deref for Span<'a> {
//     type Target = &'a str;
//     fn deref(&self) -> &&'a str {
//         &self.fragment
//     }
// }

impl<'a> Span<'a> {
    pub fn new(inner: &'a str) -> Span<'a> {
        Span {
            offset: 0,
            fragment: inner,
        }
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn fragment(&self) -> &'a str {
        &self.fragment
    }
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(i: &'a str) -> Span<'a> {
        Span::new(i)
    }
}

impl<'a> PartialEq for Span<'a> {
    fn eq(&self, other: &Span<'a>) -> bool {
        self.offset == other.offset && self.fragment == other.fragment
    }
}

impl<'a> Eq for Span<'a> {}

impl<'a> AsBytes for Span<'a> {
    fn as_bytes(&self) -> &[u8] {
        self.fragment.as_bytes()
    }
}

impl<'a> InputLength for Span<'a> {
    fn input_len(&self) -> usize {
        self.fragment.input_len()
    }
}

impl<'a> InputIter for Span<'a> {
    type Item = char;
    type Iter = CharIndices<'a>;
    type IterElem = Chars<'a>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.fragment.iter_indices()
    }

    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.fragment.iter_elements()
    }

    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.fragment.position(predicate)
    }

    #[inline]
    fn slice_index(&self, count: usize) -> Option<usize> {
        self.fragment.slice_index(count)
    }
}

impl<'a> InputTake for Span<'a>
where
    Self: Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
{
    fn take(&self, count: usize) -> Self {
        self.slice(..count)
    }

    fn take_split(&self, count: usize) -> (Self, Self) {
        (self.slice(count..), self.slice(..count))
    }
}

impl<'a> UnspecializedInput for Span<'a> {}

impl<'a> Compare<&'a str> for Span<'a> {
    #[inline(always)]
    fn compare(&self, t: &'a str) -> CompareResult {
        self.fragment.compare(t)
    }

    #[inline(always)]
    fn compare_no_case(&self, t: &'a str) -> CompareResult {
        self.fragment.compare_no_case(t)
    }
}

impl<'a> Slice<Range<usize>> for Span<'a> {
    fn slice(&self, range: Range<usize>) -> Span<'a> {
        Span {
            offset: self.offset + range.start,
            fragment: &self.fragment.slice(range),
        }
    }
}

impl<'a> Slice<RangeTo<usize>> for Span<'a> {
    fn slice(&self, range: RangeTo<usize>) -> Span<'a> {
        Span {
            offset: self.offset,
            fragment: &self.fragment.slice(range),
        }
    }
}

impl<'a> Slice<RangeFrom<usize>> for Span<'a> {
    fn slice(&self, range: RangeFrom<usize>) -> Span<'a> {
        Span {
            offset: self.offset + range.start,
            fragment: &self.fragment.slice(range),
        }
    }
}

impl<'a> Slice<RangeFull> for Span<'a> {
    fn slice(&self, range: RangeFull) -> Span<'a> {
        Span {
            offset: self.offset,
            fragment: &self.fragment.slice(range),
        }
    }
}

impl<'a> FindToken<char> for Span<'a> {
    fn find_token(&self, token: char) -> bool {
        self.fragment.find_token(token)
    }
}

impl<'a> FindSubstring<&'a str> for Span<'a> {
    #[inline]
    fn find_substring(&self, substr: &'a str) -> Option<usize> {
        self.fragment.find_substring(substr)
    }
}

impl<'a, R: FromStr> ParseTo<R> for Span<'a> {
    #[inline]
    fn parse_to(&self) -> Option<R> {
        self.fragment.parse_to()
    }
}

impl<'a> Offset for Span<'a> {
    fn offset(&self, second: &Self) -> usize {
        let fst = self.offset;
        let snd = second.offset;

        snd - fst
    }
}

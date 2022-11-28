use std::{
    iter::{Copied, Enumerate},
    ops::{Range, RangeFrom, RangeFull, RangeTo},
};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use crate::lexer::Token;

// Using custom struct as an input format requires implementations for the following traits
// https://github.com/Geal/nom/blob/main/doc/custom_input_types.md

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tokens<'a>(pub &'a [Token<'a>]);

impl<'a> InputLength for Tokens<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.input_len()
    }
}

impl<'a> InputTake for Tokens<'a> {
    #[inline]
    fn take(&self, count: usize) -> Self {
        Self(&self.0[..count])
    }

    #[inline]
    fn take_split(&self, count: usize) -> (Self, Self) {
        let (prefix, suffix) = self.0.split_at(count);
        (Self(suffix), Self(prefix))
    }
}

impl<'a> InputLength for Token<'a> {
    #[inline]
    fn input_len(&self) -> usize {
        1
    }
}

impl<'a> Slice<Range<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'a> Slice<RangeTo<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'a> Slice<RangeFrom<usize>> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'a> Slice<RangeFull> for Tokens<'a> {
    #[inline]
    fn slice(&self, range: RangeFull) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'a> InputIter for Tokens<'a> {
    type Item = Token<'a>;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Copied<::std::slice::Iter<'a, Token<'a>>>;

    #[inline]
    fn iter_indices(&self) -> Self::Iter {
        self.iter_elements().enumerate()
    }
    #[inline]
    fn iter_elements(&self) -> Self::IterElem {
        self.0.iter().copied()
    }
    #[inline]
    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.iter().position(|b| predicate(*b))
    }
    #[inline]
    fn slice_index(&self, count: usize) -> Result<usize, Needed> {
        if self.0.len() >= count {
            Ok(count)
        } else {
            Err(Needed::new(count - self.0.len()))
        }
    }
}

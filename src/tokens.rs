use std::{
    iter::{Copied, Enumerate},
    ops::{Range, RangeFrom, RangeFull, RangeTo},
};

use nom::{InputIter, InputLength, InputTake, Needed, Slice};

use crate::lexer::Token;

// Using custom struct as an input format requires implementations for the following traits
// https://github.com/Geal/nom/blob/main/doc/custom_input_types.md

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Tokens<'slice, 'token>(pub &'slice [Token<'token>]);

impl<'slice, 'token> From<Tokens<'slice, 'token>> for &'token str {
    fn from(value: Tokens<'slice, 'token>) -> Self {
        if value.0.is_empty() {
            return "";
        }
        let start_ptr = value.0[0].trivia.as_ptr();
        let end = value.0.last().unwrap().trivia;

        // Assumes all tokens are from the same string slice and are serially ordered
        // which must be true as this struct is only produced from the lexer module.
        unsafe {
            let end_ptr = end.as_ptr().add(end.len());

            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                start_ptr,
                end_ptr.offset_from(start_ptr) as usize,
            ))
        }
    }
}

impl<'slice, 'token> InputLength for Tokens<'slice, 'token> {
    #[inline]
    fn input_len(&self) -> usize {
        self.0.input_len()
    }
}

impl<'slice, 'token> InputTake for Tokens<'slice, 'token> {
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

impl<'slice, 'token> Slice<Range<usize>> for Tokens<'slice, 'token> {
    #[inline]
    fn slice(&self, range: Range<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'slice, 'token> Slice<RangeTo<usize>> for Tokens<'slice, 'token> {
    #[inline]
    fn slice(&self, range: RangeTo<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'slice, 'token> Slice<RangeFrom<usize>> for Tokens<'slice, 'token> {
    #[inline]
    fn slice(&self, range: RangeFrom<usize>) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'slice, 'token> Slice<RangeFull> for Tokens<'slice, 'token> {
    #[inline]
    fn slice(&self, range: RangeFull) -> Self {
        Self(self.0.slice(range))
    }
}

impl<'slice, 'token> InputIter for Tokens<'slice, 'token> {
    type Item = Token<'token>;
    type Iter = Enumerate<Self::IterElem>;
    type IterElem = Copied<::std::slice::Iter<'slice, Token<'token>>>;

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

// This exists because nom::error::Error doesn't have a From/Into implementation and nom::combinator::into requires it.
pub fn nom_error_into<T, U: From<T>>(
    err: nom::Err<nom::error::Error<T>>,
) -> nom::Err<nom::error::Error<U>> {
    match err {
        nom::Err::Incomplete(need) => nom::Err::Incomplete(need),
        nom::Err::Error(err) => nom::Err::Error(nom::error::Error {
            code: err.code,
            input: err.input.into(),
        }),
        nom::Err::Failure(err) => nom::Err::Failure(nom::error::Error {
            code: err.code,
            input: err.input.into(),
        }),
    }
}

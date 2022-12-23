use super::impl_nom_traits::Tokens;
use crate::lexer::Token;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VariantToken<'a, T> {
    pub variant: T,
    pub trivia: &'a str
}

// XXX: Working around the lambda function limitation about lifetimes
// https://github.com/rust-lang/rust/issues/58052
pub fn annotate<'slice, 'token, F, R>(f: F) -> F
where
    F: Fn(Tokens<'slice, 'token>) -> nom::IResult<Tokens<'slice, 'token>, R>,
    'token: 'slice,
{
    f
}

macro_rules! eat {
    ($variant:ident) => {
        crate::parser::eat::annotate(|input: Tokens| -> IResult<Tokens, _> {
            use nom::{InputIter, Slice};
            if let Some(Token { tag: crate::lexer::Tag::$variant(variant), trivia }) = input.iter_elements().next() {
                return Ok((input.slice(1..), crate::parser::eat::VariantToken { variant, trivia }))
            }
            Err(nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Char,
            }))
        })
    };
}

macro_rules! eatKey {
    ($variant:ident) => {
        crate::parser::eat::annotate(|input: Tokens| -> IResult<Tokens, _> {
            use nom::{InputIter, Slice};
            use crate::lexer::{Tag, keywords::Keyword};
            if let Some(Token { tag: Tag::Kw(Keyword::$variant(variant)), trivia }) = input.iter_elements().next() {
                return Ok((input.slice(1..), crate::parser::eat::VariantToken { variant, trivia }))
            }
            Err(nom::Err::Error(nom::error::Error {
                input,
                code: nom::error::ErrorKind::Char,
            }))
        })
    };
}

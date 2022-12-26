use super::impl_nom_traits::Tokens;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VariantToken<'a, T> {
    pub variant: T,
    pub trivia: &'a str,
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
            match input.iter_elements().next() {
                Some(crate::lexer::Token {
                    tag: crate::lexer::Tag::$variant(variant),
                    trivia,
                }) => Ok((
                    input.slice(1..),
                    crate::parser::eat::VariantToken { variant, trivia },
                )),
                _ => Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Char,
                })),
            }
        })
    };
}

macro_rules! eat_key {
    ($variant:ident) => {
        crate::parser::eat::annotate(|input: Tokens| -> IResult<Tokens, _> {
            use crate::lexer::{keywords::Keyword, Tag};
            use nom::{InputIter, Slice};
            match input.iter_elements().next() {
                Some(crate::lexer::Token {
                    tag: Tag::Kw(Keyword::$variant(variant)),
                    trivia,
                }) => Ok((
                    input.slice(1..),
                    crate::parser::eat::VariantToken { variant, trivia },
                )),
                _ => Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Char,
                })),
            }
        })
    };
}

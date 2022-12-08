use super::impl_nom_traits::Tokens;
use crate::lexer::Token;

// XXX: Working around the lambda function limitation about lifetimes
// https://github.com/rust-lang/rust/issues/58052
pub fn annotate<'slice, 'token, F>(f: F) -> F
where
    F: Fn(Tokens<'slice, 'token>) -> nom::IResult<Tokens<'slice, 'token>, Token<'token>>,
    'token: 'slice,
{
    f
}

macro_rules! eat {
    ($matcher:pat_param) => {
        crate::parser::eat::annotate(|input: Tokens| -> IResult<Tokens, Token> {
            use nom::{InputIter, Slice};
            match input
                .iter_elements()
                .next()
                .map(|t| (t, matches!(t.tag, $matcher)))
            {
                Some((t, true)) => Ok((input.slice(1..), t)),
                _ => Err(nom::Err::Error(nom::error::Error {
                    input,
                    code: nom::error::ErrorKind::Char,
                })),
            }
        })
    };
}

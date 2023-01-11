use super::impl_nom_traits::Tokens;

// XXX: Working around the lambda function limitation about lifetimes
// https://github.com/rust-lang/rust/issues/58052
pub fn annotate<'slice, 'token, F, R>(f: F) -> F
where
    F: Fn(Tokens<'slice, 'token>) -> nom::IResult<Tokens<'slice, 'token>, R>,
    'token: 'slice,
{
    f
}

#[macro_export]
macro_rules! eat {
    ($variant:ident) => {
        $crate::parser::eat::annotate(
            |input: $crate::parser::Tokens| -> nom::IResult<$crate::parser::Tokens, _> {
                use nom::{InputIter, Slice};
                match input.iter_elements().next() {
                    Some($crate::lexer::Token {
                        tag: $crate::lexer::Tag::$variant(variant),
                        trivia: _,
                    }) => Ok((input.slice(1..), variant)),
                    _ => Err(nom::Err::Error(nom::error::Error {
                        input,
                        code: nom::error::ErrorKind::Char,
                    })),
                }
            },
        )
    };
}

#[macro_export]
macro_rules! eat_key {
    ($variant:ident) => {
        $crate::parser::eat::annotate(
            |input: $crate::parser::Tokens| -> nom::IResult<$crate::parser::Tokens, _> {
                use nom::{InputIter, Slice};
                use $crate::lexer::Tag;
                use $crate::term::Keyword;
                match input.iter_elements().next() {
                    Some($crate::lexer::Token {
                        tag: Tag::Kw(Keyword::$variant(variant)),
                        trivia: _,
                    }) => Ok((input.slice(1..), variant)),
                    _ => Err(nom::Err::Error(nom::error::Error {
                        input,
                        code: nom::error::ErrorKind::Char,
                    })),
                }
            },
        )
    };
}

mod impl_nom_traits;

use std::ops::RangeFrom;

use nom::IResult;

use crate::lexer::{lex, Token, TokenTag};

pub enum ErrorKind<'a> {
    Lexer(nom::Err<nom::error::Error<&'a str>>),
    Parser(nom::Err<nom::error::Error<&'a [Token<'a>]>>),
}

pub fn identifier<'a, I>(input: I) -> IResult<I, Token<'a>>
where
    I: nom::Slice<RangeFrom<usize>> + nom::InputIter<Item = Token<'a>>,
{
    match input
        .iter_elements()
        .next()
        .map(|t| (t, matches!(t.tag, TokenTag::Identifier(_))))
    {
        Some((t, true)) => Ok((input.slice(1..), t)),
        _ => Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Char,
        })),
    }
}

// macro_rules! keyword {

// }

// pub fn includes_statement(tokens: &[Token]) {
// lhs_identifier: Identifier<'a>,
// includes: term!(implements),
// rhs_identifier: Identifier<'a>,
// semi_colon: term!(;),
// }

pub fn parse(input: &str) -> IResult<Vec<Token>, (), ErrorKind> {
    let tokens = lex(input).map_err(|err| nom::Err::Failure(ErrorKind::Lexer(err)))?;

    Ok((tokens, ()))
}

#[cfg(test)]
mod tests {
    use super::{impl_nom_traits::Tokens, *};

    #[test]
    fn test() {
        let (remaining, (id1, id2)) = nom::sequence::tuple((identifier, identifier))(Tokens(&[
            Token {
                tag: TokenTag::Identifier(crate::common::Identifier("foo")),
                trivia: "",
            },
            Token {
                tag: TokenTag::Identifier(crate::common::Identifier("bar")),
                trivia: "",
            },
        ]))
        .unwrap();

        assert!(remaining.0.is_empty());

        if let TokenTag::Identifier(crate::common::Identifier(id)) = id1.tag {
            assert_eq!(id, "foo", "id1 should be foo");
        } else {
            assert!(false, "id1 should be foo")
        }

        if let TokenTag::Identifier(crate::common::Identifier(id)) = id2.tag {
            assert_eq!(id, "bar", "id2 should be bar");
        } else {
            assert!(false, "id2 should be bar")
        }
    }
}

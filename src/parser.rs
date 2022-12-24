mod impl_nom_traits;
use impl_nom_traits::Tokens;

#[macro_use]
mod eat;
mod includes;
mod primitive_type;

use nom::{IResult, InputIter};

use crate::lexer::{lex, Token};

use self::{
    eat::VariantToken,
    includes::{includes_statement, IncludesStatement},
};

#[derive(Debug)]
pub enum ErrorKind<'a> {
    Lexer(nom::Err<nom::error::Error<&'a str>>),
    Parser(nom::Err<nom::error::Error<Vec<Token<'a>>>>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Definition<'a> {
    Includes(IncludesStatement<'a>),
    Eof(VariantToken<'a, ()>),
}

pub fn parse(input: &str) -> Result<Vec<Definition>, ErrorKind> {
    let tokens = lex(input).map_err(ErrorKind::Lexer)?;

    let (unread, (mut defs, eof)) = nom::sequence::tuple((
        nom::multi::many0(nom::branch::alt((nom::combinator::map(
            includes_statement,
            Definition::Includes,
        ),))),
        nom::combinator::map(eat!(Eof), Definition::Eof),
    ))(Tokens(&tokens[..]))
    .map_err(|err| match err {
        nom::Err::Incomplete(need) => ErrorKind::Parser(nom::Err::Incomplete(need)),
        nom::Err::Error(err) => ErrorKind::Parser(nom::Err::Error(nom::error::Error {
            code: err.code,
            input: err.input.iter_elements().collect(),
        })),
        nom::Err::Failure(err) => ErrorKind::Parser(nom::Err::Failure(nom::error::Error {
            code: err.code,
            input: err.input.iter_elements().collect(),
        })),
    })?;

    // Cannot be empty here since eof would fail then
    assert!(unread.0.is_empty());

    defs.push(eof);

    Ok(defs)
}

#[cfg(test)]
mod tests {
    use crate::lexer::Tag;

    use super::{impl_nom_traits::Tokens, *};

    #[test]
    fn test() {
        let (remaining, (id1, id2)) = nom::sequence::tuple((eat!(Id), eat!(Id)))(Tokens(&[
            Token {
                tag: Tag::Id(crate::common::Identifier("foo")),
                trivia: "",
            },
            Token {
                tag: Tag::Id(crate::common::Identifier("bar")),
                trivia: "",
            },
        ]))
        .unwrap();

        assert!(remaining.0.is_empty());
        assert_eq!(id1.variant.0, "foo", "id1 should be foo");
        assert_eq!(id2.variant.0, "bar", "id2 should be bar");
    }

    #[test]
    fn parse() {
        let result = super::parse("Foo includes Bar;").unwrap();

        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], Definition::Includes(_)));
        assert!(matches!(result[1], Definition::Eof(_)));
    }
}

mod impl_nom_traits;
use impl_nom_traits::Tokens;

#[macro_use]
mod eat;
mod includes;

use nom::{IResult, InputIter};

use crate::lexer::{lex, Tag, Token};

use self::includes::{includes_statement, IncludesStatement};

#[derive(Debug)]
pub enum ErrorKind<'a> {
    Lexer(nom::Err<nom::error::Error<&'a str>>),
    Parser(nom::Err<nom::error::Error<Vec<Token<'a>>>>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Definition<'a> {
    Includes(IncludesStatement<'a>),
    Eof(Token<'a>),
}

pub fn parse(input: &str) -> Result<Vec<Definition>, ErrorKind> {
    let tokens = lex(input).map_err(ErrorKind::Lexer)?;

    let (unread, (mut defs, eof)) = nom::sequence::tuple((
        nom::multi::many0(nom::branch::alt((nom::combinator::map(
            includes_statement,
            Definition::Includes,
        ),))),
        nom::combinator::map(eat!(Tag::Eof), Definition::Eof),
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
    use crate::lexer::{keywords::Keyword, lex};

    use super::{impl_nom_traits::Tokens, *};

    #[test]
    fn test() {
        let (remaining, (id1, id2)) =
            nom::sequence::tuple((eat!(Tag::Id(_)), eat!(Tag::Id(_))))(Tokens(&[
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

        if let Tag::Id(crate::common::Identifier(id)) = id1.tag {
            assert_eq!(id, "foo", "id1 should be foo");
        } else {
            assert!(false, "id1 should be foo")
        }

        if let Tag::Id(crate::common::Identifier(id)) = id2.tag {
            assert_eq!(id, "bar", "id2 should be bar");
        } else {
            assert!(false, "id2 should be bar")
        }
    }

    #[test]
    fn interface_mixin() {
        let tokens = lex("Foo includes Bar;").unwrap();
        let (unread, result) = includes_statement(Tokens(&tokens[..])).unwrap();

        assert!(matches!(unread.0[0].tag, Tag::Eof));
        assert!(matches!(result.target.tag, Tag::Id(_)));
        assert!(matches!(result.includes.tag, Tag::Kw(Keyword::Includes(_))));
        assert!(matches!(result.mixin.tag, Tag::Id(_)));
        assert!(matches!(
            result.termination.tag,
            Tag::Kw(Keyword::SemiColon(_))
        ));
    }

    #[test]
    fn parse() {
        let result = super::parse("Foo includes Bar;").unwrap();

        assert_eq!(result.len(), 2);
        assert!(matches!(result[0], Definition::Includes(_)));
        assert!(matches!(result[1], Definition::Eof(_)));
    }
}

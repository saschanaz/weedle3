mod impl_nom_traits;
use impl_nom_traits::Tokens;

use nom::{IResult, InputIter};

use crate::lexer::{keywords::Keyword, lex, Tag, Token};

#[derive(Debug)]
pub enum ErrorKind<'a> {
    Lexer(nom::Err<nom::error::Error<&'a str>>),
    Parser(nom::Err<nom::error::Error<Vec<Token<'a>>>>),
}

// XXX: Working around the lambda function limitation about lifetimes
// https://github.com/rust-lang/rust/issues/58052
fn annotate<'slice, 'token, F>(f: F) -> F
where
    F: Fn(Tokens<'slice, 'token>) -> IResult<Tokens<'slice, 'token>, Token<'token>>,
    'token: 'slice,
{
    f
}

macro_rules! eat {
    ($matcher:pat_param) => {
        annotate(|input: Tokens| -> IResult<Tokens, Token> {
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Definition<'a> {
    Includes(IncludesStatement<'a>),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IncludesStatement<'a> {
    pub target: Token<'a>,
    pub includes: Token<'a>,
    pub mixin: Token<'a>,
    pub termination: Token<'a>,
}

pub fn includes_statement<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IncludesStatement<'token>> {
    let (remaining, (target, includes, mixin, termination)) = nom::sequence::tuple((
        eat!(Tag::Id(_)),
        eat!(Tag::Kw(Keyword::Includes(_))),
        eat!(Tag::Id(_)),
        eat!(Tag::Kw(Keyword::SemiColon(_))),
    ))(tokens)?;

    Ok((
        remaining,
        IncludesStatement {
            target,
            includes,
            mixin,
            termination,
        },
    ))
}

pub fn parse(input: &str) -> Result<Vec<Definition>, ErrorKind> {
    let tokens = lex(input).map_err(ErrorKind::Lexer)?;

    // TODO: eat EOF
    let result = nom::multi::many0(nom::branch::alt((nom::combinator::map(
        includes_statement,
        Definition::Includes,
    ),)))(Tokens(&tokens[..]))
    .map(|(_, result)| result)
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
    });

    result
}

#[cfg(test)]
mod tests {
    use crate::lexer;

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
        let tokens = lexer::lex("Foo includes Bar;").unwrap();
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

        assert_eq!(result.len(), 1);
        assert!(matches!(result[0], Definition::Includes(_)));
    }
}

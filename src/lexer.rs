use nom::{multi::many0, IResult};

use crate::common::Identifier;
use crate::literal::{FloatLit, IntegerLit, StringLit};
use crate::whitespace::sp;
use crate::Parse;

pub mod keywords;
use keywords::Keyword;

mod nom_branch_alt;

pub type NomResult<'a, O> = IResult<&'a str, O>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Tag<'a> {
    Kw(Keyword<'a>),
    Int(IntegerLit<'a>),
    Dec(FloatLit<'a>),
    Id(Identifier<'a>),
    Str(StringLit<'a>),
    Other(char),
    Eof(()),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token<'a> {
    pub tag: Tag<'a>,
    // value: &'a str,
    pub trivia: &'a str,
    // TODO: Use https://github.com/fflorent/nom_locate/ ?
    // line: u32,

    // nom::traits::Offset should do the work
    // except this is a token index instead of the string offset,
    // but the purpose can be fulfilled:
    // 1. way to sort the tokens
    // 2. rewind (no need to do this manually with nom)
    //
    // index: u32,
}

impl Token<'_> {
    pub fn new<'a>((trivia, tag): (&'a str, Tag<'a>)) -> Token<'a> {
        Token { tag, trivia }
    }
}

// fn keyword(input: &str) -> NomResult<ch

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

fn token(input: &str) -> NomResult<Tag> {
    nom::branch::alt((
        nom::combinator::map(Keyword::parse, Tag::Kw),
        nom::combinator::map(IntegerLit::parse, Tag::Int),
        nom::combinator::map(FloatLit::parse, Tag::Dec),
        nom::combinator::map(Identifier::parse, Tag::Id),
        nom::combinator::map(StringLit::parse, Tag::Str),
        nom::combinator::map(other, Tag::Other),
    ))(input)
}

pub fn lex(input: &str) -> Result<Vec<Token>, nom::Err<nom::error::Error<&str>>> {
    // A little bit of hack with tuple since many0 is not compatible with eof
    // (It requires consuming at least one character)
    let (unread, (mut tokens, eof)) = nom::sequence::tuple((
        many0(nom::combinator::map(
            nom::sequence::tuple((sp, token)),
            Token::new,
        )),
        nom::combinator::map(
            nom::sequence::tuple((sp, nom::combinator::eof)),
            |(trivia, _)| Token {
                tag: Tag::Eof(()),
                trivia,
            },
        ),
    ))(input)?;

    // Cannot be empty here since eof would fail then
    assert!(unread.is_empty());

    tokens.push(eof);

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tokens = lex("interface mixin Foo {};").unwrap();
        assert_eq!(tokens.len(), 7);

        assert!(
            matches!(tokens[0].tag, Tag::Kw(Keyword::Interface(_))),
            "Should be Keyword::Interface"
        );
        assert!(
            matches!(tokens[1].tag, Tag::Kw(Keyword::Mixin(_))),
            "Should be Keyword::Mixin"
        );
        assert!(
            matches!(tokens[2].tag, Tag::Id(_)),
            "Should be an identifier"
        );
        assert!(
            matches!(tokens[3].tag, Tag::Kw(Keyword::OpenBrace(_))),
            "Should be Keyword::OpenBrace"
        );
        assert!(
            matches!(tokens[4].tag, Tag::Kw(Keyword::CloseBrace(_))),
            "Should be Keyword::CloseBrace"
        );
        assert!(
            matches!(tokens[5].tag, Tag::Kw(Keyword::SemiColon(_))),
            "Should be Keyword::SemiColon"
        );
        assert!(
            matches!(tokens[6].tag, Tag::Eof(_)),
            "Should be TokenTag::Eof"
        );
    }
}

use nom::{multi::many0, IResult};

use crate::common::Identifier;
use crate::literal::{FloatValueLit, IntegerLit, StringLit};
use crate::whitespace::sp;

#[macro_use]
pub mod keywords;
use keywords::Keyword;

pub type NomResult<'a, O> = IResult<&'a str, O>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Tag<'a> {
    Kw(Keyword<'a>),
    Int(IntegerLit<'a>),
    Dec(FloatValueLit<'a>),
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

    /// # Safety
    /// Make sure `input` is the source of this token.
    pub unsafe fn remaining<'a>(&self, input: &'a str) -> &'a str {
        let position: usize = self
            .trivia
            .as_ptr()
            .offset_from(input.as_ptr())
            .try_into()
            .expect("offset from input string pointer as usize");
        &input[position..]
    }
}

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

fn id_or_keyword(input: &str) -> NomResult<Tag> {
    let (input, id) = Identifier::parse(input)?;
    match Keyword::parse(id.0) {
        Ok(("", keyword)) => Ok((input, Tag::Kw(keyword))),
        _ => Ok((input, Tag::Id(id))),
    }
}

fn tag(input: &str) -> NomResult<Tag> {
    nom::branch::alt((
        nom::combinator::map(FloatValueLit::parse, Tag::Dec),
        nom::combinator::map(IntegerLit::parse, Tag::Int),
        nom::combinator::map(StringLit::parse, Tag::Str),
        id_or_keyword,
        nom::combinator::map(Keyword::parse, Tag::Kw),
        nom::combinator::map(other, Tag::Other),
    ))(input)
}

pub fn lex_single(input: &str) -> NomResult<Token> {
    nom::combinator::map(nom::sequence::tuple((sp, tag)), Token::new)(input)
}

pub fn lex(input: &str) -> Result<Vec<Token>, nom::Err<nom::error::Error<&str>>> {
    // A little bit of hack with tuple since many0 is not compatible with eof
    // (It requires consuming at least one character)
    let (unread, (mut tokens, eof)) = nom::sequence::tuple((
        many0(nom::combinator::map(
            nom::sequence::tuple((sp, tag)),
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
        println!("{tokens:?}");

        assert!(matches!(
            &tokens[..],
            [
                Token {
                    tag: Tag::Kw(Keyword::Interface(_)),
                    ..
                },
                Token {
                    tag: Tag::Kw(Keyword::Mixin(_)),
                    ..
                },
                Token {
                    tag: Tag::Id(_),
                    ..
                },
                Token {
                    tag: Tag::Kw(Keyword::OpenBrace(_)),
                    ..
                },
                Token {
                    tag: Tag::Kw(Keyword::CloseBrace(_)),
                    ..
                },
                Token {
                    tag: Tag::Kw(Keyword::SemiColon(_)),
                    ..
                },
                Token {
                    tag: Tag::Eof(_),
                    ..
                }
            ]
        ));
    }

    #[test]
    fn negative_infinity() {
        let tokens = lex("-Infinity").unwrap();
        println!("{tokens:?}");

        assert!(matches!(
            &tokens[..],
            [
                Token {
                    tag: Tag::Kw(Keyword::NegInfinity(_)),
                    ..
                },
                Token {
                    tag: Tag::Eof(_),
                    ..
                }
            ]
        ));
    }
}

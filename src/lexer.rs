use nom::{multi::many0, sequence::tuple, IResult, Parser};

use crate::common::Identifier;
use crate::literal::{FloatValueLit, IntegerLit, StringLit};
use crate::whitespace::sp;

use crate::term::Keyword;

pub type NomResult<'a, O> = IResult<&'a str, O>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Tag<'a> {
    Kw(Keyword),
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
    pub trivia: &'a str,
    // TODO: Use https://github.com/fflorent/nom_locate/ ?
}

impl Token<'_> {
    pub fn new<'a>((trivia, tag): (&'a str, Tag<'a>)) -> Token<'a> {
        Token { tag, trivia }
    }
}

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

fn id_or_keyword(input: &str) -> NomResult<Tag> {
    let (input, id) = Identifier::parse(input)?;
    match Keyword::match_word(id.0) {
        Some(keyword) => Ok((input, Tag::Kw(keyword))),
        _ => Ok((input, Tag::Id(id))),
    }
}

fn tag(input: &str) -> NomResult<Tag> {
    nom::branch::alt((
        FloatValueLit::parse.map(Tag::Dec),
        IntegerLit::parse.map(Tag::Int),
        StringLit::parse.map(Tag::Str),
        id_or_keyword,
        Keyword::parse_punc.map(Tag::Kw),
        other.map(Tag::Other),
    ))(input)
}

pub fn lex(input: &str) -> Result<Vec<Token>, nom::Err<nom::error::Error<&str>>> {
    let (unread, (mut tokens, eof)) = tuple((
        many0(tuple((sp, tag)).map(Token::new)),
        tuple((sp, nom::combinator::eof)).map(|(trivia, _)| Token {
            tag: Tag::Eof(()),
            trivia,
        }),
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
}

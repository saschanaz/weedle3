use nom::{multi::many0, IResult};

use crate::common::Identifier;
use crate::literal::{FloatLit, IntegerLit, StringLit};
use crate::whitespace::sp;
use crate::Parse;

mod keywords;
use keywords::Keyword;

pub type NomResult<'a, O> = IResult<&'a str, O>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenTag<'a> {
    Keyword(Keyword<'a>),
    Integer(IntegerLit<'a>),
    Decimal(FloatLit<'a>),
    Identifier(Identifier<'a>),
    String(StringLit<'a>),
    Other(char),
    Eof,
}

pub struct Token<'a> {
    tag: TokenTag<'a>,
    // value: &'a str,
    trivia: &'a str,
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

// fn keyword(input: &str) -> NomResult<ch

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

pub fn token(input: &str) -> NomResult<TokenTag> {
    nom::branch::alt((
        nom::combinator::map(Keyword::parse, TokenTag::Keyword),
        nom::combinator::map(IntegerLit::parse, TokenTag::Integer),
        nom::combinator::map(FloatLit::parse, TokenTag::Decimal),
        nom::combinator::map(Identifier::parse, TokenTag::Identifier),
        nom::combinator::map(StringLit::parse, TokenTag::String),
        nom::combinator::map(other, TokenTag::Other),
    ))(input)
}

pub fn tokens(input: &str) -> NomResult<Vec<Token>> {
    // A little bit of hack with tuple since many0 is not compatible with eof
    // (It requires consuming at least one character)
    let (unread, (mut tokens, (eof_trivia, _))) = nom::sequence::tuple((
        many0(nom::combinator::map(
            nom::sequence::tuple((sp, token)),
            |(trivia, tag)| Token { tag, trivia },
        )),
        nom::sequence::tuple((sp, nom::combinator::eof)),
    ))(input)?;

    tokens.push(Token {
        tag: TokenTag::Eof,
        trivia: eof_trivia,
    });

    Ok((unread, tokens))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (unread, tokens) = tokens("interface mixin Foo {};").unwrap();
        assert!(unread.is_empty());
        assert_eq!(tokens.len(), 7);

        // TODO: This should be a keyword instead
        match tokens[0].tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a keyword instead
        match tokens[1].tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a keyword instead
        match tokens[2].tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[3].tag {
            TokenTag::Keyword(Keyword::OpenBrace(_)) => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[4].tag {
            TokenTag::Keyword(Keyword::CloseBrace(_)) => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[5].tag {
            TokenTag::Keyword(Keyword::SemiColon(_)) => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        match tokens[6].tag {
            TokenTag::Eof => assert!(true, "Should be TokenTag::Eof"),
            _ => assert!(false, "Should be TokenTag::Eof"),
        }
    }
}

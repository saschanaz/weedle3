use nom::{multi::many0, IResult};

use crate::common::Identifier;
use crate::literal::{FloatLit, IntegerLit, StringLit};
use crate::whitespace::sp;
use crate::Parse;

pub type NomResult<'a, O> = IResult<&'a str, O>;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenTag<'a> {
    Integer(IntegerLit<'a>),
    Decimal(FloatLit<'a>),
    Identifier(Identifier<'a>),
    String(StringLit<'a>),
    Other,
    Eof,
}

pub struct Token<'a> {
    tag: TokenTag<'a>,
    //  value: &'a str,
    //  trivia: &'a str,

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

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

pub fn token(input: &str) -> NomResult<Token> {
    nom::branch::alt((
        nom::combinator::map(IntegerLit::parse, |result| Token {
            tag: TokenTag::Integer(result),
        }),
        nom::combinator::map(FloatLit::parse, |result| Token {
            tag: TokenTag::Decimal(result),
        }),
        nom::combinator::map(Identifier::parse, |result| Token {
            tag: TokenTag::Identifier(result),
        }),
        nom::combinator::map(StringLit::parse, |result| Token {
            tag: TokenTag::String(result),
        }),
        nom::combinator::map(other, |_| Token {
            tag: TokenTag::Other,
        }),
    ))(input)
}

pub fn tokens(input: &str) -> NomResult<Vec<(&str, Token)>> {
    // A little bit of hack with tuple since many0 is not compatible with eof
    // (It requires consuming at least one character)
    let (unread, (mut tokens, eof)) = nom::sequence::tuple((
        many0(nom::sequence::tuple((sp, token))),
        nom::combinator::eof,
    ))(input)?;

    // TODO: use _eof value for source text reconstruction
    tokens.push((eof, Token { tag: TokenTag::Eof }));

    Ok((unread, tokens))
}

// pub fn tokens(input: &str) -> NomResult<(&str, Token)> {
//     nom::sequence::tuple((sp, token))(input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let (unread, tokens) = tokens("interface mixin Foo {};").unwrap();
        assert!(unread.is_empty());
        assert_eq!(tokens.len(), 7);

        // TODO: This should be a keyword instead
        match tokens[0].1.tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a keyword instead
        match tokens[1].1.tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a keyword instead
        match tokens[2].1.tag {
            TokenTag::Identifier(_) => assert!(true, "Should be an identifier"),
            _ => assert!(false, "Should be an identifier"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[3].1.tag {
            TokenTag::Other => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[4].1.tag {
            TokenTag::Other => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        // TODO: This should be a recognized punctuation instead
        match tokens[5].1.tag {
            TokenTag::Other => assert!(true, "Should be TokenTag::Other"),
            _ => assert!(false, "Should be TokenTag::Other"),
        }

        match tokens[6].1.tag {
            TokenTag::Eof => assert!(true, "Should be TokenTag::Eof"),
            _ => assert!(false, "Should be TokenTag::Eof"),
        }
    }
}

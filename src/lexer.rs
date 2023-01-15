use nom::{multi::many0, sequence::tuple, Parser};

use crate::common::Identifier;
use crate::literal::{FloatValueLit, IntegerLit, StringLit};
use crate::term::Keyword;
use crate::whitespace::sp;

pub type NomResult<'a, O> = crate::VerboseResult<&'a str, O>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Terminal<'a> {
    Keyword(Keyword),
    Integer(IntegerLit<'a>),
    Decimal(FloatValueLit<'a>),
    Identifier(Identifier<'a>),
    String(StringLit<'a>),
    Other(char),
    Eof(()),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token<'a> {
    pub value: Terminal<'a>,
    pub trivia: &'a str,
    // TODO: Use https://github.com/fflorent/nom_locate/ ?
}

impl Token<'_> {
    pub fn new<'a>((trivia, tag): (&'a str, Terminal<'a>)) -> Token<'a> {
        Token { value: tag, trivia }
    }
}

fn other(input: &str) -> NomResult<char> {
    nom::character::complete::satisfy(|c| !"\t\n\r ".contains(c) && !c.is_alphanumeric())(input)
}

fn id_or_keyword(input: &str) -> NomResult<Terminal> {
    let (input, id) = Identifier::lex(input)?;
    match Keyword::match_word(id.0) {
        Some(keyword) => Ok((input, Terminal::Keyword(keyword))),
        _ => Ok((input, Terminal::Identifier(id))),
    }
}

fn tag(input: &str) -> NomResult<Terminal> {
    nom::branch::alt((
        FloatValueLit::lex.map(Terminal::Decimal),
        IntegerLit::lex.map(Terminal::Integer),
        StringLit::lex.map(Terminal::String),
        id_or_keyword,
        Keyword::parse_punc.map(Terminal::Keyword),
        other.map(Terminal::Other),
    ))(input)
}

pub fn lex(input: &str) -> Result<Vec<Token>, nom::Err<nom::error::VerboseError<&str>>> {
    let (unread, (mut tokens, eof)) = tuple((
        many0(tuple((sp, tag)).map(Token::new)),
        tuple((sp, nom::combinator::eof)).map(|(trivia, _)| Token {
            value: Terminal::Eof(()),
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
    use test_generator::test_resources;

    #[test]
    fn test() {
        let tokens = lex("interface mixin Foo {};").unwrap();
        println!("{tokens:?}");

        assert!(matches!(
            &tokens[..],
            [
                Token {
                    value: Terminal::Keyword(Keyword::Interface(_)),
                    ..
                },
                Token {
                    value: Terminal::Keyword(Keyword::Mixin(_)),
                    ..
                },
                Token {
                    value: Terminal::Identifier(_),
                    ..
                },
                Token {
                    value: Terminal::Keyword(Keyword::OpenBrace(_)),
                    ..
                },
                Token {
                    value: Terminal::Keyword(Keyword::CloseBrace(_)),
                    ..
                },
                Token {
                    value: Terminal::Keyword(Keyword::SemiColon(_)),
                    ..
                },
                Token {
                    value: Terminal::Eof(_),
                    ..
                }
            ]
        ));
    }

    #[test_resources("tests/defs/*.webidl")]
    fn should_lex(resource: &str) {
        let content = std::fs::read_to_string(resource).unwrap();
        let tokens = lex(&content).unwrap();

        assert!(
            matches!(tokens.last().unwrap().value, Terminal::Eof(_)),
            "Last token should be EOF"
        );

        // TODO: source file reconstruction test
    }
}

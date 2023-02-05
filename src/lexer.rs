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
pub struct Lexed<'a> {
    pub trivia: &'a str,
    pub value: Terminal<'a>,
    // TODO: Use https://github.com/fflorent/nom_locate/ ?
}

impl Lexed<'_> {
    pub fn new<'a>((trivia, value): (&'a str, Terminal<'a>)) -> Lexed<'a> {
        Lexed { trivia, value }
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

pub fn lex(input: &str) -> Result<Vec<Lexed>, nom::Err<nom::error::VerboseError<&str>>> {
    let (unread, (mut tokens, eof)) = tuple((
        many0(tuple((sp, tag)).map(Lexed::new)),
        tuple((sp, nom::combinator::eof)).map(|(trivia, _)| Lexed {
            trivia,
            value: Terminal::Eof(()),
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
                Lexed {
                    value: Terminal::Keyword(Keyword::Interface(_)),
                    ..
                },
                Lexed {
                    value: Terminal::Keyword(Keyword::Mixin(_)),
                    ..
                },
                Lexed {
                    value: Terminal::Identifier(_),
                    ..
                },
                Lexed {
                    value: Terminal::Keyword(Keyword::OpenBrace(_)),
                    ..
                },
                Lexed {
                    value: Terminal::Keyword(Keyword::CloseBrace(_)),
                    ..
                },
                Lexed {
                    value: Terminal::Keyword(Keyword::SemiColon(_)),
                    ..
                },
                Lexed {
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

use std::marker::PhantomData;

use weedle_derive::Weedle;

use crate::lexer::keywords;
use crate::literal::DefaultValue;
use crate::parser::Tokens;
use crate::{term, Parse};

pub(crate) fn is_alphanum_underscore_dash(token: char) -> bool {
    nom::AsChar::is_alphanum(token) || matches!(token, '_' | '-')
}

impl<'slice, 'a, T: Parse<'slice, 'a>> Parse<'slice, 'a> for Option<T> {
    parser!(nom::combinator::opt(weedle!(T)));
}

impl<'slice, 'a, T: Parse<'slice, 'a>> Parse<'slice, 'a> for Box<T> {
    parser!(nom::combinator::map(weedle!(T), Box::new));
}

/// Parses `item1 item2 item3...`
impl<'slice, 'a, T: Parse<'slice, 'a>> Parse<'slice, 'a> for Vec<T> {
    parser!(nom::multi::many0(T::parse));
}

impl<'slice, 'a, T: Parse<'slice, 'a>, U: Parse<'slice, 'a>> Parse<'slice, 'a> for (T, U) {
    parser!(nom::sequence::tuple((T::parse, U::parse)));
}

impl<'slice, 'a, T: Parse<'slice, 'a>, U: Parse<'slice, 'a>, V: Parse<'slice, 'a>> Parse<'slice, 'a>
    for (T, U, V)
{
    parser!(nom::sequence::tuple((T::parse, U::parse, V::parse)));
}

/// Parses `( body )`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'slice, 'a>")]
pub struct Parenthesized<T> {
    pub open_paren: keywords::OpenParen,
    pub body: T,
    pub close_paren: keywords::CloseParen,
}

/// Parses `[ body ]`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'slice, 'a>")]
pub struct Bracketed<T> {
    pub open_bracket: keywords::OpenBracket,
    pub body: T,
    pub close_bracket: keywords::CloseBracket,
}

/// Parses `{ body }`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'slice, 'a>")]
pub struct Braced<T> {
    pub open_brace: keywords::OpenBrace,
    pub body: T,
    pub close_brace: keywords::CloseBrace,
}

/// Parses `< body >`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'slice, 'a>")]
pub struct Generics<T> {
    pub open_angle: keywords::LessThan,
    pub body: T,
    pub close_angle: keywords::GreaterThan,
}

/// Parses `(item1, item2, item3,...)?`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<'slice, 'a, T, S> Parse<'slice, 'a> for Punctuated<T, S>
where
    T: Parse<'slice, 'a>,
    S: Parse<'slice, 'a> + core::default::Default,
{
    fn parse(input: Tokens<'slice, 'a>) -> crate::IResult<Tokens<'slice, 'a>, Self> {
        let (input, list) = nom::multi::separated_list0(weedle!(S), weedle!(T))(input)?;
        Ok((
            input,
            Self {
                list,
                separator: S::default(),
            },
        ))
    }
}

/// Parses `item1, item2, item3, ...`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PunctuatedNonEmpty<T, S> {
    pub list: Vec<T>,
    pub separator: std::marker::PhantomData<S>,
}

impl<'slice, 'a, T, S> Parse<'slice, 'a> for PunctuatedNonEmpty<T, S>
where
    T: Parse<'slice, 'a>,
    S: Parse<'slice, 'a>,
{
    fn parse(input: Tokens<'slice, 'a>) -> crate::IResult<Tokens<'slice, 'a>, Self> {
        let (input, list) = nom::sequence::terminated(
            nom::multi::separated_list1(weedle!(S), weedle!(T)),
            nom::combinator::opt(weedle!(S)),
        )(input)?;
        Ok((
            input,
            Self {
                list,
                separator: PhantomData::default(),
            },
        ))
    }
}

/// Represents an identifier
///
/// Follows `/[_-]?[A-Za-z][0-9A-Z_a-z-]*/`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier<'a>(pub &'a str);

impl<'a> Identifier<'a> {
    pub fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
        nom::combinator::map(
            nom::combinator::recognize(nom::sequence::tuple((
                nom::combinator::opt(nom::branch::alt((
                    nom::character::complete::char('_'),
                    nom::character::complete::char('-'),
                ))),
                nom::bytes::complete::take_while1(nom::AsChar::is_alpha),
                nom::bytes::complete::take_while(is_alphanum_underscore_dash),
            ))),
            Identifier,
        )(input)
    }
}

impl<'slice, 'a> Parse<'slice, 'a> for Identifier<'a> {
    parser!(crate::eat!(Id));
}

/// Parses rhs of an assignment expression. Ex: `= 45`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Default<'a> {
    pub assign: term!(=),
    pub value: DefaultValue<'a>,
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_optional_present { "one" =>
        "";
        Option<Identifier>;
        is_some();
    });

    test!(should_parse_optional_not_present { "" =>
        "";
        Option<Identifier>;
        is_none();
    });

    test!(should_parse_boxed { "one" =>
        "";
        Box<Identifier>;
    });

    test!(should_parse_vec { "one two three" =>
        "";
        Vec<Identifier>;
        len() == 3;
    });

    test!(should_parse_parenthesized { "( one )" =>
        "";
        Parenthesized<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_bracketed { "[ one ]" =>
        "";
        Bracketed<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_braced { "{ one }" =>
        "";
        Braced<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_generics { "<one>" =>
        "";
        Generics<Identifier>;
        body.0 == "one";
    });

    test!(should_parse_generics_two { "<one, two>" =>
        "";
        Generics<(Identifier, keywords::Comma, Identifier)> =>
            Generics {
                open_angle: keywords::LessThan,
                body: (
                    Identifier("one"),
                    keywords::Comma,
                    Identifier("two"),
                ),
                close_angle: keywords::GreaterThan,
            }
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<Identifier, keywords::Comma>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<Identifier, keywords::Comma>
    });

    test_match!(should_parse_identifier { "hello" =>
        "";
        Identifier => Identifier("hello")
    });

    test_match!(should_parse_numbered_identifier { "hello5" =>
        "";
        Identifier => Identifier("hello5")
    });

    test_match!(should_parse_underscored_identifier { "_hello_" =>
        "";
        Identifier => Identifier("_hello_")
    });

    test_match!(should_parse_hyphened_identifier { "-hello" =>
        "";
        Identifier => Identifier("-hello")
    });

    test_result_match!(should_not_parse_identifier_surrounding_with_spaces { "  hello  ";
        Identifier => Err(nom::Err::Error(_))
    });

    test_match!(should_parse_identifier_preceding_others { "hello  note" =>
        "  note";
        Identifier => Identifier("hello")
    });

    test_match!(should_parse_identifier_attached_to_symbol { "hello=" =>
        "=";
        Identifier => Identifier("hello")
    });
}

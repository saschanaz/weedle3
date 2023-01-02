use std::marker::PhantomData;

use weedle_derive::Weedle;

use crate::lexer::keywords;
use crate::literal::DefaultValue;
use crate::parser::eat::VariantToken;
use crate::{term, Parse};

pub(crate) fn is_alphanum_underscore_dash(token: char) -> bool {
    nom::AsChar::is_alphanum(token) || matches!(token, '_' | '-')
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    parser!(nom::combinator::opt(weedle!(T)));
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    parser!(nom::combinator::map(weedle!(T), Box::new));
}

/// Parses `item1 item2 item3...`
impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    parser!(nom::multi::many0(T::parse));
}

impl<'a, T: Parse<'a>, U: Parse<'a>> Parse<'a> for (T, U) {
    parser!(nom::sequence::tuple((T::parse, U::parse)));
}

impl<'a, T: Parse<'a>, U: Parse<'a>, V: Parse<'a>> Parse<'a> for (T, U, V) {
    parser!(nom::sequence::tuple((T::parse, U::parse, V::parse)));
}

/// Parses `( body )`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Parenthesized<'a, T> {
    pub open_paren: VariantToken<'a, keywords::OpenParen<'a>>,
    pub body: T,
    pub close_paren: VariantToken<'a, keywords::CloseParen<'a>>,
}

/// Parses `[ body ]`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Bracketed<'a, T> {
    pub open_bracket: VariantToken<'a, keywords::OpenBracket<'a>>,
    pub body: T,
    pub close_bracket: VariantToken<'a, keywords::CloseBracket<'a>>,
}

/// Parses `{ body }`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Braced<'a, T> {
    pub open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    pub body: T,
    pub close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
}

/// Parses `< body >`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Generics<'a, T> {
    pub open_angle: VariantToken<'a, keywords::LessThan<'a>>,
    pub body: T,
    pub close_angle: VariantToken<'a, keywords::GreaterThan<'a>>,
}

/// Parses `(item1, item2, item3,...)?`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: std::marker::PhantomData<S>,
}

impl<'a, T, S> Parse<'a> for Punctuated<T, S>
where
    T: Parse<'a>,
    S: Parse<'a>,
{
    fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
        let (input, list) = nom::multi::separated_list0(weedle!(S), weedle!(T))(input)?;
        Ok((
            input,
            Self {
                list,
                separator: PhantomData::default(),
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

impl<'a, T, S> Parse<'a> for PunctuatedNonEmpty<T, S>
where
    T: Parse<'a>,
    S: Parse<'a>,
{
    fn parse(input: &'a str) -> crate::IResult<&'a str, Self> {
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
            crate::whitespace::ws(nom::combinator::recognize(nom::sequence::tuple((
                nom::combinator::opt(nom::branch::alt((
                    nom::character::complete::char('_'),
                    nom::character::complete::char('-'),
                ))),
                nom::bytes::complete::take_while1(nom::AsChar::is_alpha),
                nom::bytes::complete::take_while(is_alphanum_underscore_dash),
            )))),
            Identifier,
        )(input)
    }
}

impl<'a> Parse<'a> for VariantToken<'a, Identifier<'a>> {
    parser!(|input: &'a str| {
        use crate::parser::Tokens;
        use nom::IResult;

        let (i, token) = crate::lexer::lex_single(input)?;
        let array = [token];
        let tokens = Tokens(&array);
        match crate::eat!(Id)(tokens) {
            Ok((_, token)) => Ok((i, token)),
            Err(_) => Err(nom::Err::Error(nom::error::Error {
                input: i,
                code: nom::error::ErrorKind::Char,
            })),
        }
    });
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
        Option<VariantToken<Identifier>>;
        is_some();
    });

    test!(should_parse_optional_not_present { "" =>
        "";
        Option<VariantToken<Identifier>>;
        is_none();
    });

    test!(should_parse_boxed { "one" =>
        "";
        Box<VariantToken<Identifier>>;
    });

    test!(should_parse_vec { "one two three" =>
        "";
        Vec<VariantToken<Identifier>>;
        len() == 3;
    });

    test!(should_parse_parenthesized { "( one )" =>
        "";
        Parenthesized<VariantToken<Identifier>>;
        body.variant.0 == "one";
    });

    test!(should_parse_bracketed { "[ one ]" =>
        "";
        Bracketed<VariantToken<Identifier>>;
        body.variant.0 == "one";
    });

    test!(should_parse_braced { "{ one }" =>
        "";
        Braced<VariantToken<Identifier>>;
        body.variant.0 == "one";
    });

    test!(should_parse_generics { "<one>" =>
        "";
        Generics<VariantToken<Identifier>>;
        body.variant.0 == "one";
    });

    test!(should_parse_generics_two { "<one, two>" =>
        "";
        Generics<(VariantToken<Identifier>, VariantToken<keywords::Comma>, VariantToken<Identifier>)> =>
            Generics {
                open_angle: VariantToken::default(),
                body: (
                    VariantToken { variant: Identifier("one"), trivia: "" },
                    VariantToken::default(),
                    VariantToken { variant: Identifier("two"), trivia: " " },
                ),
                close_angle: VariantToken::default(),
            }
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<VariantToken<Identifier>, VariantToken<keywords::Comma>>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<VariantToken<Identifier>, VariantToken<keywords::Comma>>
    });

    test!(should_parse_identifier { "hello" =>
        "";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_numbered_identifier { "hello5" =>
        "";
        Identifier;
        0 == "hello5";
    });

    test!(should_parse_underscored_identifier { "_hello_" =>
        "";
        Identifier;
        0 == "_hello_";
    });

    test!(should_parse_hyphened_identifier { "-hello" =>
        "";
        Identifier;
        0 == "-hello";
    });

    test!(should_parse_identifier_surrounding_with_spaces { "  hello  " =>
        "";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_identifier_preceding_others { "hello  note" =>
        "note";
        Identifier;
        0 == "hello";
    });

    test!(should_parse_identifier_attached_to_symbol { "hello=" =>
        "=";
        Identifier;
        0 == "hello";
    });
}

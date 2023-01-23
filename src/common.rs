use weedle_derive::Weedle;

use crate::literal::DefaultValue;
use crate::tokens::{contextful_cut, Tokens};
use crate::{term, Parse, VerboseResult};

pub(crate) fn is_alphanum_underscore_dash(token: char) -> bool {
    nom::AsChar::is_alphanum(token) || matches!(token, '_' | '-')
}

fn marker<'slice, 'a, S>(i: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, S>
where
    S: ::std::default::Default,
{
    Ok((i, S::default()))
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    parser!(nom::combinator::opt(weedle!(T)));
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    parser!(nom::combinator::map(weedle!(T), Box::new));
}

/// Parses `item1 item2 item3...`
impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    parser!(nom::multi::many0(T::parse_tokens));
}

impl<'a, T: Parse<'a>, U: Parse<'a>> Parse<'a> for (T, U) {
    parser!(nom::sequence::tuple((T::parse_tokens, U::parse_tokens)));
}

impl<'a, T: Parse<'a>, U: Parse<'a>, V: Parse<'a>> Parse<'a> for (T, U, V) {
    parser!(nom::sequence::tuple((
        T::parse_tokens,
        U::parse_tokens,
        V::parse_tokens
    )));
}

/// Parses `( body )`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Parenthesized<T> {
    pub open_paren: term::OpenParen,
    #[weedle(cut = "Unrecognized argument")]
    pub body: T,
    #[weedle(cut = "Unrecognized argument")]
    pub close_paren: term::CloseParen,
}

/// Parses `( body )`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub(crate) struct ParenthesizedNonEmpty<T> {
    #[weedle(post_check = "prevent_empty_parentheses")]
    pub open_paren: term::OpenParen,
    pub body: T,
    pub close_paren: term::CloseParen,
}

impl<T> From<ParenthesizedNonEmpty<T>> for Parenthesized<T> {
    fn from(value: ParenthesizedNonEmpty<T>) -> Self {
        let ParenthesizedNonEmpty {
            open_paren,
            body,
            close_paren,
        } = value;
        Self {
            open_paren,
            body,
            close_paren,
        }
    }
}

fn prevent_empty_parentheses<'slice, 'a>(
    input: Tokens<'slice, 'a>,
) -> VerboseResult<Tokens<'slice, 'a>, ()> {
    contextful_cut(
        "Unexpected empty parentheses",
        nom::combinator::not(nom::combinator::peek(eat_key!(CloseParen))),
    )(input)
}

/// Parses `[ body ]`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Bracketed<T> {
    #[weedle(post_check = "prevent_empty_brackets")]
    pub open_bracket: term::OpenBracket,
    pub body: T,
    #[weedle(post_check = "prevent_double_extended_attributes")]
    pub close_bracket: term::CloseBracket,
}

fn prevent_empty_brackets<'slice, 'a>(
    input: Tokens<'slice, 'a>,
) -> VerboseResult<Tokens<'slice, 'a>, ()> {
    contextful_cut(
        "Unexpected empty brackets",
        nom::combinator::not(nom::combinator::peek(eat_key!(CloseBracket))),
    )(input)
}

fn prevent_double_extended_attributes<'slice, 'a>(
    input: Tokens<'slice, 'a>,
) -> VerboseResult<Tokens<'slice, 'a>, ()> {
    contextful_cut(
        "Illegal double extended attribute lists, consider merging them",
        nom::combinator::not(nom::combinator::peek(eat_key!(OpenBracket))),
    )(input)
}

/// Parses `{ body }`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Braced<T> {
    #[weedle(cut = "Missing body")]
    pub open_brace: term::OpenBrace,
    #[weedle(cut = "Unrecognized member definition")]
    pub body: T,
    #[weedle(cut = "Unrecognized member definition")]
    pub close_brace: term::CloseBrace,
}

/// Parses `< body >`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Generics<T> {
    pub open_angle: term::LessThan,
    #[weedle(cut = "Unrecognized type parameter")]
    pub body: T,
    pub close_angle: term::GreaterThan,
}

/// Parses `(item1, item2, item3,...)?`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<'a, T, S> Parse<'a> for Punctuated<T, S>
where
    T: Parse<'a>,
    S: Parse<'a> + ::std::default::Default,
{
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        let (input, (list, separator)) = nom::sequence::tuple((
            nom::multi::separated_list0(weedle!(S), weedle!(T)),
            marker,
        ))(input)?;
        Ok((input, Self { list, separator }))
    }
}

/// Parses `item1, item2, item3, ...`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PunctuatedNonEmpty<T, S> {
    pub list: Vec<T>,
    pub separator: S,
}

impl<'a, T, S> Parse<'a> for PunctuatedNonEmpty<T, S>
where
    T: Parse<'a>,
    S: Parse<'a> + ::std::default::Default,
{
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        let (input, (list, separator)) = nom::sequence::tuple((
            nom::sequence::terminated(
                nom::multi::separated_list1(weedle!(S), weedle!(T)),
                nom::combinator::opt(weedle!(S)),
            ),
            marker,
        ))(input)?;
        Ok((input, Self { list, separator }))
    }
}

/// Represents an identifier
///
/// Follows `/[_-]?[A-Za-z][0-9A-Z_a-z-]*/`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Identifier<'a>(pub &'a str);

impl<'a> Identifier<'a> {
    lexer!(nom::combinator::map(
        nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(nom::branch::alt((
                nom::character::complete::char('_'),
                nom::character::complete::char('-'),
            ))),
            nom::bytes::complete::take_while1(nom::AsChar::is_alpha),
            nom::bytes::complete::take_while(is_alphanum_underscore_dash),
        ))),
        Identifier,
    ));
}

impl<'a> Parse<'a> for Identifier<'a> {
    parser!(eat!(Identifier));
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
        Generics<(Identifier, term!(,), Identifier)> =>
            Generics {
                open_angle: term!(<),
                body: (Identifier("one"), term!(,), Identifier("two")),
                close_angle: term!(>),
            }
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<Identifier, term!(,)>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<Identifier, term!(,)>
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

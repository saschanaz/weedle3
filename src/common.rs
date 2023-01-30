use std::marker::PhantomData;

use weedle_derive::Weedle;

use crate::literal::DefaultValue;
use crate::parser::eat::VariantToken;
use crate::tokens::{contextful_cut, Tokens};
use crate::{term, Parse, VerboseResult};

pub(crate) fn is_alphanum_underscore_dash(token: char) -> bool {
    nom::AsChar::is_alphanum(token) || matches!(token, '_' | '-')
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    parser!(nom::combinator::opt(weedle!(T)));

    fn write(&self) -> String {
        match self {
            Some(x) => x.write(),
            None => "".to_owned(),
        }
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    parser!(nom::combinator::map(weedle!(T), Box::new));

    fn write(&self) -> String {
        self.as_ref().write()
    }
}

/// Parses `item1 item2 item3...`
impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    parser!(nom::multi::many0(T::parse_tokens));

    fn write(&self) -> String {
        self.iter().map(|item| item.write()).collect::<Vec<_>>().join("")
    }
}

impl<'a, T: Parse<'a>, U: Parse<'a>> Parse<'a> for (T, U) {
    parser!(nom::sequence::tuple((T::parse_tokens, U::parse_tokens)));

    fn write(&self) -> String {
        vec![self.0.write(), self.1.write()].join("")
    }
}

impl<'a, T: Parse<'a>, U: Parse<'a>, V: Parse<'a>> Parse<'a> for (T, U, V) {
    parser!(nom::sequence::tuple((
        T::parse_tokens,
        U::parse_tokens,
        V::parse_tokens
    )));

    fn write(&self) -> String {
        vec![self.0.write(), self.1.write(), self.2.write()].join("")
    }
}

/// Parses `( body )`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Parenthesized<'a, T> {
    pub open_paren: VariantToken<'a, term::OpenParen>,
    #[weedle(cut = "Unrecognized argument")]
    pub body: T,
    #[weedle(cut = "Unrecognized argument")]
    pub close_paren: VariantToken<'a, term::CloseParen>,
}

/// Parses `( body )`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub(crate) struct ParenthesizedNonEmpty<'a, T> {
    #[weedle(post_check = "prevent_empty_parentheses")]
    pub open_paren: VariantToken<'a, term::OpenParen>,
    pub body: T,
    pub close_paren: VariantToken<'a, term::CloseParen>,
}

impl<'a, T> From<ParenthesizedNonEmpty<'a, T>> for Parenthesized<'a, T> {
    fn from(value: ParenthesizedNonEmpty<'a, T>) -> Self {
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
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Bracketed<'a, T> {
    #[weedle(post_check = "prevent_empty_brackets")]
    pub open_bracket: VariantToken<'a, term::OpenBracket>,
    pub body: T,
    #[weedle(
        cut = "Unrecognized extended attribute",
        post_check = "prevent_double_extended_attributes"
    )]
    pub close_bracket: VariantToken<'a, term::CloseBracket>,
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
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Braced<'a, T> {
    #[weedle(cut = "Missing body")]
    pub open_brace: VariantToken<'a, term::OpenBrace>,
    #[weedle(cut = "Unrecognized member definition")]
    pub body: T,
    #[weedle(cut = "Unrecognized member definition")]
    pub close_brace: VariantToken<'a, term::CloseBrace>,
}

/// Parses `< body >`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Generics<'a, T> {
    pub open_angle: VariantToken<'a, term::LessThan>,
    #[weedle(cut = "Unrecognized type parameter")]
    pub body: T,
    pub close_angle: VariantToken<'a, term::GreaterThan>,
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
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        let (input, list) = nom::multi::separated_list0(weedle!(S), weedle!(T))(input)?;
        Ok((
            input,
            Self {
                list,
                separator: PhantomData::default(),
            },
        ))
    }

    fn write(&self) -> String {
        // XXX: Each item needs its own separator
        self.list.write()
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
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
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

    fn write(&self) -> String {
        // XXX: Each item needs its own separator
        self.list.write()
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

impl<'a> Parse<'a> for VariantToken<'a, Identifier<'a>> {
    parser!(eat!(Identifier));

    fn write(&self) -> String {
        let trivia = self.trivia;
        let variant = self.variant.0;
        format!("{trivia}{variant}")
    }
}

/// Parses rhs of an assignment expression. Ex: `= 45`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Default<'a> {
    pub assign: term!(=),
    #[weedle(cut = "Unrecognized default value")]
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
        Generics<(VariantToken<Identifier>, VariantToken<term::Comma>, VariantToken<Identifier>)> =>
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
        Punctuated<VariantToken<Identifier>, VariantToken<term::Comma>>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<VariantToken<Identifier>, VariantToken<term::Comma>>
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

    test!(should_parse_identifier_surrounding_with_spaces { "  hello  " =>
        "";
        VariantToken<Identifier>;
        variant.0 == "hello";
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

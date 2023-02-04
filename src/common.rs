use weedle_derive::Weedle;

use crate::literal::DefaultValue;
use crate::term::Token;
use crate::tokens::{contextful_cut, separated_list_incl, LexedSlice};
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
        self.iter()
            .map(|item| item.write())
            .collect::<Vec<_>>()
            .join("")
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
    pub open_paren: Token<'a, term::OpenParen>,
    #[weedle(cut = "Unrecognized argument")]
    pub body: T,
    #[weedle(cut = "Unrecognized argument")]
    pub close_paren: Token<'a, term::CloseParen>,
}

/// Parses `( body )`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub(crate) struct ParenthesizedNonEmpty<'a, T> {
    #[weedle(post_check = "prevent_empty_parentheses")]
    pub open_paren: Token<'a, term::OpenParen>,
    pub body: T,
    pub close_paren: Token<'a, term::CloseParen>,
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
    input: LexedSlice<'slice, 'a>,
) -> VerboseResult<LexedSlice<'slice, 'a>, ()> {
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
    pub open_bracket: Token<'a, term::OpenBracket>,
    pub body: T,
    #[weedle(
        cut = "Unrecognized extended attribute",
        post_check = "prevent_double_extended_attributes"
    )]
    pub close_bracket: Token<'a, term::CloseBracket>,
}

fn prevent_empty_brackets<'slice, 'a>(
    input: LexedSlice<'slice, 'a>,
) -> VerboseResult<LexedSlice<'slice, 'a>, ()> {
    contextful_cut(
        "Unexpected empty brackets",
        nom::combinator::not(nom::combinator::peek(eat_key!(CloseBracket))),
    )(input)
}

fn prevent_double_extended_attributes<'slice, 'a>(
    input: LexedSlice<'slice, 'a>,
) -> VerboseResult<LexedSlice<'slice, 'a>, ()> {
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
    pub open_brace: Token<'a, term::OpenBrace>,
    #[weedle(cut = "Unrecognized member definition")]
    pub body: T,
    #[weedle(cut = "Unrecognized member definition")]
    pub close_brace: Token<'a, term::CloseBrace>,
}

/// Parses `< body >`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct Generics<'a, T> {
    pub open_angle: Token<'a, term::LessThan>,
    #[weedle(cut = "Unrecognized type parameter")]
    pub body: T,
    pub close_angle: Token<'a, term::GreaterThan>,
}

/// Parses `(item1, item2, item3,...)?`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Punctuated<T, S> {
    pub list: Vec<T>,
    pub separator: Vec<S>,
}

impl<'a, T, S> Parse<'a> for Punctuated<T, S>
where
    T: Parse<'a>,
    S: Parse<'a>,
{
    fn parse_tokens<'slice>(
        input: LexedSlice<'slice, 'a>,
    ) -> VerboseResult<LexedSlice<'slice, 'a>, Self> {
        // TODO: replace separated_list0 (and _list1 below)
        let (input, (list, separator)) =
            separated_list_incl::<false, _, _, _, _, _, _>(weedle!(S), weedle!(T))(input)?;
        Ok((input, Self { list, separator }))
    }

    fn write(&self) -> String {
        // XXX: Each item needs its own separator
        // self.list.write()

        let vec: Vec<_> = self
            .list
            .iter()
            .zip(self.separator.iter())
            .map(|(item, sep)| {
                let item = item.write();
                let sep = sep.write();
                format!("{item}{sep}")
            })
            .collect();
        let mut result = vec.join("");
        if self.list.len() > self.separator.len() {
            result += &self.list.last().unwrap().write();
        }
        result
    }
}

/// Parses `item1, item2, item3, ...`
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PunctuatedNonEmpty<T, S> {
    pub list: Vec<T>,
    pub separator: Vec<S>,
}

impl<'a, T, S> Parse<'a> for PunctuatedNonEmpty<T, S>
where
    T: Parse<'a>,
    S: Parse<'a>,
{
    fn parse_tokens<'slice>(
        input: LexedSlice<'slice, 'a>,
    ) -> VerboseResult<LexedSlice<'slice, 'a>, Self> {
        let (input, ((list, mut separator), trailing)) = nom::sequence::tuple((
            separated_list_incl::<true, _, _, _, _, _, _>(weedle!(S), weedle!(T)),
            nom::combinator::opt(weedle!(S)),
        ))(input)?;
        if let Some(trailing) = trailing {
            separator.push(trailing)
        }
        Ok((input, Self { list, separator }))
    }

    fn write(&self) -> String {
        let vec: Vec<_> = self
            .list
            .iter()
            .zip(self.separator.iter())
            .map(|(item, sep)| {
                let item = item.write();
                let sep = sep.write();
                format!("{item}{sep}")
            })
            .collect();
        let mut result = vec.join("");
        if self.list.len() > self.separator.len() {
            result += &self.list.last().unwrap().write();
        }
        result
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

impl<'a> Parse<'a> for Token<'a, Identifier<'a>> {
    parser!(eat!(Identifier));

    fn write(&self) -> String {
        let trivia = self.trivia;
        let variant = self.value.0;
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

impl<'a> Parse<'a> for Token<'a, ()> {
    parser!(|_| unimplemented!("EOF detection is in weedle::parse"));

    fn write(&self) -> String {
        self.trivia.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_optional_present { "one" =>
        "";
        Option<Token<Identifier>>;
        is_some();
    });

    test!(should_parse_optional_not_present { "" =>
        "";
        Option<Token<Identifier>>;
        is_none();
    });

    test!(should_parse_boxed { "one" =>
        "";
        Box<Token<Identifier>>;
    });

    test!(should_parse_vec { "one two three" =>
        "";
        Vec<Token<Identifier>>;
        len() == 3;
    });

    test!(should_parse_parenthesized { "( one )" =>
        "";
        Parenthesized<Token<Identifier>>;
        body.value.0 == "one";
    });

    test!(should_parse_bracketed { "[ one ]" =>
        "";
        Bracketed<Token<Identifier>>;
        body.value.0 == "one";
    });

    test!(should_parse_braced { "{ one }" =>
        "";
        Braced<Token<Identifier>>;
        body.value.0 == "one";
    });

    test!(should_parse_generics { "<one>" =>
        "";
        Generics<Token<Identifier>>;
        body.value.0 == "one";
    });

    test!(should_parse_generics_two { "<one, two>" =>
        "";
        Generics<(Token<Identifier>, Token<term::Comma>, Token<Identifier>)> =>
            Generics {
                open_angle: Token::default(),
                body: (
                    Token { value: Identifier("one"), trivia: "" },
                    Token::default(),
                    Token { value: Identifier("two"), trivia: " " },
                ),
                close_angle: Token::default(),
            }
    });

    test!(should_parse_comma_separated_values { "one, two, three" =>
        "";
        Punctuated<Token<Identifier>, Token<term::Comma>>;
        list.len() == 3;
    });

    test!(err should_not_parse_comma_separated_values_empty { "" =>
        PunctuatedNonEmpty<Token<Identifier>, Token<term::Comma>>
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
        Token<Identifier>;
        value.0 == "hello";
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

use nom::{IResult, Parser};

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::{common::Identifier, lexer::keywords};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeNoArgs<'a>(pub VariantToken<'a, Identifier<'a>>);

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExtendedAttribute<'a> {
    NoArgs(ExtendedAttributeNoArgs<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeList<'a> {
    pub open_bracket: VariantToken<'a, keywords::OpenBracket<'a>>,
    pub body: Vec<ExtendedAttribute<'a>>,
    pub close_bracket: VariantToken<'a, keywords::CloseBracket<'a>>,
}

pub fn extended_attribute_list<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, ExtendedAttributeList<'token>> {
    let (remaining, (open_bracket, body, close_bracket, _)) = nom::sequence::tuple((
        eat_key!(OpenBracket),
        // TODO: Store commas too
        nom::multi::separated_list1(
            eat_key!(Comma),
            nom::branch::alt((eat!(Id)
                .map(ExtendedAttributeNoArgs)
                .map(ExtendedAttribute::NoArgs),)),
        ),
        eat_key!(CloseBracket),
        nom::combinator::cut(nom::combinator::not(eat_key!(OpenBracket))),
    ))(tokens)?;

    Ok((
        remaining,
        ExtendedAttributeList {
            open_bracket,
            body,
            close_bracket,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        single_extended_attribute_no_args,
        extended_attribute_list,
        "[Foo]",
        ExtendedAttributeList {
            body,
            ..
        } if matches!(&body[..], [
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                variant: Identifier("Foo"),
                ..
            }))
        ])
    );

    test_match!(
        double_extended_attribute_no_args,
        extended_attribute_list,
        "[Foo, Bar]",
        ExtendedAttributeList {
            body,
            ..
        } if matches!(&body[..], [
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                variant: Identifier("Foo"),
                ..
            })),
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                variant: Identifier("Bar"),
                ..
            }))
        ])
    );

    test_result_match!(
        double_extended_attribute_list,
        extended_attribute_list,
        "[Foo][Foo]",
        Err(nom::Err::Failure(_))
    );
}

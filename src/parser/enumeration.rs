// https://webidl.spec.whatwg.org/#prod-Enum

use nom::IResult;

use crate::{common::Identifier, lexer::keywords, literal::StringLit};

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EnumDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub r#enum: VariantToken<'a, keywords::Enum<'a>>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    pub body: Vec<VariantToken<'a, StringLit<'a>>>,
    pub close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl EnumDefinition<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, EnumDefinition<'token>> {
        let (tokens, (r#enum, identifier, open_brace, body, _, close_brace, semi_colon)) =
            nom::sequence::tuple((
                eat_key!(Enum),
                nom::combinator::cut(eat!(Id)),
                nom::combinator::cut(eat_key!(OpenBrace)),
                // TODO: store commas too
                nom::combinator::cut(nom::multi::separated_list1(eat_key!(Comma), eat!(Str))),
                nom::combinator::opt(eat_key!(Comma)),
                nom::combinator::cut(eat_key!(CloseBrace)),
                nom::combinator::cut(eat_key!(SemiColon)),
            ))(tokens)?;

        Ok((
            tokens,
            EnumDefinition {
                ext_attrs: None,
                r#enum,
                identifier,
                open_brace,
                body,
                close_brace,
                semi_colon,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        enum_single,
        EnumDefinition::parse,
        "enum Foo { \"foo\" };",
        EnumDefinition {
            identifier: VariantToken {
              variant: Identifier("Foo"),
              ..
            },
            body,
            ..
        } if matches!(&body[..], [
          VariantToken {
            variant: StringLit("foo"),
            ..
          }
        ])
    );

    test_match!(
        enum_double_dangling_comma,
        EnumDefinition::parse,
        "enum Foo { \"foo\", \"bar\", };",
        EnumDefinition {
            identifier: VariantToken {
              variant: Identifier("Foo"),
              ..
            },
            body,
            ..
        } if matches!(&body[..], [
          VariantToken {
            variant: StringLit("foo"),
            ..
          },
          VariantToken {
            variant: StringLit("bar"),
            ..
          }
        ])
    );

    test_result_match!(
        enum_empty,
        EnumDefinition::parse,
        "enum Foo {};",
        Err(nom::Err::Failure(_))
    );
}

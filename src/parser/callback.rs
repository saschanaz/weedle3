// https://webidl.spec.whatwg.org/#prod-CallbackRest

use nom::IResult;

use crate::{common::Identifier, lexer::keywords};

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
    r#type::Type,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CallbackDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub callback: VariantToken<'a, keywords::Callback<'a>>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: VariantToken<'a, keywords::Assign<'a>>,
    pub r#type: Type<'a>,
    pub open_paren: VariantToken<'a, keywords::OpenParen<'a>>,
    pub close_paren: VariantToken<'a, keywords::CloseParen<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl CallbackDefinition<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, CallbackDefinition<'token>> {
        let (tokens, (callback, identifier, assign, r#type, open_paren, close_paren, semi_colon)) =
            nom::sequence::tuple((
                eat_key!(Callback),
                nom::combinator::cut(eat!(Id)),
                nom::combinator::cut(eat_key!(Assign)),
                nom::combinator::cut(Type::parse),
                nom::combinator::cut(eat_key!(OpenParen)),
                nom::combinator::cut(eat_key!(CloseParen)),
                nom::combinator::cut(eat_key!(SemiColon)),
            ))(tokens)?;

        Ok((
            tokens,
            CallbackDefinition {
                ext_attrs: None,
                callback,
                identifier,
                assign,
                r#type,
                open_paren,
                close_paren,
                semi_colon,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::r#type::primitive_type::PrimitiveType;

    use super::*;

    test_match!(
        no_argument_callback,
        CallbackDefinition::parse,
        "callback Foo = float ();",
        CallbackDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            r#type: Type::Primitive(PrimitiveType::Float(_)),
            ..
        }
    );
}

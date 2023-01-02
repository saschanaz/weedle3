// https://webidl.spec.whatwg.org/#prod-Dictionary

mod member;

use nom::IResult;

use crate::{common::Identifier, lexer::keywords};

use self::member::DictionaryMember;

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub dictionary: VariantToken<'a, keywords::Dictionary<'a>>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    pub body: Vec<DictionaryMember<'a>>,
    pub close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl DictionaryDefinition<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, DictionaryDefinition<'token>> {
        // TODO: fill more things
        let (tokens, (dictionary, identifier, open_brace, members, semi_colon)) =
            nom::sequence::tuple((
                eat_key!(Dictionary),
                nom::combinator::cut(eat!(Id)),
                nom::combinator::cut(eat_key!(OpenBrace)),
                nom::multi::many_till(DictionaryMember::parse, eat_key!(CloseBrace)),
                nom::combinator::cut(eat_key!(SemiColon)),
            ))(tokens)?;

        Ok((
            tokens,
            DictionaryDefinition {
                ext_attrs: None,
                dictionary,
                identifier,
                open_brace,
                body: members.0,
                close_brace: members.1,
                semi_colon,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::r#type::{
        primitive_type::PrimitiveType, DistinguishableType, NullableType, Type,
        TypeWithExtendedAttributes,
    };

    use super::*;

    test_match!(
        empty_dictionary,
        DictionaryDefinition::parse,
        "dictionary Foo {};",
        DictionaryDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        single_member_dictionary,
        DictionaryDefinition::parse,
        "dictionary Foo { required float bar; };",
        DictionaryDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            body,
            ..
        } if matches!(&body[..], [DictionaryMember {
            required: Some(_),
            r#type: TypeWithExtendedAttributes {
                r#type: Type::Distinguishable(NullableType { r#type: DistinguishableType::Primitive(PrimitiveType::Float(_)) , .. }),
                ..
            },
            identifier: VariantToken {
                variant: Identifier("bar"),
                ..
            },
            ..
        }])
    );
}
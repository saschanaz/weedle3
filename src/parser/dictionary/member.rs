use nom::IResult;

use crate::parser::extended_attributes::{extended_attribute_list, ExtendedAttributeList};
use crate::parser::r#type::{type_with_extended_attributes, TypeWithExtendedAttributes};
use crate::{common::Identifier, lexer::keywords};

use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryMember<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub required: Option<VariantToken<'a, keywords::Required<'a>>>,
    pub r#type: TypeWithExtendedAttributes<'a>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn dictionary_member<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, DictionaryMember<'token>> {
    // TODO: fill more things
    let (tokens, (ext_attrs, required, r#type, identifier, semi_colon)) = nom::sequence::tuple((
        nom::combinator::opt(extended_attribute_list),
        nom::combinator::opt(eat_key!(Required)),
        type_with_extended_attributes,
        eat!(Id),
        eat_key!(SemiColon),
    ))(tokens)?;

    Ok((
        tokens,
        DictionaryMember {
            ext_attrs,
            required,
            r#type,
            identifier,
            semi_colon,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{
        extended_attributes::{ExtendedAttribute, ExtendedAttributeNoArgs},
        r#type::{primitive_type::PrimitiveType, Type},
    };

    test_match!(
        required_member,
        dictionary_member,
        "required float foo;",
        DictionaryMember {
            required: Some(_),
            r#type: TypeWithExtendedAttributes {
                r#type: Type::Primitive(PrimitiveType::Float(_)),
                ..
            },
            identifier: VariantToken {
                variant: Identifier("foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        non_required_member,
        dictionary_member,
        "float foo;",
        DictionaryMember {
            required: None,
            r#type: TypeWithExtendedAttributes {
                r#type: Type::Primitive(PrimitiveType::Float(_)),
                ..
            },
            identifier: VariantToken {
                variant: Identifier("foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        extended_required_member,
        dictionary_member,
        "[Foo] required [Clamp] float foo;",
        DictionaryMember {
            ext_attrs: Some(ExtendedAttributeList{
                body: mem_ext_attrs,
                ..
            }),
            required: Some(_),
            r#type: TypeWithExtendedAttributes {
                ext_attrs: Some(ExtendedAttributeList{
                    body: typ_ext_attrs,
                    ..
                }),
                r#type: Type::Primitive(PrimitiveType::Float(_)),
            },
            identifier: VariantToken {
                variant: Identifier("foo"),
                ..
            },
            ..
        } if matches!(&mem_ext_attrs[..], [ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
            variant: Identifier("Foo"),
            ..
        }))]) && matches!(&typ_ext_attrs[..], [ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
            variant: Identifier("Clamp"),
            ..
        }))])
    );

    test_result_match!(
        double_extended_member,
        dictionary_member,
        "[Foo] [Clamp] float foo;",
        Err(nom::Err::Failure(_))
    );
}

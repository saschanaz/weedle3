// https://webidl.spec.whatwg.org/#prod-Type

pub mod primitive_type;
pub use primitive_type::primitive_type;

use nom::{IResult, Parser};

use self::primitive_type::PrimitiveType;

use super::{
    extended_attributes::{extended_attribute_list, ExtendedAttributeList},
    impl_nom_traits::Tokens,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Primitive(PrimitiveType<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeWithExtendedAttributes<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub r#type: Type<'a>,
}

pub fn r#type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, Type<'token>> {
    // TODO: fill more things
    nom::branch::alt((primitive_type.map(Type::Primitive),))(tokens)
}

pub fn type_with_extended_attributes<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, TypeWithExtendedAttributes<'token>> {
    // TODO: fill more things
    let (tokens, (ext_attrs, r#type)) =
        nom::sequence::tuple((nom::combinator::opt(extended_attribute_list), r#type))(tokens)?;

    Ok((tokens, TypeWithExtendedAttributes { ext_attrs, r#type }))
}

#[cfg(test)]
mod tests {
    use crate::{
        common::Identifier,
        parser::{
            eat::VariantToken,
            extended_attributes::{ExtendedAttribute, ExtendedAttributeNoArgs},
        },
    };

    use super::*;

    test_match!(
        unsigned_long_long,
        r#type,
        "unsigned long long",
        Type::Primitive(_)
    );

    test_match!(
        clamp_unsigned_long_long,
        type_with_extended_attributes,
        "[Clamp] unsigned long long",
        TypeWithExtendedAttributes {
            ext_attrs: Some(attrs),
            r#type: Type::Primitive(_),
        } if matches!(&attrs.body[..], [
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                variant: Identifier("Clamp"),
                ..
            }))
        ])
    );
}

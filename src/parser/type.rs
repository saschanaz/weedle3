// https://webidl.spec.whatwg.org/#prod-Type

pub mod primitive_type;
pub use primitive_type::PrimitiveType;

pub mod string_type;
pub use string_type::StringType;

use nom::{IResult, Parser};

use super::{extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Primitive(PrimitiveType<'a>),
    String(StringType<'a>),
}

impl Type<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, Type<'token>> {
        // TODO: fill more things
        nom::branch::alt((
            PrimitiveType::parse.map(Type::Primitive),
            StringType::parse.map(Type::String),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeWithExtendedAttributes<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub r#type: Type<'a>,
}

impl TypeWithExtendedAttributes<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, TypeWithExtendedAttributes<'token>> {
        // TODO: fill more things
        let (tokens, (ext_attrs, r#type)) = nom::sequence::tuple((
            nom::combinator::opt(ExtendedAttributeList::parse),
            Type::parse,
        ))(tokens)?;

        Ok((tokens, TypeWithExtendedAttributes { ext_attrs, r#type }))
    }
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
        Type::parse,
        "unsigned long long",
        Type::Primitive(_)
    );

    test_match!(
        clamp_unsigned_long_long,
        TypeWithExtendedAttributes::parse,
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

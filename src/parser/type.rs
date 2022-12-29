// https://webidl.spec.whatwg.org/#prod-Type

pub mod buffer_related_type;
pub mod primitive_type;
pub mod record_type;
pub mod sequence_type;
pub mod string_type;

use nom::{IResult, Parser};

use crate::{common::Identifier, lexer::keywords};

use self::{
    buffer_related_type::BufferRelatedType, primitive_type::PrimitiveType, record_type::RecordType,
    sequence_type::SequenceType, string_type::StringType,
};

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Primitive(PrimitiveType<'a>),
    String(StringType<'a>),
    Identifier(VariantToken<'a, Identifier<'a>>),
    Sequence(SequenceType<'a>),
    Object(VariantToken<'a, keywords::Object<'a>>),
    Symbol(VariantToken<'a, keywords::Symbol<'a>>),
    BufferRelated(BufferRelatedType<'a>),
    Record(RecordType<'a>),
    Undefined(VariantToken<'a, keywords::Undefined<'a>>),
}

impl Type<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, Type<'token>> {
        // TODO: fill more things
        nom::branch::alt((
            PrimitiveType::parse.map(Type::Primitive),
            StringType::parse.map(Type::String),
            eat!(Id).map(Type::Identifier),
            SequenceType::parse.map(Type::Sequence),
            eat_key!(Object).map(Type::Object),
            eat_key!(Symbol).map(Type::Symbol),
            BufferRelatedType::parse.map(Type::BufferRelated),
            RecordType::parse.map(Type::Record),
            eat_key!(Undefined).map(Type::Undefined),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NullableType<'a> {
    pub r#type: Type<'a>,
    pub q_mark: Option<VariantToken<'a, keywords::QuestionMark<'a>>>,
}

impl NullableType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, NullableType<'token>> {
        let (tokens, (r#type, q_mark)) = nom::sequence::tuple((
            Type::parse,
            nom::combinator::opt(eat_key!(QuestionMark)),
        ))(tokens)?;

        Ok((tokens, NullableType { r#type, q_mark }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypeWithExtendedAttributes<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub r#type: NullableType<'a>,
}

impl TypeWithExtendedAttributes<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, TypeWithExtendedAttributes<'token>> {
        // TODO: fill more things
        let (tokens, (ext_attrs, r#type)) = nom::sequence::tuple((
            nom::combinator::opt(ExtendedAttributeList::parse),
            NullableType::parse,
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
            r#type: NullableType { r#type: Type::Primitive(_), q_mark: None }
        } if matches!(&attrs.body[..], [
            ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                variant: Identifier("Clamp"),
                ..
            }))
        ])
    );

    test_match!(
        xfoo_foo_null,
        TypeWithExtendedAttributes::parse,
        "[XFoo] Foo?",
        TypeWithExtendedAttributes {
            ext_attrs: Some(attrs),
            r#type: NullableType { r#type: Type::Identifier(_), q_mark: Some(_) }
        } if matches!(&attrs.body[..], [
            ExtendedAttribute::NoArgs(_)
        ])
    );
}

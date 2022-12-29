// https://webidl.spec.whatwg.org/#prod-Type

pub mod buffer_related_type;
pub mod primitive_type;
pub mod record_type;
pub mod sequence_type;
pub mod string_type;

use nom::{combinator::cut, IResult, Parser};

use crate::{common::Identifier, lexer::keywords};

use self::{
    buffer_related_type::BufferRelatedType, primitive_type::PrimitiveType, record_type::RecordType,
    sequence_type::SequenceType, string_type::StringType,
};

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DistinguishableType<'a> {
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

impl DistinguishableType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, DistinguishableType<'token>> {
        // TODO: fill more things
        nom::branch::alt((
            PrimitiveType::parse.map(DistinguishableType::Primitive),
            StringType::parse.map(DistinguishableType::String),
            eat!(Id).map(DistinguishableType::Identifier),
            SequenceType::parse.map(DistinguishableType::Sequence),
            eat_key!(Object).map(DistinguishableType::Object),
            eat_key!(Symbol).map(DistinguishableType::Symbol),
            BufferRelatedType::parse.map(DistinguishableType::BufferRelated),
            RecordType::parse.map(DistinguishableType::Record),
            eat_key!(Undefined).map(DistinguishableType::Undefined),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NullableType<'a> {
    pub r#type: DistinguishableType<'a>,
    pub q_mark: Option<VariantToken<'a, keywords::QuestionMark<'a>>>,
}

impl NullableType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, NullableType<'token>> {
        let (tokens, (r#type, q_mark)) = nom::sequence::tuple((
            DistinguishableType::parse,
            nom::combinator::opt(eat_key!(QuestionMark)),
        ))(tokens)?;

        Ok((tokens, NullableType { r#type, q_mark }))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Distinguishable(NullableType<'a>),
    Any(VariantToken<'a, keywords::Any<'a>>),
    // Promise(),
}

impl Type<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, Type<'token>> {
        nom::branch::alt((
            NullableType::parse.map(Type::Distinguishable),
            nom::sequence::tuple((
                eat_key!(Any),
                cut(nom::combinator::not(eat_key!(QuestionMark))),
            ))
            .map(|(any, _)| Type::Any(any)),
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
        DistinguishableType::parse,
        "unsigned long long",
        DistinguishableType::Primitive(_)
    );

    test_match!(
        clamp_unsigned_long_long,
        TypeWithExtendedAttributes::parse,
        "[Clamp] unsigned long long",
        TypeWithExtendedAttributes {
            ext_attrs: Some(attrs),
            r#type: Type::Distinguishable(NullableType { r#type: DistinguishableType::Primitive(_), q_mark: None })
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
            r#type: Type::Distinguishable(NullableType { r#type: DistinguishableType::Identifier(_), q_mark: Some(_) })
        } if matches!(&attrs.body[..], [
            ExtendedAttribute::NoArgs(_)
        ])
    );

    test_match!(
        xfoo_any,
        TypeWithExtendedAttributes::parse,
        "[XFoo] any",
        TypeWithExtendedAttributes {
            ext_attrs: Some(attrs),
            r#type: Type::Any(_)
        } if matches!(&attrs.body[..], [
            ExtendedAttribute::NoArgs(_)
        ])
    );

    test_result_match!(
        any_null,
        TypeWithExtendedAttributes::parse,
        "any?",
        Err(nom::Err::Failure(_))
    );
}

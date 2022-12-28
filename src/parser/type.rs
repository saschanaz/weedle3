// https://webidl.spec.whatwg.org/#prod-Type

pub mod primitive_type;
pub mod record_type;
pub mod sequence_type;
pub mod string_type;

use nom::{IResult, Parser};

use crate::{common::Identifier, lexer::keywords};

use self::{
    primitive_type::PrimitiveType, record_type::RecordType, sequence_type::SequenceType,
    string_type::StringType,
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

    ArrayBuffer(VariantToken<'a, keywords::ArrayBuffer<'a>>),
    DataView(VariantToken<'a, keywords::DataView<'a>>),
    Int8Array(VariantToken<'a, keywords::Int8Array<'a>>),
    Int16Array(VariantToken<'a, keywords::Int16Array<'a>>),
    Int32Array(VariantToken<'a, keywords::Int32Array<'a>>),
    Uint8Array(VariantToken<'a, keywords::Uint8Array<'a>>),
    Uint16Array(VariantToken<'a, keywords::Uint16Array<'a>>),
    Uint32Array(VariantToken<'a, keywords::Uint32Array<'a>>),
    Uint8ClampedArray(VariantToken<'a, keywords::Uint8ClampedArray<'a>>),
    BigInt64Array(VariantToken<'a, keywords::BigInt64Array<'a>>),
    BigUint64Array(VariantToken<'a, keywords::BigUint64Array<'a>>),
    Float32Array(VariantToken<'a, keywords::Float32Array<'a>>),
    Float64Array(VariantToken<'a, keywords::Float64Array<'a>>),

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
            eat_key!(ArrayBuffer).map(Type::ArrayBuffer),
            eat_key!(DataView).map(Type::DataView),
            eat_key!(Int8Array).map(Type::Int8Array),
            eat_key!(Int16Array).map(Type::Int16Array),
            eat_key!(Int32Array).map(Type::Int32Array),
            eat_key!(Uint8Array).map(Type::Uint8Array),
            eat_key!(Uint16Array).map(Type::Uint16Array),
            eat_key!(Uint32Array).map(Type::Uint32Array),
            eat_key!(Uint8ClampedArray).map(Type::Uint8ClampedArray),
            eat_key!(BigInt64Array).map(Type::BigInt64Array),
            eat_key!(BigUint64Array).map(Type::BigUint64Array),
            eat_key!(Float32Array).map(Type::Float32Array),
            eat_key!(Float64Array).map(Type::Float64Array),
            RecordType::parse.map(Type::Record),
            eat_key!(Undefined).map(Type::Undefined),
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

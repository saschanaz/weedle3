// https://webidl.spec.whatwg.org/#prod-BufferRelatedType

use nom::{IResult, Parser};

use crate::lexer::keywords;
use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BufferRelatedType<'a> {
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
}

impl BufferRelatedType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, BufferRelatedType<'token>> {
        nom::branch::alt((
            eat_key!(ArrayBuffer).map(BufferRelatedType::ArrayBuffer),
            eat_key!(DataView).map(BufferRelatedType::DataView),
            eat_key!(Int8Array).map(BufferRelatedType::Int8Array),
            eat_key!(Int16Array).map(BufferRelatedType::Int16Array),
            eat_key!(Int32Array).map(BufferRelatedType::Int32Array),
            eat_key!(Uint8Array).map(BufferRelatedType::Uint8Array),
            eat_key!(Uint16Array).map(BufferRelatedType::Uint16Array),
            eat_key!(Uint32Array).map(BufferRelatedType::Uint32Array),
            eat_key!(Uint8ClampedArray).map(BufferRelatedType::Uint8ClampedArray),
            eat_key!(BigInt64Array).map(BufferRelatedType::BigInt64Array),
            eat_key!(BigUint64Array).map(BufferRelatedType::BigUint64Array),
            eat_key!(Float32Array).map(BufferRelatedType::Float32Array),
            eat_key!(Float64Array).map(BufferRelatedType::Float64Array),
        ))(tokens)
    }
}

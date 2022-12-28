// https://webidl.spec.whatwg.org/#prod-Const

use nom::{IResult, Parser};

use crate::{
    common::Identifier,
    lexer::keywords,
    literal::{FloatValueLit, IntegerLit},
    parser::{eat::VariantToken, impl_nom_traits::Tokens, r#type::primitive_type::PrimitiveType},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstType<'a> {
    Primitive(PrimitiveType<'a>),
    Identifier(VariantToken<'a, Identifier<'a>>),
}

impl ConstType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, ConstType<'token>> {
        nom::branch::alt((
            PrimitiveType::parse.map(ConstType::Primitive),
            eat!(Id).map(ConstType::Identifier),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BooleanLit<'a> {
    True(VariantToken<'a, keywords::True<'a>>),
    False(VariantToken<'a, keywords::False<'a>>),
}

impl BooleanLit<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, BooleanLit<'token>> {
        nom::branch::alt((
            eat_key!(True).map(BooleanLit::True),
            eat_key!(False).map(BooleanLit::False),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatLit<'a> {
    Value(VariantToken<'a, FloatValueLit<'a>>),
    NegInfinity(VariantToken<'a, keywords::NegInfinity<'a>>),
    Infinity(VariantToken<'a, keywords::Infinity<'a>>),
    NaN(VariantToken<'a, keywords::NaN<'a>>),
}

impl FloatLit<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, FloatLit<'token>> {
        nom::branch::alt((
            eat!(Dec).map(FloatLit::Value),
            eat_key!(NegInfinity).map(FloatLit::NegInfinity),
            eat_key!(Infinity).map(FloatLit::Infinity),
            eat_key!(NaN).map(FloatLit::NaN),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstValue<'a> {
    Integer(VariantToken<'a, IntegerLit<'a>>),
    Boolean(BooleanLit<'a>),
    Float(FloatLit<'a>),
}

impl ConstValue<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, ConstValue<'token>> {
        nom::branch::alt((
            eat!(Int).map(ConstValue::Integer),
            BooleanLit::parse.map(ConstValue::Boolean),
            FloatLit::parse.map(ConstValue::Float),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstMember<'a> {
    r#const: VariantToken<'a, keywords::Const<'a>>,
    const_type: ConstType<'a>,
    identifier: VariantToken<'a, Identifier<'a>>,
    assign: VariantToken<'a, keywords::Assign<'a>>,
    const_value: ConstValue<'a>,
    semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl ConstMember<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, ConstMember<'token>> {
        let (tokens, (r#const, const_type, identifier, assign, const_value, semi_colon)) =
            nom::sequence::tuple((
                eat_key!(Const),
                nom::combinator::cut(ConstType::parse),
                nom::combinator::cut(eat!(Id)),
                nom::combinator::cut(eat_key!(Assign)),
                nom::combinator::cut(ConstValue::parse),
                nom::combinator::cut(eat_key!(SemiColon)),
            ))(tokens)?;

        Ok((
            tokens,
            ConstMember {
                r#const,
                const_type,
                identifier,
                assign,
                const_value,
                semi_colon,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        literal::DecLit,
        parser::r#type::primitive_type::{FloatSize, FloatType, IntegerSize, IntegerType},
    };

    use super::*;

    test_match!(
        integer_type,
        ConstMember::parse,
        "const short Foo = 42;",
        ConstMember {
            const_type: ConstType::Primitive(PrimitiveType::Integer(IntegerType {
                size: IntegerSize::Short(_),
                ..
            })),
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            const_value: ConstValue::Integer(VariantToken {
                variant: IntegerLit::Dec(DecLit("42")),
                ..
            }),
            ..
        }
    );

    test_match!(
        boolean_type,
        ConstMember::parse,
        "const boolean Foo = true;",
        ConstMember {
            const_type: ConstType::Primitive(PrimitiveType::Boolean(_)),
            const_value: ConstValue::Boolean(BooleanLit::True(_)),
            ..
        }
    );

    test_match!(
        float_type,
        ConstMember::parse,
        "const float Foo = 4.2;",
        ConstMember {
            const_type: ConstType::Primitive(PrimitiveType::Float(FloatType {
                size: FloatSize::Float(_),
                ..
            })),
            const_value: ConstValue::Float(FloatLit::Value(VariantToken {
                variant: FloatValueLit("4.2"),
                ..
            })),
            ..
        }
    );
}

use nom::{IResult, Parser};

use crate::{
    common::Identifier,
    lexer::keywords,
    literal::IntegerLit,
    parser::{
        eat::VariantToken,
        impl_nom_traits::Tokens,
        r#type::primitive_type::{primitive_type, PrimitiveType},
    },
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstType<'a> {
    Primitive(PrimitiveType<'a>),
    Identifier(VariantToken<'a, Identifier<'a>>),
}

pub fn const_type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, ConstType<'token>> {
    nom::branch::alt((
        primitive_type.map(ConstType::Primitive),
        eat!(Id).map(ConstType::Identifier),
    ))(tokens)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstValue<'a> {
    Integer(VariantToken<'a, IntegerLit<'a>>),
}

pub fn const_value<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, ConstValue<'token>> {
    nom::branch::alt((eat!(Int).map(ConstValue::Integer),))(tokens)
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

pub fn const_member<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, ConstMember<'token>> {
    let (tokens, (r#const, const_type, identifier, assign, const_value, semi_colon)) =
        nom::sequence::tuple((
            eat_key!(Const),
            nom::combinator::cut(const_type),
            nom::combinator::cut(eat!(Id)),
            nom::combinator::cut(eat_key!(Assign)),
            nom::combinator::cut(const_value),
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

#[cfg(test)]
mod tests {
    use crate::{
        literal::DecLit,
        parser::r#type::primitive_type::{IntegerSize, IntegerType},
    };

    use super::*;

    test_match!(
        integer_type,
        const_member,
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
}

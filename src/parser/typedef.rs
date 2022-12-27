// https://webidl.spec.whatwg.org/#prod-Typedef

use nom::IResult;

use crate::{common::Identifier, lexer::keywords};

use super::{
    eat::VariantToken,
    extended_attributes::ExtendedAttributeList,
    impl_nom_traits::Tokens,
    r#type::{type_with_extended_attributes, TypeWithExtendedAttributes},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TypedefDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub typedef: VariantToken<'a, keywords::Typedef<'a>>,
    pub r#type: TypeWithExtendedAttributes<'a>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn typedef<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, TypedefDefinition<'token>> {
    let (tokens, (typedef, r#type, identifier, semi_colon)) = nom::sequence::tuple((
        eat_key!(Typedef),
        nom::combinator::cut(type_with_extended_attributes),
        nom::combinator::cut(eat!(Id)),
        nom::combinator::cut(eat_key!(SemiColon)),
    ))(tokens)?;

    Ok((
        tokens,
        TypedefDefinition {
            ext_attrs: None,
            typedef,
            r#type,
            identifier,
            semi_colon,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parser::r#type::{primitive_type::PrimitiveType, Type};

    use super::*;

    test_match!(
        typedef_foo,
        typedef,
        "typedef float Foo;",
        TypedefDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            r#type: TypeWithExtendedAttributes {
                r#type: Type::Primitive(PrimitiveType::Float(_)),
                ..
            },
            ..
        }
    );

    test_result_match!(
        typedef_empty,
        typedef,
        "typedef;",
        Err(nom::Err::Failure(_))
    );
}

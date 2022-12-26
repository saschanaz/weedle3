use nom::IResult;

use crate::parser::r#type::{r#type, Type};
use crate::{common::Identifier, lexer::keywords};

use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryMember<'a> {
    pub required: Option<VariantToken<'a, keywords::Required<'a>>>,
    pub r#type: Type<'a>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn dictionary_member<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, DictionaryMember<'token>> {
    // TODO: fill more things
    let (tokens, (required, r#type, identifier, semi_colon)) = nom::sequence::tuple((
        nom::combinator::opt(eat_key!(Required)),
        r#type,
        eat!(Id),
        eat_key!(SemiColon),
    ))(tokens)?;

    Ok((
        tokens,
        DictionaryMember {
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
    use crate::parser::r#type::primitive_type::PrimitiveType;

    test_match!(
        required_member,
        dictionary_member,
        "required float Foo;",
        DictionaryMember {
            required: Some(_),
            r#type: Type::Primitive(PrimitiveType::Float(_)),
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        non_required_member,
        dictionary_member,
        "float Foo;",
        DictionaryMember {
            required: None,
            r#type: Type::Primitive(PrimitiveType::Float(_)),
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );
}

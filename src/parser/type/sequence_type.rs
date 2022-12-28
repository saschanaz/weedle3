// https://webidl.spec.whatwg.org/#prod-DistinguishableType

use nom::combinator::cut;
use nom::{IResult, Parser};

use crate::lexer::keywords;
use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

use super::TypeWithExtendedAttributes;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SequenceToken<'a> {
    Sequence(VariantToken<'a, keywords::Sequence<'a>>),
    FrozenArray(VariantToken<'a, keywords::FrozenArray<'a>>),
    ObservableArray(VariantToken<'a, keywords::ObservableArray<'a>>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SequenceType<'a> {
    sequence: SequenceToken<'a>,
    open_angle: VariantToken<'a, keywords::LessThan<'a>>,
    r#type: Box<TypeWithExtendedAttributes<'a>>,
    close_angle: VariantToken<'a, keywords::GreaterThan<'a>>,
}

impl SequenceType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, SequenceType<'token>> {
        let (tokens, (sequence, open_angle, r#type, close_angle)) = nom::sequence::tuple((
            nom::branch::alt((
                eat_key!(Sequence).map(SequenceToken::Sequence),
                eat_key!(FrozenArray).map(SequenceToken::FrozenArray),
                eat_key!(ObservableArray).map(SequenceToken::ObservableArray),
            )),
            cut(eat_key!(LessThan)),
            cut(TypeWithExtendedAttributes::parse),
            cut(eat_key!(GreaterThan)),
        ))(tokens)?;

        Ok((
            tokens,
            SequenceType {
                sequence,
                open_angle,
                r#type: Box::new(r#type),
                close_angle,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::r#type::{primitive_type::PrimitiveType, Type};

    use super::*;

    test_match!(
        sequence_short,
        SequenceType::parse,
        "sequence<short>",
        SequenceType {
            sequence: SequenceToken::Sequence(_),
            r#type,
            ..
        } if matches!(*r#type, TypeWithExtendedAttributes {
            r#type: Type::Primitive(PrimitiveType::Integer(_)),
            ..
        })
    );
}

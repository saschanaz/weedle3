// https://webidl.spec.whatwg.org/#prod-RecordType

use nom::combinator::cut;
use nom::IResult;

use crate::lexer::keywords;
use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

use super::string_type::StringType;
use super::TypeWithExtendedAttributes;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordType<'a> {
    record: VariantToken<'a, keywords::Record<'a>>,
    open_angle: VariantToken<'a, keywords::LessThan<'a>>,
    string_type: StringType<'a>,
    separator: VariantToken<'a, keywords::Comma<'a>>,
    r#type: Box<TypeWithExtendedAttributes<'a>>,
    close_angle: VariantToken<'a, keywords::GreaterThan<'a>>,
}

impl RecordType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, RecordType<'token>> {
        let (tokens, (record, open_angle, string_type, separator, r#type, close_angle)) =
            nom::sequence::tuple((
                eat_key!(Record),
                cut(eat_key!(LessThan)),
                cut(StringType::parse),
                cut(eat_key!(Comma)),
                cut(TypeWithExtendedAttributes::parse),
                cut(eat_key!(GreaterThan)),
            ))(tokens)?;

        Ok((
            tokens,
            RecordType {
                record,
                open_angle,
                string_type,
                separator,
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
        record_string_short,
        RecordType::parse,
        "record<DOMString, short>",
        RecordType {
            string_type: StringType::Dom(_),
            r#type,
            ..
        } if matches!(*r#type, TypeWithExtendedAttributes {
            r#type: Type::Primitive(PrimitiveType::Integer(_)),
            ..
        })
    );
}

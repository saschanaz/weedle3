mod primitive_type;
use primitive_type::primitive_type;

use nom::{IResult, Parser};

use self::primitive_type::PrimitiveType;

use super::{impl_nom_traits::Tokens};

pub enum Type<'a> {
    Primitive(PrimitiveType<'a>),
}

pub fn r#type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, Type<'token>> {
    nom::branch::alt((primitive_type.map(Type::Primitive),))(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        unsigned_long_long,
        r#type,
        "unsigned long long",
        Type::Primitive(_)
    );
}

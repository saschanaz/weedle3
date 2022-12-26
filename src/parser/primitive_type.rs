// https://webidl.spec.whatwg.org/#prod-PrimitiveType

use nom::{IResult, Parser};

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::lexer::{keywords, Token};

pub enum IntegerSize<'a> {
    LongLong((VariantToken<'a, keywords::Long<'a>>, VariantToken<'a, keywords::Long<'a>>)),
    Long(VariantToken<'a, keywords::Long<'a>>),
    Short(VariantToken<'a, keywords::Short<'a>>),
}

pub struct IntegerType<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub size: IntegerSize<'a>,
}

pub enum FloatSize<'a> {
    Float(VariantToken<'a, keywords::Float<'a>>),
    Double(VariantToken<'a, keywords::Double<'a>>),
}

pub struct FloatType<'a> {
    pub unrestricted: Option<VariantToken<'a, keywords::Unrestricted<'a>>>,
    pub size: FloatSize<'a>,
}

fn integer_size<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IntegerSize<'token>> {
    nom::branch::alt((
        eat_key!(Short).map(IntegerSize::Short),
        nom::sequence::tuple((eat_key!(Long), nom::combinator::opt(eat_key!(Long)))).map(
            |(long, long_long)| match long_long {
                Some(long_long) => IntegerSize::LongLong((long, long_long)),
                None => IntegerSize::Long(long),
            },
        ),
    ))(tokens)
}

fn integer_type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>> {
    // TODO: use nom::error::VerboseErrorKind? how?
    nom::branch::alt((
        nom::sequence::tuple((eat_key!(Unsigned), nom::combinator::cut(integer_size))).map(
            |(unsigned, size)| IntegerType {
                unsigned: Some(unsigned),
                size,
            },
        ),
        integer_size.map(|size| IntegerType {
            unsigned: None,
            size,
        }),
    ))(tokens)
}

fn float_size<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, FloatSize<'token>> {
    nom::branch::alt((
        eat_key!(Float).map(FloatSize::Float),
        eat_key!(Double).map(FloatSize::Double),
    ))(tokens)
}

fn float_type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, FloatType<'token>> {
    // TODO: use nom::error::VerboseErrorKind? how?
    nom::branch::alt((
        nom::sequence::tuple((eat_key!(Unrestricted), nom::combinator::cut(float_size))).map(
            |(unrestricted, size)| FloatType {
                unrestricted: Some(unrestricted),
                size,
            },
        ),
        float_size.map(|size| FloatType {
            unrestricted: None,
            size,
        }),
    ))(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        unsigned_long_long,
        integer_type,
        "unsigned long long",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::LongLong(_),
        }
    );

    test_match!(
        signed_long_long,
        integer_type,
        "long long",
        IntegerType {
            unsigned: None,
            size: IntegerSize::LongLong(_),
        }
    );

    test_match!(
        unsigned_long,
        integer_type,
        "unsigned long",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::Long(_),
        }
    );

    test_match!(
        signed_long,
        integer_type,
        "long",
        IntegerType {
            unsigned: None,
            size: IntegerSize::Long(_)
        }
    );

    test_match!(
        unsigned_short,
        integer_type,
        "unsigned short",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::Short(_)
        }
    );

    test_match!(
        signed_short,
        integer_type,
        "short",
        IntegerType {
            unsigned: None,
            size: IntegerSize::Short(_)
        }
    );

    test_result_match!(
        unsigned_foo,
        integer_type,
        "unsigned foo",
        Err(nom::Err::Failure(_))
    );

    test_match!(
        unrestricted_float,
        float_type,
        "unrestricted float",
        FloatType {
            unrestricted: Some(_),
            size: FloatSize::Float(_)
        }
    );

    test_match!(
        float,
        float_type,
        "float",
        FloatType {
            unrestricted: None,
            size: FloatSize::Float(_)
        }
    );

    test_match!(
        unrestricted_double,
        float_type,
        "unrestricted double",
        FloatType {
            unrestricted: Some(_),
            size: FloatSize::Double(_)
        }
    );

    test_match!(
        double,
        float_type,
        "double",
        FloatType {
            unrestricted: None,
            size: FloatSize::Double(_)
        }
    );

    test_result_match!(
        unrestricted_foo,
        float_type,
        "unrestricted foo",
        Err(nom::Err::Failure(_))
    );
}

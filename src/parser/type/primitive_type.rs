// https://webidl.spec.whatwg.org/#prod-PrimitiveType

use nom::{IResult, Parser};

use crate::lexer::keywords;
use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerSize<'a> {
    LongLong(
        (
            VariantToken<'a, keywords::Long<'a>>,
            VariantToken<'a, keywords::Long<'a>>,
        ),
    ),
    Long(VariantToken<'a, keywords::Long<'a>>),
    Short(VariantToken<'a, keywords::Short<'a>>),
}

impl IntegerSize<'_> {
    pub fn parse<'slice, 'token>(
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
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IntegerType<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub size: IntegerSize<'a>,
}

impl IntegerType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>> {
        // TODO: use nom::error::VerboseErrorKind? how?
        nom::branch::alt((
            nom::sequence::tuple((eat_key!(Unsigned), nom::combinator::cut(IntegerSize::parse)))
                .map(|(unsigned, size)| IntegerType {
                    unsigned: Some(unsigned),
                    size,
                }),
            IntegerSize::parse.map(|size| IntegerType {
                unsigned: None,
                size,
            }),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatSize<'a> {
    Float(VariantToken<'a, keywords::Float<'a>>),
    Double(VariantToken<'a, keywords::Double<'a>>),
}

impl FloatSize<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, FloatSize<'token>> {
        nom::branch::alt((
            eat_key!(Float).map(FloatSize::Float),
            eat_key!(Double).map(FloatSize::Double),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FloatType<'a> {
    pub unrestricted: Option<VariantToken<'a, keywords::Unrestricted<'a>>>,
    pub size: FloatSize<'a>,
}

impl FloatType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, FloatType<'token>> {
        // TODO: use nom::error::VerboseErrorKind? how?
        nom::branch::alt((
            nom::sequence::tuple((
                eat_key!(Unrestricted),
                nom::combinator::cut(FloatSize::parse),
            ))
            .map(|(unrestricted, size)| FloatType {
                unrestricted: Some(unrestricted),
                size,
            }),
            FloatSize::parse.map(|size| FloatType {
                unrestricted: None,
                size,
            }),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PrimitiveType<'a> {
    Integer(IntegerType<'a>),
    Float(FloatType<'a>),
    Boolean(VariantToken<'a, keywords::Boolean<'a>>),
    Byte(VariantToken<'a, keywords::Byte<'a>>),
    Octet(VariantToken<'a, keywords::Octet<'a>>),
    Bigint(VariantToken<'a, keywords::Bigint<'a>>),
}

impl PrimitiveType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, PrimitiveType<'token>> {
        nom::branch::alt((
            IntegerType::parse.map(PrimitiveType::Integer),
            FloatType::parse.map(PrimitiveType::Float),
            nom::combinator::map(eat_key!(Boolean), PrimitiveType::Boolean),
            nom::combinator::map(eat_key!(Byte), PrimitiveType::Byte),
            nom::combinator::map(eat_key!(Octet), PrimitiveType::Octet),
            nom::combinator::map(eat_key!(Bigint), PrimitiveType::Bigint),
        ))(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        unsigned_long_long,
        IntegerType::parse,
        "unsigned long long",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::LongLong(_),
        }
    );

    test_match!(
        signed_long_long,
        IntegerType::parse,
        "long long",
        IntegerType {
            unsigned: None,
            size: IntegerSize::LongLong(_),
        }
    );

    test_match!(
        unsigned_long,
        IntegerType::parse,
        "unsigned long",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::Long(_),
        }
    );

    test_match!(
        signed_long,
        IntegerType::parse,
        "long",
        IntegerType {
            unsigned: None,
            size: IntegerSize::Long(_)
        }
    );

    test_match!(
        unsigned_short,
        IntegerType::parse,
        "unsigned short",
        IntegerType {
            unsigned: Some(_),
            size: IntegerSize::Short(_)
        }
    );

    test_match!(
        signed_short,
        IntegerType::parse,
        "short",
        IntegerType {
            unsigned: None,
            size: IntegerSize::Short(_)
        }
    );

    test_result_match!(
        unsigned_foo,
        IntegerType::parse,
        "unsigned foo",
        Err(nom::Err::Failure(_))
    );

    test_match!(
        unrestricted_float,
        FloatType::parse,
        "unrestricted float",
        FloatType {
            unrestricted: Some(_),
            size: FloatSize::Float(_)
        }
    );

    test_match!(
        float,
        FloatType::parse,
        "float",
        FloatType {
            unrestricted: None,
            size: FloatSize::Float(_)
        }
    );

    test_match!(
        unrestricted_double,
        FloatType::parse,
        "unrestricted double",
        FloatType {
            unrestricted: Some(_),
            size: FloatSize::Double(_)
        }
    );

    test_match!(
        double,
        FloatType::parse,
        "double",
        FloatType {
            unrestricted: None,
            size: FloatSize::Double(_)
        }
    );

    test_result_match!(
        unrestricted_foo,
        FloatType::parse,
        "unrestricted foo",
        Err(nom::Err::Failure(_))
    );

    test_match!(
        boolean,
        PrimitiveType::parse,
        "boolean",
        PrimitiveType::Boolean(_)
    );

    test_match!(byte, PrimitiveType::parse, "byte", PrimitiveType::Byte(_));

    test_match!(
        octet,
        PrimitiveType::parse,
        "octet",
        PrimitiveType::Octet(_)
    );

    test_match!(
        bigint,
        PrimitiveType::parse,
        "bigint",
        PrimitiveType::Bigint(_)
    );
}
// https://webidl.spec.whatwg.org/#prod-PrimitiveType

use nom::{IResult, Parser};

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::lexer::{keywords, Token};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongLong<'a> {
    pub long: VariantToken<'a, keywords::Long<'a>>,
    pub long_long: VariantToken<'a, keywords::Long<'a>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Long<'a>(pub VariantToken<'a, keywords::Long<'a>>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Short<'a>(pub VariantToken<'a, keywords::Short<'a>>);

pub enum IntegerSize<'a> {
    LongLong(LongLong<'a>),
    Long(Long<'a>),
    Short(Short<'a>),
}

pub struct IntegerType<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub size: IntegerSize<'a>,
}

fn integer_size<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IntegerSize<'token>> {
    let (tokens, short) = eat_key_optional!(Short)(tokens);
    if let Some(short) = short {
        return Ok((tokens, IntegerSize::Short(Short(short))));
    }

    let (tokens, long) = eat_key!(Long)(tokens)?;
    let (tokens, long_long) = eat_key_optional!(Long)(tokens);
    Ok((
        tokens,
        match long_long {
            Some(long_long) => IntegerSize::LongLong(LongLong { long, long_long }),
            _ => IntegerSize::Long(Long(long)),
        },
    ))
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
}

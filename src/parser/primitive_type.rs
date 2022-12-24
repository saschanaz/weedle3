// https://webidl.spec.whatwg.org/#prod-PrimitiveType

use nom::IResult;

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::lexer::{keywords, Token};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongLong<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub long: VariantToken<'a, keywords::Long<'a>>,
    pub long_long: VariantToken<'a, keywords::Long<'a>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Long<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub long: VariantToken<'a, keywords::Long<'a>>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Short<'a> {
    pub unsigned: Option<VariantToken<'a, keywords::Unsigned<'a>>>,
    pub short: VariantToken<'a, keywords::Short<'a>>,
}
pub enum IntegerType<'a> {
    LongLong(LongLong<'a>),
    Long(Long<'a>),
    Short(Short<'a>),
}

fn signed_integer_type<'slice, 'token>(
    unsigned: Option<VariantToken<'token, keywords::Unsigned<'token>>>,
) -> impl Fn(Tokens<'slice, 'token>) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>>
where
    'token: 'slice,
{
    move |tokens| {
        let (tokens, short) = eat_key_optional!(Short)(tokens);
        if let Some(short) = short {
            return Ok((tokens, IntegerType::Short(Short { unsigned, short })));
        }

        let (tokens, long) = eat_key!(Long)(tokens)?;
        let (tokens, long_long) = eat_key_optional!(Long)(tokens);
        Ok((
            tokens,
            match long_long {
                Some(long_long) => IntegerType::LongLong(LongLong {
                    unsigned,
                    long,
                    long_long,
                }),
                _ => IntegerType::Long(Long { unsigned, long }),
            },
        ))
    }
}

fn integer_type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>> {
    let (tokens, unsigned) = eat_key_optional!(Unsigned)(tokens);

    // TODO: use nom::error::VerboseErrorKind? how?
    nom::combinator::cut(signed_integer_type(unsigned))(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        unsigned_long_long,
        integer_type,
        "unsigned long long",
        IntegerType::LongLong(LongLong {
            unsigned: Some(_),
            ..
        })
    );

    test_match!(
        signed_long_long,
        integer_type,
        "long long",
        IntegerType::LongLong(LongLong { unsigned: None, .. })
    );

    test_match!(
        unsigned_long,
        integer_type,
        "unsigned long",
        IntegerType::Long(Long {
            unsigned: Some(_),
            ..
        })
    );

    test_match!(
        signed_long,
        integer_type,
        "long",
        IntegerType::Long(Long { unsigned: None, .. })
    );

    test_match!(
        unsigned_short,
        integer_type,
        "unsigned short",
        IntegerType::Short(Short {
            unsigned: Some(_),
            ..
        })
    );

    test_match!(
        signed_short,
        integer_type,
        "short",
        IntegerType::Short(Short { unsigned: None, .. })
    );

    test_result_match!(
        unsigned_foo,
        integer_type,
        "unsigned foo",
        Err(nom::Err::Failure(_))
    );
}

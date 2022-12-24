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
    tokens: Tokens<'slice, 'token>,
    unsigned: Option<VariantToken<'token, keywords::Unsigned<'token>>>,
) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>> {
    let (tokens, short) = eat_key_optional!(Short)(tokens);
    if let Some(short) = short {
        return Ok((tokens, IntegerType::Short(Short { unsigned, short })));
    }

    let (tokens, long) = eat_key!(Long)(tokens)?;
    println!("{tokens:?}");
    let (tokens, long_long) = eat_key_optional!(Long)(tokens);
    println!("{tokens:?}");
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

fn integer_type<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IntegerType<'token>> {
    let (tokens, unsigned) = eat_key_optional!(Unsigned)(tokens);

    println!("{tokens:?}");

    signed_integer_type(tokens, unsigned).map_err(|err| {
        match unsigned {
            Some(_) => nom::Err::Failure(nom::error::Error {
                input: tokens,
                // TODO: use nom::error::VerboseErrorKind?
                code: nom::error::ErrorKind::Fail,
            }),
            None => err,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer::{lex, Tag},
        parser::impl_nom_traits::Tokens,
    };

    #[test]
    fn unsigned_long_long() {
        let tokens = lex("unsigned long long").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::LongLong(LongLong {
                unsigned: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn signed_long_long() {
        let tokens = lex("long long").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::LongLong(LongLong { unsigned: None, .. })
        ));
    }

    #[test]
    fn unsigned_long() {
        let tokens = lex("unsigned long").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::Long(Long {
                unsigned: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn signed_long() {
        let tokens = lex("long").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::Long(Long { unsigned: None, .. })
        ));
    }

    #[test]
    fn unsigned_short() {
        let tokens = lex("unsigned short").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::Short(Short {
                unsigned: Some(_),
                ..
            })
        ));
    }

    #[test]
    fn signed_short() {
        let tokens = lex("short").unwrap();
        println!("{tokens:?}");
        let (unread, result) = integer_type(Tokens(&tokens[..])).unwrap();

        assert!(matches!(
            unread.0,
            [Token {
                tag: Tag::Eof(_),
                ..
            }]
        ));
        assert!(matches!(
            result,
            IntegerType::Short(Short { unsigned: None, .. })
        ));
    }

    #[test]
    fn unsigned_foo() {
        let tokens = lex("unsigned foo").unwrap();
        println!("{tokens:?}");
        let result = integer_type(Tokens(&tokens[..]));

        assert!(matches!(result, Err(nom::Err::Failure(_))));
    }
}

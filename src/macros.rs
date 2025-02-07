macro_rules! parser {
    ($parse:expr) => {
        fn parse_tokens<'slice>(
            input: $crate::tokens::Tokens<'slice, 'a>,
        ) -> $crate::VerboseResult<$crate::tokens::Tokens<'slice, 'a>, Self> {
            $parse(input)
        }
    };
}

macro_rules! lexer {
    ($lex:expr) => {
        pub fn lex(input: &'a str) -> $crate::VerboseResult<&'a str, Self> {
            $lex(input)
        }
    };
}

macro_rules! weedle {
    ($t:ty) => {
        <$t as $crate::Parse<'a>>::parse_tokens
    };
}

// nom::branch::alt supports at-most 21 parsers, increasing to infinity ones.
macro_rules! alt {
    ($member0:expr, $member1:expr, $($member2:expr,)+) => {
        nom::branch::alt(($member0, $member1, alt!($($member2,)+)))
    };
    ($member0:expr, $($member1:expr,)+) => {
        nom::branch::alt(($member0, alt!($($member1,)+)))
    };
    ($member0:expr,) => {
        $member0
    };
}

// XXX: Working around the lambda function limitation about lifetimes
// https://github.com/rust-lang/rust/issues/58052
pub fn annotate<'slice, 'token, F, R>(f: F) -> F
where
    F: Fn(
        crate::tokens::Tokens<'slice, 'token>,
    ) -> crate::VerboseResult<crate::tokens::Tokens<'slice, 'token>, R>,
    'token: 'slice,
{
    f
}

macro_rules! eat {
    ($variant:ident) => {
        $crate::macros::annotate(
            |input: $crate::tokens::Tokens| -> $crate::VerboseResult<$crate::tokens::Tokens, _> {
                use nom::{InputIter, Slice};
                match input.iter_elements().next() {
                    Some($crate::lexer::Token {
                        value: $crate::lexer::Terminal::$variant(variant),
                        trivia: _,
                    }) => Ok((input.slice(1..), variant)),
                    _ => nom::combinator::fail(input),
                }
            },
        )
    };
}

macro_rules! eat_key {
    ($variant:ident) => {
        $crate::macros::annotate(
            |input: $crate::tokens::Tokens| -> $crate::VerboseResult<$crate::tokens::Tokens, _> {
                use nom::{InputIter, Slice};
                use $crate::lexer::Terminal;
                use $crate::term::Keyword;
                match input.iter_elements().next() {
                    Some($crate::lexer::Token {
                        value: Terminal::Keyword(Keyword::$variant(variant)),
                        trivia: _,
                    }) => Ok((input.slice(1..), variant)),
                    _ => nom::combinator::fail(input),
                }
            },
        )
    };
}

macro_rules! try_eat_keys {
    ($typ:ident, $input:ident, $($variant:ident),+) => {
        $(
            if let Ok((tokens, result)) = eat_key!($variant)($input) {
                return Ok((tokens, $typ(result.value())));
            }
        )+
    };
}

#[cfg(test)]
macro_rules! test {
    (@arg $parsed:ident) => {};
    (@arg $parsed:ident $($lhs:tt).+ == $rhs:expr; $($rest:tt)*) => {
        assert_eq!($parsed.$($lhs).+, $rhs);
        test!(@arg $parsed $($rest)*);
    };
    (@arg $parsed:ident $($lhs:tt).+(); $($rest:tt)*) => {
        assert!($parsed.$($lhs).+());
        test!(@arg $parsed $($rest)*);
    };
    (@arg $parsed:ident $($lhs:tt).+() == $rhs:expr; $($rest:tt)*) => {
        assert_eq!($parsed.$($lhs).+(), $rhs);
        test!(@arg $parsed $($rest)*);
    };
    (err $name:ident { $raw:expr => $typ:ty }) => {
        #[test]
        fn $name() {
            <$typ>::parse($raw).unwrap_err();
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty => $val:expr }) => {
        #[test]
        fn $name() {
            let (rem, parsed) = <$typ>::parse($raw).unwrap();
            assert_eq!(rem, $rem);
            assert_eq!(parsed, $val);
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty; $($body:tt)* }) => {
        #[test]
        fn $name() {
            let (_rem, _parsed) = <$typ>::parse($raw).unwrap();
            assert_eq!(_rem, $rem);
            test!(@arg _parsed $($body)*);
        }
    };
}

#[cfg(test)]
macro_rules! test_variants {
    ($struct_:ident { $( $variant:ident == $value:expr ),* $(,)* }) => {
        #[allow(non_snake_case)]
        mod $struct_ {
            $(
                mod $variant {
                    use $crate::types::*;
                    #[test]
                    fn should_parse() {
                        let (rem, parsed) = $struct_::parse($value).unwrap();
                        assert_eq!(rem, "");
                        match parsed {
                            $struct_::$variant(_) => {},
                            _ => { panic!("Failed to parse"); }
                        }
                    }
                }
            )*
        }
    };
}

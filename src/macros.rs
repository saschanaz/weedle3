macro_rules! parser {
    ($parse:expr) => {
        fn parse(
            input: $crate::parser::Tokens<'slice, 'a>,
        ) -> $crate::IResult<$crate::parser::Tokens<'slice, 'a>, Self> {
            $parse(input)
        }
    };
}

macro_rules! weedle {
    ($t:ty) => {
        <$t as $crate::Parse<'slice, 'a>>::parse
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

#[cfg(test)]
macro_rules! test_match {
    ($name:ident { $input:literal => $rem:expr; $typ:ty => $match:pat_param $(if $guard:expr)? }) => {
        #[test]
        fn $name() {
            let (unread, result) = <$typ>::parse($input).unwrap();

            assert_eq!(unread, $rem);
            assert!(matches!(result, $match $(if $guard )?));
        }
    };
}

#[cfg(test)]
macro_rules! test_result_match {
    ($name:ident { $input:literal; $typ:ty => $match:pat_param }) => {
        #[test]
        fn $name() {
            let result = <$typ>::parse($input);

            assert!(matches!(result, $match));
        }
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
            let tokens = $crate::lexer::lex($raw).unwrap();
            <$typ>::parse($crate::parser::Tokens(&tokens[..])).unwrap_err();
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty => $val:expr }) => {
        #[test]
        fn $name() {
            let tokens = $crate::lexer::lex($raw).unwrap();
            let (rem, parsed) = <$typ>::parse($crate::parser::Tokens(&tokens[..])).unwrap();
            assert_eq!(unsafe { rem.remaining($raw) }, $rem);
            assert_eq!(parsed, $val);
        }
    };
    ($name:ident { $raw:expr => $rem:expr; $typ:ty; $($body:tt)* }) => {
        #[test]
        fn $name() {
            let tokens = $crate::lexer::lex($raw).unwrap();
            let (_rem, _parsed) = <$typ>::parse($crate::parser::Tokens(&tokens[..])).unwrap();
            assert_eq!(unsafe { _rem.remaining($raw) }, $rem);
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
                        let tokens = $crate::lexer::lex($value).unwrap();
                        let (rem, parsed) = $struct_::parse($crate::parser::Tokens(&tokens[..])).unwrap();
                        assert_eq!(unsafe { rem.remaining($value) }, "");
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

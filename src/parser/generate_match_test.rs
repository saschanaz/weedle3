#[cfg(test)]
macro_rules! test_match {
    ($name:ident, $func:ident, $input:literal, $match:pat_param) => {
        #[test]
        fn $name() {
            use crate::{
                lexer::{lex, Tag, Token},
                parser::impl_nom_traits::Tokens,
            };
            let tokens = lex($input).unwrap();
            let (unread, result) = $func(Tokens(&tokens[..])).unwrap();

            assert!(matches!(
                unread.0,
                [Token {
                    tag: Tag::Eof(_),
                    ..
                }]
            ));
            assert!(matches!(result, $match));
        }
    };
}

#[cfg(test)]
macro_rules! test_result_match {
    ($name:ident, $func:ident, $input:literal, $match:pat_param) => {
        #[test]
        fn $name() {
            use crate::{lexer::lex, parser::impl_nom_traits::Tokens};
            let tokens = lex($input).unwrap();
            let result = $func(Tokens(&tokens[..]));

            assert!(matches!(result, $match));
        }
    };
}

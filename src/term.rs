/*
 * The following will ultimate generate:
 *
 * ```rust
 * enum Keyword<'a> {
 *     OpenParen(OpenParen),
 *     CloseParen(CloseParen),
 *     Or(Or),
 *     Optional(Optional),
 *     /* ... */
 * }
 *
 * impl<'a> Keyword<'a> {
 *     pub fn parse_punc(input: &str) -> nom::IResult<&str, &str> {
 *         nom::branch::alt((
 *             nom::combinator::map(
 *                 nom::combinator::recognize(nom::bytes::complete::tag("(")),
 *                 Keyword::OpenParen,
 *             ),
 *             nom::combinator::map(
 *                 nom::combinator::recognize(nom::bytes::complete::tag(")")),
 *                 Keyword::CloseParen,
 *             ),
 *             /* ... */
 *         ))(input)
 *     }
 *     pub fn match_word(input: &str) -> Option<Keyword> {
 *         match input {
 *             "or" => Some(Keyword::Or(Or)),
 *             "optional" => Some(Keyword::Optional(Optional)),
 *             /* ... */
 *             _ => None
 *         }
 *     }
 * }
 * ```
 *
 * Use `cargo-expand` to see the full macro expansion.
 */

#[cfg(test)]
macro_rules! generate_tests {
    ($typ:ident, $tok:expr) => {
        #[allow(non_snake_case)]
        #[cfg(test)]
        mod $typ {
            use super::super::{$typ, Keyword};
            use $crate::lexer::{lex, Tag};

            #[test]
            fn should_parse() {
                let tokens = lex($tok).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].tag, Tag::Kw(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_preceding_spaces() {
                let tokens = lex(concat!("  ", $tok)).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].tag, Tag::Kw(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_succeeding_spaces() {
                let tokens = lex(concat!($tok, "  ")).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].tag, Tag::Kw(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_surrounding_spaces() {
                let tokens = lex(concat!("  ", $tok, "  ")).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].tag, Tag::Kw(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_if_anything_next() {
                let tokens = lex(concat!($tok, "  anything")).unwrap();
                assert_eq!(tokens.len(), 3);
                assert_eq!(tokens[0].tag, Tag::Kw(Keyword::$typ($typ)));
            }
        }
    };
}

macro_rules! generate_keyword_struct {
    ($(#[$attr:meta])* $typ:ident => $tok:expr) => {
        $(#[$attr])*
        #[derive(Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub struct $typ;

        impl $typ {
            pub fn value(&self) -> &'static str {
                return $tok;
            }
        }

        impl<'a> $crate::Parse<'a> for $typ {
            parser!($crate::eat_key!($typ));
        }
    };
}

macro_rules! generate_keywords_enum {
    (
        $( #[$attr:meta] $typ_punc:ident => $tok_punc:expr, )* === $( $typ_word:ident => $tok_word:expr, )*
    ) => {
        $(generate_keyword_struct!(
            #[$attr]
            $typ_punc => $tok_punc
        );)*

        $(generate_keyword_struct!(
            #[doc = "Represents the terminal symbol `"]
            #[doc = $tok_word]
            #[doc = "`"]
            $typ_word => $tok_word
        );)*

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Keyword {
            $(
                #[$attr]
                $typ_punc($typ_punc),
            )*
            $(
                #[doc = "Represents the terminal symbol `"]
                #[doc = $tok_word]
                #[doc = "`"]
                $typ_word($typ_word),
            )*
        }

        impl Keyword {
            pub fn parse_punc(input: &str) -> nom::IResult<&str, Keyword> {
                alt!(
                    $(nom::combinator::map(
                        nom::combinator::recognize(nom::bytes::complete::tag($tok_punc)),
                        |_| Keyword::$typ_punc($typ_punc)
                    ),)*
                )(input)
            }

            pub fn match_word(input: &str) -> Option<Keyword> {
                match input {
                    $($tok_word => Some(Keyword::$typ_word($typ_word)),)*
                    _ => None,
                }
            }
        }

        #[cfg(test)]
        mod test {
            $( generate_tests!($typ_punc, $tok_punc); )*
            $( generate_tests!($typ_word, $tok_word); )*
        }
    };
}

generate_keywords_enum! {
    /// Represents the terminal symbol `(`
    OpenParen => "(",

    /// Represents the terminal symbol `)`
    CloseParen => ")",

    /// Represents the terminal symbol `[`
    OpenBracket => "[",

    /// Represents the terminal symbol `]`
    CloseBracket => "]",

    /// Represents the terminal symbol `{`
    OpenBrace => "{",

    /// Represents the terminal symbol `}`
    CloseBrace => "}",

    /// Represents the terminal symbol `,`
    Comma => ",",

    /// Represents the terminal symbol `-`
    Minus => "-",

    /// Represents the terminal symbol `...`
    Ellipsis => "...",

    /// Represents the terminal symbol `.`
    Dot => ".",

    /// Represents the terminal symbol `:`
    Colon => ":",

    /// Represents the terminal symbol `;`
    SemiColon => ";",

    /// Represents the terminal symbol `<`
    LessThan => "<",

    /// Represents the terminal symbol `=`
    Assign => "=",

    /// Represents the terminal symbol `>`
    GreaterThan => ">",

    /// Represents the terminal symbol `?`
    QMark => "?",

    /// Represents the terminal symbol `*`
    Wildcard => "*",

    ===

    Or => "or",
    Optional => "optional",
    Async => "async",
    Attribute => "attribute",
    Callback => "callback",
    Const => "const",
    Deleter => "deleter",
    Dictionary => "dictionary",
    Enum => "enum",
    Getter => "getter",
    Includes => "includes",
    Inherit => "inherit",
    Interface => "interface",
    Iterable => "iterable",
    Maplike => "maplike",
    Namespace => "namespace",
    Partial => "partial",
    Required => "required",
    Setlike => "setlike",
    Setter => "setter",
    Static => "static",
    Stringifier => "stringifier",
    Typedef => "typedef",
    Unrestricted => "unrestricted",
    Symbol => "symbol",
    NegInfinity => "-Infinity",
    ByteString => "ByteString",
    DOMString => "DOMString",
    FrozenArray => "FrozenArray",
    Infinity => "Infinity",
    NaN => "NaN",
    ObservableArray => "ObservableArray",
    USVString => "USVString",
    Any => "any",
    Bigint => "bigint",
    Boolean => "boolean",
    Byte => "byte",
    Double => "double",
    False => "false",
    Float => "float",
    Long => "long",
    Null => "null",
    Object => "object",
    Octet => "octet",
    Sequence => "sequence",
    Short => "short",
    True => "true",
    Unsigned => "unsigned",
    Undefined => "undefined",
    Record => "record",
    ArrayBuffer => "ArrayBuffer",
    DataView => "DataView",
    Int8Array => "Int8Array",
    Int16Array => "Int16Array",
    Int32Array => "Int32Array",
    Uint8Array => "Uint8Array",
    Uint16Array => "Uint16Array",
    Uint32Array => "Uint32Array",
    Uint8ClampedArray => "Uint8ClampedArray",
    BigInt64Array => "BigInt64Array",
    BigUint64Array => "BigUint64Array",
    Float32Array => "Float32Array",
    Float64Array => "Float64Array",
    Promise => "Promise",
    ReadOnly => "readonly",
    Mixin => "mixin",
    Constructor => "constructor",
}

#[macro_export]
macro_rules! term {
    (OpenParen) => {
        $crate::term::OpenParen
    };
    (CloseParen) => {
        $crate::term::CloseParen
    };
    (OpenBracket) => {
        $crate::term::OpenBracket
    };
    (CloseBracket) => {
        $crate::term::CloseBracket
    };
    (OpenBrace) => {
        $crate::term::OpenBrace
    };
    (CloseBrace) => {
        $crate::term::CloseBrace
    };
    (,) => {
        $crate::term::Comma
    };
    (-) => {
        $crate::term::Minus
    };
    (.) => {
        $crate::term::Dot
    };
    (...) => {
        $crate::term::Ellipsis
    };
    (:) => {
        $crate::term::Colon
    };
    (;) => {
        $crate::term::SemiColon
    };
    (<) => {
        $crate::term::LessThan
    };
    (=) => {
        $crate::term::Assign
    };
    (>) => {
        $crate::term::GreaterThan
    };
    (?) => {
        $crate::term::QMark
    };
    (*) => {
        $crate::term::Wildcard
    };
    (or) => {
        $crate::term::Or
    };
    (optional) => {
        $crate::term::Optional
    };
    (async) => {
        $crate::term::Async
    };
    (attribute) => {
        $crate::term::Attribute
    };
    (callback) => {
        $crate::term::Callback
    };
    (const) => {
        $crate::term::Const
    };
    (deleter) => {
        $crate::term::Deleter
    };
    (dictionary) => {
        $crate::term::Dictionary
    };
    (enum) => {
        $crate::term::Enum
    };
    (getter) => {
        $crate::term::Getter
    };
    (includes) => {
        $crate::term::Includes
    };
    (inherit) => {
        $crate::term::Inherit
    };
    (interface) => {
        $crate::term::Interface
    };
    (iterable) => {
        $crate::term::Iterable
    };
    (maplike) => {
        $crate::term::Maplike
    };
    (namespace) => {
        $crate::term::Namespace
    };
    (partial) => {
        $crate::term::Partial
    };
    (required) => {
        $crate::term::Required
    };
    (setlike) => {
        $crate::term::Setlike
    };
    (setter) => {
        $crate::term::Setter
    };
    (static) => {
        $crate::term::Static
    };
    (stringifier) => {
        $crate::term::Stringifier
    };
    (typedef) => {
        $crate::term::Typedef
    };
    (unrestricted) => {
        $crate::term::Unrestricted
    };
    (symbol) => {
        $crate::term::Symbol
    };
    (- Infinity) => {
        $crate::term::NegInfinity
    };
    (ByteString) => {
        $crate::term::ByteString
    };
    (DOMString) => {
        $crate::term::DOMString
    };
    (FrozenArray) => {
        $crate::term::FrozenArray
    };
    (Infinity) => {
        $crate::term::Infinity
    };
    (NaN) => {
        $crate::term::NaN
    };
    (ObservableArray) => {
        $crate::term::ObservableArray
    };
    (USVString) => {
        $crate::term::USVString
    };
    (any) => {
        $crate::term::Any
    };
    (bigint) => {
        $crate::term::Bigint
    };
    (boolean) => {
        $crate::term::Boolean
    };
    (byte) => {
        $crate::term::Byte
    };
    (double) => {
        $crate::term::Double
    };
    (false) => {
        $crate::term::False
    };
    (float) => {
        $crate::term::Float
    };
    (long) => {
        $crate::term::Long
    };
    (null) => {
        $crate::term::Null
    };
    (object) => {
        $crate::term::Object
    };
    (octet) => {
        $crate::term::Octet
    };
    (sequence) => {
        $crate::term::Sequence
    };
    (short) => {
        $crate::term::Short
    };
    (true) => {
        $crate::term::True
    };
    (unsigned) => {
        $crate::term::Unsigned
    };
    (undefined) => {
        $crate::term::Undefined
    };
    (record) => {
        $crate::term::Record
    };
    (ArrayBuffer) => {
        $crate::term::ArrayBuffer
    };
    (DataView) => {
        $crate::term::DataView
    };
    (Int8Array) => {
        $crate::term::Int8Array
    };
    (Int16Array) => {
        $crate::term::Int16Array
    };
    (Int32Array) => {
        $crate::term::Int32Array
    };
    (Uint8Array) => {
        $crate::term::Uint8Array
    };
    (Uint16Array) => {
        $crate::term::Uint16Array
    };
    (Uint32Array) => {
        $crate::term::Uint32Array
    };
    (Uint8ClampedArray) => {
        $crate::term::Uint8ClampedArray
    };
    (BigInt64Array) => {
        $crate::term::BigInt64Array
    };
    (BigUint64Array) => {
        $crate::term::BigUint64Array
    };
    (Float32Array) => {
        $crate::term::Float32Array
    };
    (Float64Array) => {
        $crate::term::Float64Array
    };
    (Promise) => {
        $crate::term::Promise
    };
    (readonly) => {
        $crate::term::ReadOnly
    };
    (mixin) => {
        $crate::term::Mixin
    };
    (constructor) => {
        $crate::term::Constructor
    };
}

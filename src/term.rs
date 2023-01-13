macro_rules! generate_tests {
    ($typ:ident, $tok:expr) => {
        paste::paste! {
            #[cfg(test)]
            mod [<test_ $typ:lower>] {
                use super::$typ;
                use crate::Parse;

                #[test]
                fn should_parse() {
                    let (rem, parsed) = $typ::parse(concat!($tok)).unwrap();
                    assert_eq!(rem, "");
                    assert_eq!(parsed, $typ);
                }

                #[test]
                fn should_parse_with_preceding_spaces() {
                    let (rem, parsed) = $typ::parse(concat!("  ", $tok)).unwrap();
                    assert_eq!(rem, "");
                    assert_eq!(parsed, $typ);
                }

                #[test]
                fn should_parse_with_succeeding_spaces() {
                    let (rem, parsed) = $typ::parse(concat!($tok, "  ")).unwrap();
                    assert_eq!(rem, "");
                    assert_eq!(parsed, $typ);
                }

                #[test]
                fn should_parse_with_surrounding_spaces() {
                    let (rem, parsed) = $typ::parse(concat!("  ", $tok, "  ")).unwrap();
                    assert_eq!(rem, "");
                    assert_eq!(parsed, $typ);
                }

                #[test]
                fn should_parse_if_anything_next() {
                    let (rem, parsed) = $typ::parse(concat!($tok, "  anything")).unwrap();
                    assert_eq!(rem, "anything");
                    assert_eq!(parsed, $typ);
                }
            }
        }
    };
}

macro_rules! generate_terms {
    ($( $(#[$attr:meta])* $typ:ident => $tok:expr, )*) => {
        $(
            $(#[$attr])*
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $typ;

            impl<'a> $crate::Parse<'a> for $typ {
                parser!(nom::combinator::value(
                    $typ,
                    crate::whitespace::ws(
                        nom::bytes::complete::tag($tok)
                    )
                ));
            }

            generate_tests!($typ, $tok);
        )*
    };
}

struct AlphaNumUnderscoreDash;

impl nom::FindToken<char> for AlphaNumUnderscoreDash {
    fn find_token(&self, token: char) -> bool {
        crate::common::is_alphanum_underscore_dash(token)
    }
}

pub(crate) fn ident_tag(tag: &'static str) -> impl FnMut(&str) -> nom::IResult<&str, &str> {
    move |input| {
        nom::sequence::terminated(
            nom::bytes::complete::tag(tag),
            nom::combinator::not(nom::combinator::map_parser(
                nom::bytes::complete::take(1usize),
                nom::bytes::complete::is_a(AlphaNumUnderscoreDash),
            )),
        )(input)
    }
}

macro_rules! generate_terms_for_names {
    ($($typ:ident => $tok:expr,)*) => {
        $(
            #[doc = "Represents the terminal symbol `"]
            #[doc = $tok]
            #[doc = "`"]
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $typ;

            impl<'a> $crate::Parse<'a> for $typ {
                parser!(nom::combinator::value(
                    $typ,
                    $crate::whitespace::ws($crate::term::ident_tag($tok))
                ));
            }

            generate_tests!($typ, $tok);
        )*
    };
}

generate_terms! {
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

    /// Represents the terminal symbol `.`
    Dot => ".",

    /// Represents the terminal symbol `...`
    Ellipsis => "...",

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
}

generate_terms_for_names! {
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
    (boolean) => {
        $crate::term::Boolean
    };
    (byte) => {
        $crate::term::Byte
    };
    (bigint) => {
        $crate::term::Bigint
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

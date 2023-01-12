/*
 * The following will ultimate generate:
 *
 * ```
 * enum Keyword<'a> {
 *     OpenParen(&'a str),
 *     CloseParen(&'a str),
 *     /* ... */
 * }
 * impl<'a> Keyword<'a> {
 *     pub fn parse(input: &str) -> nom::IResult<&str, &str> {
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
 * }
 * ```
 */

macro_rules! generate_keywords_enum {
    (
        $($typ:ident => $tok:expr,)*
    ) => {
        $(
            #[doc=$tok]
            #[derive(Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $typ;

            impl $typ {
                pub fn value(&self) -> &'static str {
                    return $tok;
                }
            }

            impl<'slice, 'a> $crate::Parse<'slice, 'a> for $typ {
                parser!($crate::eat_key!($typ));
            }
        )*

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Keyword {
            $(
                #[doc=$tok]
                $typ($typ),
            )*
        }

        impl Keyword {
            pub fn parse(input: &str) -> nom::IResult<&str, Keyword> {
                alt!(
                    $(nom::combinator::map(
                        nom::combinator::recognize(nom::bytes::complete::tag($tok)),
                        |_| Keyword::$typ($typ)
                    ),)*
                )(input)
            }
        }
    };
}

generate_keywords_enum!(
    Or => "or",
    Optional => "optional",
    Async => "async",
    Attribute => "attribute",
    Callback => "callback",
    Constructor => "constructor",
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
    OpenParen => "(",
    CloseParen => ")",
    OpenBracket => "[",
    CloseBracket => "]",
    OpenBrace => "{",
    CloseBrace => "}",
    Comma => ",",
    Minus => "-",
    Ellipsis => "...",
    Dot => ".",
    Colon => ":",
    SemiColon => ";",
    LessThan => "<",
    Assign => "=",
    GreaterThan => ">",
    QMark => "?",
    Wildcard => "*",
);

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
    (bigint) => {
        $crate::term::Bigint
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

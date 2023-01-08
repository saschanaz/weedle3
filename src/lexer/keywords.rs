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

use crate::parser::Tokens;
use nom::IResult;

macro_rules! generate_keywords_enum {
    (
        $($typ:ident => $tok:expr,)*
    ) => {
        $(
            #[doc=$tok]
            #[derive(Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $typ;

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

// One group can have at most 21 items, see:
// https://docs.rs/nom/latest/nom/branch/fn.alt.html
generate_keywords_enum!(
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
    Any => "any",
    Object => "object",
    Symbol => "symbol",
    ByteString => "ByteString",
    DOMString => "DOMString",
    USVString => "USVString",
    Async => "async",
    Attribute => "attribute",
    Callback => "callback",
    Constructor => "constructor",
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
    Or => "or",
    Optional => "optional",
    Const => "const",
    NegInfinity => "-Infinity",
    FrozenArray => "FrozenArray",
    Infinity => "Infinity",
    NaN => "NaN",
    ObservableArray => "ObservableArray",
    Bigint => "bigint",
    Boolean => "boolean",
    Byte => "byte",
    Double => "double",
    False => "false",
    Float => "float",
    Long => "long",
    Null => "null",
    Octet => "octet",
    Sequence => "sequence",
    Short => "short",
    True => "true",
    Unsigned => "unsigned",
    Undefined => "undefined",
    Record => "record",
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
    QuestionMark => "?",
    Wildcard => "*",
);

#[macro_export]
macro_rules! term {
    (OpenParen) => {
        $crate::lexer::keywords::OpenParen
    };
    (CloseParen) => {
        $crate::lexer::keywords::CloseParen
    };
    (OpenBracket) => {
        $crate::lexer::keywords::OpenBracket
    };
    (CloseBracket) => {
        $crate::lexer::keywords::CloseBracket
    };
    (OpenBrace) => {
        $crate::lexer::keywords::OpenBrace
    };
    (CloseBrace) => {
        $crate::lexer::keywords::CloseBrace
    };
    (,) => {
        $crate::lexer::keywords::Comma
    };
    (-) => {
        $crate::lexer::keywords::Minus
    };
    (.) => {
        $crate::lexer::keywords::Dot
    };
    (...) => {
        $crate::lexer::keywords::Ellipsis
    };
    (:) => {
        $crate::lexer::keywords::Colon
    };
    (;) => {
        $crate::lexer::keywords::SemiColon
    };
    (<) => {
        $crate::lexer::keywords::LessThan
    };
    (=) => {
        $crate::lexer::keywords::Assign
    };
    (>) => {
        $crate::lexer::keywords::GreaterThan
    };
    (?) => {
        $crate::lexer::keywords::QMark
    };
    (*) => {
        $crate::lexer::keywords::Wildcard
    };
    (or) => {
        $crate::lexer::keywords::Or
    };
    (optional) => {
        $crate::lexer::keywords::Optional
    };
    (async) => {
        $crate::lexer::keywords::Async
    };
    (attribute) => {
        $crate::lexer::keywords::Attribute
    };
    (callback) => {
        $crate::lexer::keywords::Callback
    };
    (const) => {
        $crate::lexer::keywords::Const
    };
    (deleter) => {
        $crate::lexer::keywords::Deleter
    };
    (dictionary) => {
        $crate::lexer::keywords::Dictionary
    };
    (enum) => {
        $crate::lexer::keywords::Enum
    };
    (getter) => {
        $crate::lexer::keywords::Getter
    };
    (includes) => {
        $crate::lexer::keywords::Includes
    };
    (inherit) => {
        $crate::lexer::keywords::Inherit
    };
    (interface) => {
        $crate::lexer::keywords::Interface
    };
    (iterable) => {
        $crate::lexer::keywords::Iterable
    };
    (maplike) => {
        $crate::lexer::keywords::Maplike
    };
    (namespace) => {
        $crate::lexer::keywords::Namespace
    };
    (partial) => {
        $crate::lexer::keywords::Partial
    };
    (required) => {
        $crate::lexer::keywords::Required
    };
    (setlike) => {
        $crate::lexer::keywords::Setlike
    };
    (setter) => {
        $crate::lexer::keywords::Setter
    };
    (static) => {
        $crate::lexer::keywords::Static
    };
    (stringifier) => {
        $crate::lexer::keywords::Stringifier
    };
    (typedef) => {
        $crate::lexer::keywords::Typedef
    };
    (unrestricted) => {
        $crate::lexer::keywords::Unrestricted
    };
    (symbol) => {
        $crate::lexer::keywords::Symbol
    };
    (- Infinity) => {
        $crate::lexer::keywords::NegInfinity
    };
    (ByteString) => {
        $crate::lexer::keywords::ByteString
    };
    (DOMString) => {
        $crate::lexer::keywords::DOMString
    };
    (FrozenArray) => {
        $crate::lexer::keywords::FrozenArray
    };
    (Infinity) => {
        $crate::lexer::keywords::Infinity
    };
    (NaN) => {
        $crate::lexer::keywords::NaN
    };
    (ObservableArray) => {
        $crate::lexer::keywords::ObservableArray
    };
    (USVString) => {
        $crate::lexer::keywords::USVString
    };
    (any) => {
        $crate::lexer::keywords::Any
    };
    (boolean) => {
        $crate::lexer::keywords::Boolean
    };
    (byte) => {
        $crate::lexer::keywords::Byte
    };
    (double) => {
        $crate::lexer::keywords::Double
    };
    (false) => {
        $crate::lexer::keywords::False
    };
    (float) => {
        $crate::lexer::keywords::Float
    };
    (long) => {
        $crate::lexer::keywords::Long
    };
    (null) => {
        $crate::lexer::keywords::Null
    };
    (object) => {
        $crate::lexer::keywords::Object
    };
    (octet) => {
        $crate::lexer::keywords::Octet
    };
    (bigint) => {
        $crate::lexer::keywords::Bigint
    };
    (sequence) => {
        $crate::lexer::keywords::Sequence
    };
    (short) => {
        $crate::lexer::keywords::Short
    };
    (true) => {
        $crate::lexer::keywords::True
    };
    (unsigned) => {
        $crate::lexer::keywords::Unsigned
    };
    (undefined) => {
        $crate::lexer::keywords::Undefined
    };
    (record) => {
        $crate::lexer::keywords::Record
    };
    (ArrayBuffer) => {
        $crate::lexer::keywords::ArrayBuffer
    };
    (DataView) => {
        $crate::lexer::keywords::DataView
    };
    (Int8Array) => {
        $crate::lexer::keywords::Int8Array
    };
    (Int16Array) => {
        $crate::lexer::keywords::Int16Array
    };
    (Int32Array) => {
        $crate::lexer::keywords::Int32Array
    };
    (Uint8Array) => {
        $crate::lexer::keywords::Uint8Array
    };
    (Uint16Array) => {
        $crate::lexer::keywords::Uint16Array
    };
    (Uint32Array) => {
        $crate::lexer::keywords::Uint32Array
    };
    (Uint8ClampedArray) => {
        $crate::lexer::keywords::Uint8ClampedArray
    };
    (BigInt64Array) => {
        $crate::lexer::keywords::BigInt64Array
    };
    (BigUint64Array) => {
        $crate::lexer::keywords::BigUint64Array
    };
    (Float32Array) => {
        $crate::lexer::keywords::Float32Array
    };
    (Float64Array) => {
        $crate::lexer::keywords::Float64Array
    };
    (Promise) => {
        $crate::lexer::keywords::Promise
    };
    (readonly) => {
        $crate::lexer::keywords::ReadOnly
    };
    (mixin) => {
        $crate::lexer::keywords::Mixin
    };
    (constructor) => {
        $crate::lexer::keywords::Constructor
    };
}

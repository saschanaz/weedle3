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
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $typ<'a>(pub &'a str);

            impl<'slice, 'a> $crate::Parse<'slice, 'a> for $typ<'a> {
                parser!($crate::eat_key!($typ));
            }

            impl<'a> Default for $typ<'a> {
                fn default() -> Self {
                    crate::lexer::keywords::$typ($tok)
                }
            }
        )*

        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Keyword<'a> {
            $(
                #[doc=$tok]
                $typ($typ<'a>),
            )*
        }

        impl<'a> Keyword<'a> {
            pub fn parse(input: &str) -> nom::IResult<&str, Keyword> {
                alt!(
                    $(nom::combinator::map(
                        nom::combinator::recognize(nom::bytes::complete::tag($tok)),
                        |k| Keyword::$typ($typ(k))
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
        $crate::lexer::keywords::OpenParen<'a>
    };
    (CloseParen) => {
        $crate::lexer::keywords::CloseParen<'a>
    };
    (OpenBracket) => {
        $crate::lexer::keywords::OpenBracket<'a>
    };
    (CloseBracket) => {
        $crate::lexer::keywords::CloseBracket<'a>
    };
    (OpenBrace) => {
        $crate::lexer::keywords::OpenBrace<'a>
    };
    (CloseBrace) => {
        $crate::lexer::keywords::CloseBrace<'a>
    };
    (,) => {
        $crate::lexer::keywords::Comma<'a>
    };
    (-) => {
        $crate::lexer::keywords::Minus<'a>
    };
    (.) => {
        $crate::lexer::keywords::Dot<'a>
    };
    (...) => {
        $crate::lexer::keywords::Ellipsis<'a>
    };
    (:) => {
        $crate::lexer::keywords::Colon<'a>
    };
    (;) => {
        $crate::lexer::keywords::SemiColon<'a>
    };
    (<) => {
        $crate::lexer::keywords::LessThan<'a>
    };
    (=) => {
        $crate::lexer::keywords::Assign<'a>
    };
    (>) => {
        $crate::lexer::keywords::GreaterThan<'a>
    };
    (?) => {
        $crate::lexer::keywords::QMark<'a>
    };
    (*) => {
        $crate::lexer::keywords::Wildcard<'a>
    };
    (or) => {
        $crate::lexer::keywords::Or<'a>
    };
    (optional) => {
        $crate::lexer::keywords::Optional<'a>
    };
    (async) => {
        $crate::lexer::keywords::Async<'a>
    };
    (attribute) => {
        $crate::lexer::keywords::Attribute<'a>
    };
    (callback) => {
        $crate::lexer::keywords::Callback<'a>
    };
    (const) => {
        $crate::lexer::keywords::Const<'a>
    };
    (deleter) => {
        $crate::lexer::keywords::Deleter<'a>
    };
    (dictionary) => {
        $crate::lexer::keywords::Dictionary<'a>
    };
    (enum) => {
        $crate::lexer::keywords::Enum<'a>
    };
    (getter) => {
        $crate::lexer::keywords::Getter<'a>
    };
    (includes) => {
        $crate::lexer::keywords::Includes<'a>
    };
    (inherit) => {
        $crate::lexer::keywords::Inherit<'a>
    };
    (interface) => {
        $crate::lexer::keywords::Interface<'a>
    };
    (iterable) => {
        $crate::lexer::keywords::Iterable<'a>
    };
    (maplike) => {
        $crate::lexer::keywords::Maplike<'a>
    };
    (namespace) => {
        $crate::lexer::keywords::Namespace<'a>
    };
    (partial) => {
        $crate::lexer::keywords::Partial<'a>
    };
    (required) => {
        $crate::lexer::keywords::Required<'a>
    };
    (setlike) => {
        $crate::lexer::keywords::Setlike<'a>
    };
    (setter) => {
        $crate::lexer::keywords::Setter<'a>
    };
    (static) => {
        $crate::lexer::keywords::Static<'a>
    };
    (stringifier) => {
        $crate::lexer::keywords::Stringifier<'a>
    };
    (typedef) => {
        $crate::lexer::keywords::Typedef<'a>
    };
    (unrestricted) => {
        $crate::lexer::keywords::Unrestricted<'a>
    };
    (symbol) => {
        $crate::lexer::keywords::Symbol<'a>
    };
    (- Infinity) => {
        $crate::lexer::keywords::NegInfinity<'a>
    };
    (ByteString) => {
        $crate::lexer::keywords::ByteString<'a>
    };
    (DOMString) => {
        $crate::lexer::keywords::DOMString<'a>
    };
    (FrozenArray) => {
        $crate::lexer::keywords::FrozenArray<'a>
    };
    (Infinity) => {
        $crate::lexer::keywords::Infinity<'a>
    };
    (NaN) => {
        $crate::lexer::keywords::NaN<'a>
    };
    (ObservableArray) => {
        $crate::lexer::keywords::ObservableArray<'a>
    };
    (USVString) => {
        $crate::lexer::keywords::USVString<'a>
    };
    (any) => {
        $crate::lexer::keywords::Any<'a>
    };
    (boolean) => {
        $crate::lexer::keywords::Boolean<'a>
    };
    (byte) => {
        $crate::lexer::keywords::Byte<'a>
    };
    (double) => {
        $crate::lexer::keywords::Double<'a>
    };
    (false) => {
        $crate::lexer::keywords::False<'a>
    };
    (float) => {
        $crate::lexer::keywords::Float<'a>
    };
    (long) => {
        $crate::lexer::keywords::Long<'a>
    };
    (null) => {
        $crate::lexer::keywords::Null<'a>
    };
    (object) => {
        $crate::lexer::keywords::Object<'a>
    };
    (octet) => {
        $crate::lexer::keywords::Octet<'a>
    };
    (bigint) => {
        $crate::lexer::keywords::Bigint<'a>
    };
    (sequence) => {
        $crate::lexer::keywords::Sequence<'a>
    };
    (short) => {
        $crate::lexer::keywords::Short<'a>
    };
    (true) => {
        $crate::lexer::keywords::True<'a>
    };
    (unsigned) => {
        $crate::lexer::keywords::Unsigned<'a>
    };
    (undefined) => {
        $crate::lexer::keywords::Undefined<'a>
    };
    (record) => {
        $crate::lexer::keywords::Record<'a>
    };
    (ArrayBuffer) => {
        $crate::lexer::keywords::ArrayBuffer<'a>
    };
    (DataView) => {
        $crate::lexer::keywords::DataView<'a>
    };
    (Int8Array) => {
        $crate::lexer::keywords::Int8Array<'a>
    };
    (Int16Array) => {
        $crate::lexer::keywords::Int16Array<'a>
    };
    (Int32Array) => {
        $crate::lexer::keywords::Int32Array<'a>
    };
    (Uint8Array) => {
        $crate::lexer::keywords::Uint8Array<'a>
    };
    (Uint16Array) => {
        $crate::lexer::keywords::Uint16Array<'a>
    };
    (Uint32Array) => {
        $crate::lexer::keywords::Uint32Array<'a>
    };
    (Uint8ClampedArray) => {
        $crate::lexer::keywords::Uint8ClampedArray<'a>
    };
    (BigInt64Array) => {
        $crate::lexer::keywords::BigInt64Array<'a>
    };
    (BigUint64Array) => {
        $crate::lexer::keywords::BigUint64Array<'a>
    };
    (Float32Array) => {
        $crate::lexer::keywords::Float32Array<'a>
    };
    (Float64Array) => {
        $crate::lexer::keywords::Float64Array<'a>
    };
    (Promise) => {
        $crate::lexer::keywords::Promise<'a>
    };
    (readonly) => {
        $crate::lexer::keywords::ReadOnly<'a>
    };
    (mixin) => {
        $crate::lexer::keywords::Mixin<'a>
    };
    (constructor) => {
        $crate::lexer::keywords::Constructor<'a>
    };
}

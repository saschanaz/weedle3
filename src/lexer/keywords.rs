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

use crate::parser::{eat::VariantToken, Tokens};
use nom::IResult;

macro_rules! generate_keywords_enum {
    (
        $($typ:ident => $tok:expr,)*
    ) => {
        $(
            #[doc=$tok]
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $typ<'a>(pub &'a str);

            impl<'a> $crate::Parse<'a> for VariantToken<'a, $typ<'a>> {
                parser!(|input: &'a str| {
                    let (i, token) = $crate::lexer::lex_single(input)?;
                    let array = [token];
                    let tokens = Tokens(&array);
                    match $crate::eat_key!($typ)(tokens) {
                        Ok((_, token)) => Ok((i, token)),
                        Err(_) => Err(nom::Err::Error(nom::error::Error {
                            input: i,
                            code: nom::error::ErrorKind::Char,
                        }))
                    }
                });
            }

            impl<'a> Default for $typ<'a> {
                fn default() -> Self {
                    crate::lexer::keywords::$typ($tok)
                }
            }

            impl<'a> Default for VariantToken<'a, $typ<'a>> {
                fn default() -> Self {
                    VariantToken {
                        variant: crate::lexer::keywords::$typ::default(),
                        trivia: "",
                    }
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
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::OpenParen<'a>>
    };
    (CloseParen) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::CloseParen<'a>>
    };
    (OpenBracket) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::OpenBracket<'a>>
    };
    (CloseBracket) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::CloseBracket<'a>>
    };
    (OpenBrace) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::OpenBrace<'a>>
    };
    (CloseBrace) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::CloseBrace<'a>>
    };
    (,) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Comma<'a>>
    };
    (-) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Minus<'a>>
    };
    (.) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Dot<'a>>
    };
    (...) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Ellipsis<'a>>
    };
    (:) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Colon<'a>>
    };
    (;) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::SemiColon<'a>>
    };
    (<) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::LessThan<'a>>
    };
    (=) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Assign<'a>>
    };
    (>) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::GreaterThan<'a>>
    };
    (?) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::QMark<'a>>
    };
    (*) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Wildcard<'a>>
    };
    (or) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Or<'a>>
    };
    (optional) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Optional<'a>>
    };
    (async) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Async<'a>>
    };
    (attribute) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Attribute<'a>>
    };
    (callback) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Callback<'a>>
    };
    (const) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Const<'a>>
    };
    (deleter) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Deleter<'a>>
    };
    (dictionary) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Dictionary<'a>>
    };
    (enum) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Enum<'a>>
    };
    (getter) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Getter<'a>>
    };
    (includes) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Includes<'a>>
    };
    (inherit) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Inherit<'a>>
    };
    (interface) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Interface<'a>>
    };
    (iterable) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Iterable<'a>>
    };
    (maplike) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Maplike<'a>>
    };
    (namespace) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Namespace<'a>>
    };
    (partial) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Partial<'a>>
    };
    (required) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Required<'a>>
    };
    (setlike) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Setlike<'a>>
    };
    (setter) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Setter<'a>>
    };
    (static) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Static<'a>>
    };
    (stringifier) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Stringifier<'a>>
    };
    (typedef) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Typedef<'a>>
    };
    (unrestricted) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Unrestricted<'a>>
    };
    (symbol) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Symbol<'a>>
    };
    (- Infinity) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::NegInfinity<'a>>
    };
    (ByteString) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::ByteString<'a>>
    };
    (DOMString) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::DOMString<'a>>
    };
    (FrozenArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::FrozenArray<'a>>
    };
    (Infinity) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Infinity<'a>>
    };
    (NaN) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::NaN<'a>>
    };
    (ObservableArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::ObservableArray<'a>>
    };
    (USVString) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::USVString<'a>>
    };
    (any) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Any<'a>>
    };
    (boolean) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Boolean<'a>>
    };
    (byte) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Byte<'a>>
    };
    (double) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Double<'a>>
    };
    (false) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::False<'a>>
    };
    (float) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Float<'a>>
    };
    (long) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Long<'a>>
    };
    (null) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Null<'a>>
    };
    (object) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Object<'a>>
    };
    (octet) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Octet<'a>>
    };
    (bigint) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Bigint<'a>>
    };
    (sequence) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Sequence<'a>>
    };
    (short) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Short<'a>>
    };
    (true) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::True<'a>>
    };
    (unsigned) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Unsigned<'a>>
    };
    (undefined) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Undefined<'a>>
    };
    (record) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Record<'a>>
    };
    (ArrayBuffer) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::ArrayBuffer<'a>>
    };
    (DataView) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::DataView<'a>>
    };
    (Int8Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Int8Array<'a>>
    };
    (Int16Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Int16Array<'a>>
    };
    (Int32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Int32Array<'a>>
    };
    (Uint8Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Uint8Array<'a>>
    };
    (Uint16Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Uint16Array<'a>>
    };
    (Uint32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Uint32Array<'a>>
    };
    (Uint8ClampedArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Uint8ClampedArray<'a>>
    };
    (BigInt64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::BigInt64Array<'a>>
    };
    (BigUint64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::BigUint64Array<'a>>
    };
    (Float32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Float32Array<'a>>
    };
    (Float64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Float64Array<'a>>
    };
    (Promise) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Promise<'a>>
    };
    (readonly) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::ReadOnly<'a>>
    };
    (mixin) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Mixin<'a>>
    };
    (constructor) => {
        $crate::parser::eat::VariantToken<'a, $crate::lexer::keywords::Constructor<'a>>
    };
}

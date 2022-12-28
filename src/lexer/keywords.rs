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
            #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
            pub struct $typ<'a>(pub &'a str);
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
                use crate::lexer::nom_branch_alt::alt;
                alt((
                    $(nom::combinator::map(
                        nom::combinator::recognize(nom::bytes::complete::tag($tok)),
                        |k| Keyword::$typ($typ(k))
                    ),)*
                ))(input)
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
    Dot => ".",
    Ellipsis => "...",
    Colon => ":",
    SemiColon => ";",
    LessThan => "<",
    Assign => "=",
    GreaterThan => ">",
    QuestionMark => "?",
    Wildcard => "*",
);

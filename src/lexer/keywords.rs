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
        ($($typ:ident => $tok:expr,)*),
        ($($typ_typename:ident => $tok_typename:expr,)*),
        ($($typ_string:ident => $tok_string:expr,)*),
        ($($typ_argname:ident => $tok_argname:expr,)*),
        ($($typ_argname2:ident => $tok_argname2:expr,)*),
        ($($typ_other:ident => $tok_other:expr,)*),
        ($($typ_other2:ident => $tok_other2:expr,)*)
    ) => {
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
        pub enum Keyword<'a> {
            $(
                #[doc=$tok]
                $typ(&'a str),
            )*
            $(
                #[doc=$tok_typename]
                $typ_typename(&'a str),
            )*
            $(
                #[doc=$tok_string]
                $typ_string(&'a str),
            )*
            $(
                #[doc=$tok_argname]
                $typ_argname(&'a str),
            )*
            $(
                #[doc=$tok_argname2]
                $typ_argname2(&'a str),
            )*
            $(
                #[doc=$tok_other]
                $typ_other(&'a str),
            )*
            $(
                #[doc=$tok_other2]
                $typ_other2(&'a str),
            )*
        }

        impl<'a> Keyword<'a> {
            pub fn parse(input: &str) -> nom::IResult<&str, Keyword> {
                nom::branch::alt((
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize(nom::bytes::complete::tag($tok)),
                            Keyword::$typ
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_typename)),
                            Keyword::$typ_typename
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_string)),
                            Keyword::$typ_string
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_argname)),
                            Keyword::$typ_argname
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_argname2)),
                            Keyword::$typ_argname2
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_other)),
                            Keyword::$typ_other
                        ),)*
                    )),
                    nom::branch::alt((
                        $(nom::combinator::map(
                            nom::combinator::recognize($crate::term::ident_tag($tok_other2)),
                            Keyword::$typ_other2
                        ),)*
                    )),
                ))(input)
            }
        }
    };
}

// One group can have at most 21 items, see:
// https://docs.rs/nom/latest/nom/branch/fn.alt.html
generate_keywords_enum!((
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
), (
    ArrayBuffer => "ArrayBuffer",
    DataView => "DataView",
    Int8Array => "Int8Array",
    Int16Array => "Int16Array",
    Int32Array => "Int32Array",
    Uint8Array => "Uint8Array",
    Uint16Array => "Uint16Array",
    Uint32Array => "Uint32Array",
    Uint8ClampedArray => "Uint8ClampedArray",
    Float32Array => "Float32Array",
    Float64Array => "Float64Array",
    Any => "any",
    Object => "object",
    Symbol => "symbol",
), (
    ByteString => "ByteString",
    DOMString => "DOMString",
    USVString => "USVString",
), (
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
), (
    Unrestricted => "unrestricted",
), (
    Or => "or",
    Optional => "optional",
    Const => "const",
    NegInfinity => "-Infinity",
    FrozenArray => "FrozenArray",
    Infinity => "Infinity",
    NaN => "NaN",
    ObservableArray => "ObservableArray",
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
), (
    Record => "record",
    Promise => "Promise",
    ReadOnly => "readonly",
    Mixin => "mixin",
));

macro_rules! generate_terms {
    ($( $(#[$attr:meta])* $typ:ident => $tok:expr ),*) => {
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
    ($( $(#[$attr:meta])* $typ:ident => $tok:expr,)*) => {
        $(
            $(#[$attr])*
            #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
            pub struct $typ;

            impl<'a> $crate::Parse<'a> for $typ {
                parser!(nom::combinator::value(
                    $typ,
                    $crate::whitespace::ws($crate::term::ident_tag($tok))
                ));
            }
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
    Wildcard => "*"
}

generate_terms_for_names! {
    /// Represents the terminal symbol `or`
    Or => "or",

    /// Represents the terminal symbol `optional`
    Optional => "optional",

    /// Represents the terminal symbol `async`
    Async => "async",

    /// Represents the terminal symbol `attribute`
    Attribute => "attribute",

    /// Represents the terminal symbol `callback`
    Callback => "callback",

    /// Represents the terminal symbol `const`
    Const => "const",

    /// Represents the terminal symbol `deleter`
    Deleter => "deleter",

    /// Represents the terminal symbol `dictionary`
    Dictionary => "dictionary",

    /// Represents the terminal symbol `enum`
    Enum => "enum",

    /// Represents the terminal symbol `getter`
    Getter => "getter",

    /// Represents the terminal symbol `includes`
    Includes => "includes",

    /// Represents the terminal symbol `inherit`
    Inherit => "inherit",

    /// Represents the terminal symbol `interface`
    Interface => "interface",

    /// Represents the terminal symbol `iterable`
    Iterable => "iterable",

    /// Represents the terminal symbol `maplike`
    Maplike => "maplike",

    /// Represents the terminal symbol `namespace`
    Namespace => "namespace",

    /// Represents the terminal symbol `partial`
    Partial => "partial",

    /// Represents the terminal symbol `required`
    Required => "required",

    /// Represents the terminal symbol `setlike`
    Setlike => "setlike",

    /// Represents the terminal symbol `setter`
    Setter => "setter",

    /// Represents the terminal symbol `static`
    Static => "static",

    /// Represents the terminal symbol `stringifier`
    Stringifier => "stringifier",

    /// Represents the terminal symbol `typedef`
    Typedef => "typedef",

    /// Represents the terminal symbol `unrestricted`
    Unrestricted => "unrestricted",

    /// Represents the terminal symbol `symbol`
    Symbol => "symbol",

    /// Represents the terminal symbol `Infinity`
    NegInfinity => "-Infinity",

    /// Represents the terminal symbol `ByteString`
    ByteString => "ByteString",

    /// Represents the terminal symbol `DOMString`
    DOMString => "DOMString",

    /// Represents the terminal symbol `FrozenArray`
    FrozenArray => "FrozenArray",

    /// Represents the terminal symbol `Infinity`
    Infinity => "Infinity",

    /// Represents the terminal symbol `NaN`
    NaN => "NaN",

    ObservableArray => "ObservableArray",

    /// Represents the terminal symbol `USVString`
    USVString => "USVString",

    /// Represents the terminal symbol `any`
    Any => "any",

    /// Represents the terminal symbol `boolean`
    Boolean => "boolean",

    /// Represents the terminal symbol `byte`
    Byte => "byte",

    /// Represents the terminal symbol `double`
    Double => "double",

    /// Represents the terminal symbol `false`
    False => "false",

    /// Represents the terminal symbol `float`
    Float => "float",

    /// Represents the terminal symbol `long`
    Long => "long",

    /// Represents the terminal symbol `null`
    Null => "null",

    /// Represents the terminal symbol `object`
    Object => "object",

    /// Represents the terminal symbol `octet`
    Octet => "octet",

    /// Represents the terminal symbol `sequence`
    Sequence => "sequence",

    /// Represents the terminal symbol `short`
    Short => "short",

    /// Represents the terminal symbol `true`
    True => "true",

    /// Represents the terminal symbol `unsigned`
    Unsigned => "unsigned",

    /// Represents the terminal symbol `undefined`
    Undefined => "undefined",

    /// Represents the terminal symbol `record`
    Record => "record",

    /// Represents the terminal symbol `ArrayBuffer`
    ArrayBuffer => "ArrayBuffer",

    /// Represents the terminal symbol `DataView`
    DataView => "DataView",

    /// Represents the terminal symbol `Int8Array`
    Int8Array => "Int8Array",

    /// Represents the terminal symbol `Int16Array`
    Int16Array => "Int16Array",

    /// Represents the terminal symbol `Int32Array`
    Int32Array => "Int32Array",

    /// Represents the terminal symbol `Uint8Array`
    Uint8Array => "Uint8Array",

    /// Represents the terminal symbol `Uint16Array`
    Uint16Array => "Uint16Array",

    /// Represents the terminal symbol `Uint32Array`
    Uint32Array => "Uint32Array",

    /// Represents the terminal symbol `Uint8ClampedArray`
    Uint8ClampedArray => "Uint8ClampedArray",

    /// Represents the terminal symbol `Float32Array`
    Float32Array => "Float32Array",

    /// Represents the terminal symbol `Float64Array`
    Float64Array => "Float64Array",

    /// Represents the terminal symbol `ArrayBufferView`
    ArrayBufferView => "ArrayBufferView",

    /// Represents the terminal symbol `BufferSource
    BufferSource => "BufferSource",

    /// Represents the terminal symbol `Promise`
    Promise => "Promise",

    /// Represents the terminal symbol `Error`
    Error => "Error",

    /// Represents the terminal symbol `readonly`
    ReadOnly => "readonly",

    /// Represents the terminal symbol `mixin`
    Mixin => "mixin",

    /// Represents the terminal symbol `implements`
    Implements => "implements",

    /// Represents the terminal symbol `legacycaller`
    LegacyCaller => "legacycaller",

    /// Represents the terminal symbol `constructor`
    Constructor => "constructor",
}

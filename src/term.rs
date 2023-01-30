/*
 * The following will ultimate generate:
 *
 * ```rust
 * #[derive(Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
 * pub struct OpenParen;
 *
 * impl OpenParen {
 *     pub fn value(&self) -> &'static str {
 *         return "(";
 *     }
 * }
 *
 * impl<'a> $crate::Parse<'a> for OpenParen {
 *     parser!(eat_key!(OpenParen));
 * }
 *
 * /* ... */
 *
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
            use $crate::lexer::{lex, Terminal};

            #[test]
            fn should_parse() {
                let tokens = lex($tok).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].value, Terminal::Keyword(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_preceding_spaces() {
                let tokens = lex(concat!("  ", $tok)).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].value, Terminal::Keyword(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_succeeding_spaces() {
                let tokens = lex(concat!($tok, "  ")).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].value, Terminal::Keyword(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_with_surrounding_spaces() {
                let tokens = lex(concat!("  ", $tok, "  ")).unwrap();
                assert_eq!(tokens.len(), 2);
                assert_eq!(tokens[0].value, Terminal::Keyword(Keyword::$typ($typ)));
            }

            #[test]
            fn should_parse_if_anything_next() {
                let tokens = lex(concat!($tok, "  anything")).unwrap();
                assert_eq!(tokens.len(), 3);
                assert_eq!(tokens[0].value, Terminal::Keyword(Keyword::$typ($typ)));
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

        impl<'a> $crate::Parse<'a> for $crate::parser::eat::VariantToken<'a, $typ> {
            parser!(eat_key!($typ));

            fn write(&self) -> String {
                let trivia = self.trivia;
                let variant = self.variant.value();
                format!("{trivia}{variant}")
            }
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
            pub fn parse_punc(input: &str) -> $crate::VerboseResult<&str, Keyword>
            {
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
    }
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
        $crate::parser::eat::VariantToken<'a, $crate::term::OpenParen>
    };
    (CloseParen) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::CloseParen>
    };
    (OpenBracket) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::OpenBracket>
    };
    (CloseBracket) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::CloseBracket>
    };
    (OpenBrace) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::OpenBrace>
    };
    (CloseBrace) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::CloseBrace>
    };
    (,) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Comma>
    };
    (-) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Minus>
    };
    (.) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Dot>
    };
    (...) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Ellipsis>
    };
    (:) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Colon>
    };
    (;) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::SemiColon>
    };
    (<) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::LessThan>
    };
    (=) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Assign>
    };
    (>) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::GreaterThan>
    };
    (?) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::QMark>
    };
    (*) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Wildcard>
    };
    (or) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Or>
    };
    (optional) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Optional>
    };
    (async) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Async>
    };
    (attribute) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Attribute>
    };
    (callback) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Callback>
    };
    (const) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Const>
    };
    (deleter) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Deleter>
    };
    (dictionary) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Dictionary>
    };
    (enum) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Enum>
    };
    (getter) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Getter>
    };
    (includes) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Includes>
    };
    (inherit) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Inherit>
    };
    (interface) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Interface>
    };
    (iterable) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Iterable>
    };
    (maplike) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Maplike>
    };
    (namespace) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Namespace>
    };
    (partial) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Partial>
    };
    (required) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Required>
    };
    (setlike) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Setlike>
    };
    (setter) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Setter>
    };
    (static) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Static>
    };
    (stringifier) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Stringifier>
    };
    (typedef) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Typedef>
    };
    (unrestricted) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Unrestricted>
    };
    (symbol) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Symbol>
    };
    (- Infinity) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::NegInfinity>
    };
    (ByteString) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::ByteString>
    };
    (DOMString) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::DOMString>
    };
    (FrozenArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::FrozenArray>
    };
    (Infinity) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Infinity>
    };
    (NaN) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::NaN>
    };
    (ObservableArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::ObservableArray>
    };
    (USVString) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::USVString>
    };
    (any) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Any>
    };
    (bigint) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Bigint>
    };
    (boolean) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Boolean>
    };
    (byte) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Byte>
    };
    (double) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Double>
    };
    (false) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::False>
    };
    (float) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Float>
    };
    (long) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Long>
    };
    (null) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Null>
    };
    (object) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Object>
    };
    (octet) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Octet>
    };
    (sequence) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Sequence>
    };
    (short) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Short>
    };
    (true) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::True>
    };
    (unsigned) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Unsigned>
    };
    (undefined) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Undefined>
    };
    (record) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Record>
    };
    (ArrayBuffer) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::ArrayBuffer>
    };
    (DataView) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::DataView>
    };
    (Int8Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Int8Array>
    };
    (Int16Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Int16Array>
    };
    (Int32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Int32Array>
    };
    (Uint8Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Uint8Array>
    };
    (Uint16Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Uint16Array>
    };
    (Uint32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Uint32Array>
    };
    (Uint8ClampedArray) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Uint8ClampedArray>
    };
    (BigInt64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::BigInt64Array>
    };
    (BigUint64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::BigUint64Array>
    };
    (Float32Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Float32Array>
    };
    (Float64Array) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Float64Array>
    };
    (Promise) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Promise>
    };
    (readonly) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::ReadOnly>
    };
    (mixin) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Mixin>
    };
    (constructor) => {
        $crate::parser::eat::VariantToken<'a, $crate::term::Constructor>
    };
}

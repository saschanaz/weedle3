#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Token<'a, T> {
    pub trivia: &'a str,
    pub variant: T,
}

impl<'a, T> Default for Token<'a, T>
where
    T: Default,
{
    fn default() -> Self {
        Token {
            variant: T::default(),
            trivia: "",
        }
    }
}

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

        impl<'a> $crate::Parse<'a> for $crate::term::Token<'a, $typ> {
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
        $crate::term::Token<'a, $crate::term::OpenParen>
    };
    (CloseParen) => {
        $crate::term::Token<'a, $crate::term::CloseParen>
    };
    (OpenBracket) => {
        $crate::term::Token<'a, $crate::term::OpenBracket>
    };
    (CloseBracket) => {
        $crate::term::Token<'a, $crate::term::CloseBracket>
    };
    (OpenBrace) => {
        $crate::term::Token<'a, $crate::term::OpenBrace>
    };
    (CloseBrace) => {
        $crate::term::Token<'a, $crate::term::CloseBrace>
    };
    (,) => {
        $crate::term::Token<'a, $crate::term::Comma>
    };
    (-) => {
        $crate::term::Token<'a, $crate::term::Minus>
    };
    (.) => {
        $crate::term::Token<'a, $crate::term::Dot>
    };
    (...) => {
        $crate::term::Token<'a, $crate::term::Ellipsis>
    };
    (:) => {
        $crate::term::Token<'a, $crate::term::Colon>
    };
    (;) => {
        $crate::term::Token<'a, $crate::term::SemiColon>
    };
    (<) => {
        $crate::term::Token<'a, $crate::term::LessThan>
    };
    (=) => {
        $crate::term::Token<'a, $crate::term::Assign>
    };
    (>) => {
        $crate::term::Token<'a, $crate::term::GreaterThan>
    };
    (?) => {
        $crate::term::Token<'a, $crate::term::QMark>
    };
    (*) => {
        $crate::term::Token<'a, $crate::term::Wildcard>
    };
    (or) => {
        $crate::term::Token<'a, $crate::term::Or>
    };
    (optional) => {
        $crate::term::Token<'a, $crate::term::Optional>
    };
    (async) => {
        $crate::term::Token<'a, $crate::term::Async>
    };
    (attribute) => {
        $crate::term::Token<'a, $crate::term::Attribute>
    };
    (callback) => {
        $crate::term::Token<'a, $crate::term::Callback>
    };
    (const) => {
        $crate::term::Token<'a, $crate::term::Const>
    };
    (deleter) => {
        $crate::term::Token<'a, $crate::term::Deleter>
    };
    (dictionary) => {
        $crate::term::Token<'a, $crate::term::Dictionary>
    };
    (enum) => {
        $crate::term::Token<'a, $crate::term::Enum>
    };
    (getter) => {
        $crate::term::Token<'a, $crate::term::Getter>
    };
    (includes) => {
        $crate::term::Token<'a, $crate::term::Includes>
    };
    (inherit) => {
        $crate::term::Token<'a, $crate::term::Inherit>
    };
    (interface) => {
        $crate::term::Token<'a, $crate::term::Interface>
    };
    (iterable) => {
        $crate::term::Token<'a, $crate::term::Iterable>
    };
    (maplike) => {
        $crate::term::Token<'a, $crate::term::Maplike>
    };
    (namespace) => {
        $crate::term::Token<'a, $crate::term::Namespace>
    };
    (partial) => {
        $crate::term::Token<'a, $crate::term::Partial>
    };
    (required) => {
        $crate::term::Token<'a, $crate::term::Required>
    };
    (setlike) => {
        $crate::term::Token<'a, $crate::term::Setlike>
    };
    (setter) => {
        $crate::term::Token<'a, $crate::term::Setter>
    };
    (static) => {
        $crate::term::Token<'a, $crate::term::Static>
    };
    (stringifier) => {
        $crate::term::Token<'a, $crate::term::Stringifier>
    };
    (typedef) => {
        $crate::term::Token<'a, $crate::term::Typedef>
    };
    (unrestricted) => {
        $crate::term::Token<'a, $crate::term::Unrestricted>
    };
    (symbol) => {
        $crate::term::Token<'a, $crate::term::Symbol>
    };
    (- Infinity) => {
        $crate::term::Token<'a, $crate::term::NegInfinity>
    };
    (ByteString) => {
        $crate::term::Token<'a, $crate::term::ByteString>
    };
    (DOMString) => {
        $crate::term::Token<'a, $crate::term::DOMString>
    };
    (FrozenArray) => {
        $crate::term::Token<'a, $crate::term::FrozenArray>
    };
    (Infinity) => {
        $crate::term::Token<'a, $crate::term::Infinity>
    };
    (NaN) => {
        $crate::term::Token<'a, $crate::term::NaN>
    };
    (ObservableArray) => {
        $crate::term::Token<'a, $crate::term::ObservableArray>
    };
    (USVString) => {
        $crate::term::Token<'a, $crate::term::USVString>
    };
    (any) => {
        $crate::term::Token<'a, $crate::term::Any>
    };
    (bigint) => {
        $crate::term::Token<'a, $crate::term::Bigint>
    };
    (boolean) => {
        $crate::term::Token<'a, $crate::term::Boolean>
    };
    (byte) => {
        $crate::term::Token<'a, $crate::term::Byte>
    };
    (double) => {
        $crate::term::Token<'a, $crate::term::Double>
    };
    (false) => {
        $crate::term::Token<'a, $crate::term::False>
    };
    (float) => {
        $crate::term::Token<'a, $crate::term::Float>
    };
    (long) => {
        $crate::term::Token<'a, $crate::term::Long>
    };
    (null) => {
        $crate::term::Token<'a, $crate::term::Null>
    };
    (object) => {
        $crate::term::Token<'a, $crate::term::Object>
    };
    (octet) => {
        $crate::term::Token<'a, $crate::term::Octet>
    };
    (sequence) => {
        $crate::term::Token<'a, $crate::term::Sequence>
    };
    (short) => {
        $crate::term::Token<'a, $crate::term::Short>
    };
    (true) => {
        $crate::term::Token<'a, $crate::term::True>
    };
    (unsigned) => {
        $crate::term::Token<'a, $crate::term::Unsigned>
    };
    (undefined) => {
        $crate::term::Token<'a, $crate::term::Undefined>
    };
    (record) => {
        $crate::term::Token<'a, $crate::term::Record>
    };
    (ArrayBuffer) => {
        $crate::term::Token<'a, $crate::term::ArrayBuffer>
    };
    (DataView) => {
        $crate::term::Token<'a, $crate::term::DataView>
    };
    (Int8Array) => {
        $crate::term::Token<'a, $crate::term::Int8Array>
    };
    (Int16Array) => {
        $crate::term::Token<'a, $crate::term::Int16Array>
    };
    (Int32Array) => {
        $crate::term::Token<'a, $crate::term::Int32Array>
    };
    (Uint8Array) => {
        $crate::term::Token<'a, $crate::term::Uint8Array>
    };
    (Uint16Array) => {
        $crate::term::Token<'a, $crate::term::Uint16Array>
    };
    (Uint32Array) => {
        $crate::term::Token<'a, $crate::term::Uint32Array>
    };
    (Uint8ClampedArray) => {
        $crate::term::Token<'a, $crate::term::Uint8ClampedArray>
    };
    (BigInt64Array) => {
        $crate::term::Token<'a, $crate::term::BigInt64Array>
    };
    (BigUint64Array) => {
        $crate::term::Token<'a, $crate::term::BigUint64Array>
    };
    (Float32Array) => {
        $crate::term::Token<'a, $crate::term::Float32Array>
    };
    (Float64Array) => {
        $crate::term::Token<'a, $crate::term::Float64Array>
    };
    (Promise) => {
        $crate::term::Token<'a, $crate::term::Promise>
    };
    (readonly) => {
        $crate::term::Token<'a, $crate::term::ReadOnly>
    };
    (mixin) => {
        $crate::term::Token<'a, $crate::term::Mixin>
    };
    (constructor) => {
        $crate::term::Token<'a, $crate::term::Constructor>
    };
}

use nom::Parser;
use weedle_derive::Weedle;

use crate::{parser::eat::VariantToken, term, Parse};

/// Parses `-?[1-9][0-9]*`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DecLit<'a>(pub &'a str);

impl<'a> DecLit<'a> {
    lexer!(nom::combinator::map(
        nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(nom::character::complete::char('-')),
            nom::character::complete::one_of("123456789"),
            nom::bytes::complete::take_while(nom::AsChar::is_dec_digit),
        ))),
        DecLit,
    ));
}

/// Parses `-?0[Xx][0-9A-Fa-f]+)`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct HexLit<'a>(pub &'a str);

impl<'a> HexLit<'a> {
    lexer!(nom::combinator::map(
        nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(nom::character::complete::char('-')),
            nom::character::complete::char('0'),
            nom::character::complete::one_of("xX"),
            nom::bytes::complete::take_while(nom::AsChar::is_hex_digit),
        ))),
        HexLit,
    ));
}

/// Parses `-?0[0-7]*`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OctLit<'a>(pub &'a str);

impl<'a> OctLit<'a> {
    lexer!(nom::combinator::map(
        nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(nom::character::complete::char('-')),
            nom::character::complete::char('0'),
            nom::bytes::complete::take_while(nom::AsChar::is_oct_digit),
        ))),
        OctLit,
    ));
}

/// Represents an integer value
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerLit<'a> {
    Dec(DecLit<'a>),
    Hex(HexLit<'a>),
    Oct(OctLit<'a>),
}

impl<'a> IntegerLit<'a> {
    lexer!(nom::branch::alt((
        DecLit::lex.map(IntegerLit::Dec),
        HexLit::lex.map(IntegerLit::Hex),
        OctLit::lex.map(IntegerLit::Oct),
    )));
}

impl<'a> Parse<'a> for VariantToken<'a, IntegerLit<'a>> {
    parser!(eat!(Integer));
}

/// Represents a string value
///
/// Follow `/"[^"]*"/`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringLit<'a>(pub &'a str);

impl<'a> StringLit<'a> {
    lexer!(nom::combinator::map(
        nom::sequence::delimited(
            nom::character::complete::char('\"'),
            nom::bytes::complete::take_while(|c| c != '\"'),
            nom::character::complete::char('\"'),
        ),
        StringLit,
    ));
}

impl<'a> Parse<'a> for VariantToken<'a, StringLit<'a>> {
    parser!(eat!(String));
}

/// Represents `[ ]`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmptyArrayLit<'a> {
    pub open_bracket: term!(OpenBracket),
    #[weedle(cut = "Default sequence value must be empty")]
    pub close_bracket: term!(CloseBracket),
}

/// Represents `{ }`
#[derive(Weedle, Copy, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EmptyDictionaryLit<'a> {
    pub open_brace: term!(OpenBrace),
    #[weedle(cut = "Default dictionary value must be empty")]
    pub close_brace: term!(CloseBrace),
}

/// Represents a default literal value. Ex: `34|34.23|"value"|[ ]|true|false|null`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DefaultValue<'a> {
    Boolean(BooleanLit),
    EmptyArray(EmptyArrayLit<'a>),
    EmptyDictionary(EmptyDictionaryLit<'a>),
    Float(FloatLit<'a>),
    Integer(VariantToken<'a, IntegerLit<'a>>),
    Null(term!(null)),
    String(VariantToken<'a, StringLit<'a>>),
}

/// Represents `true`, `false`, `34.23`, `null`, `56`, ...
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstValue<'a> {
    Boolean(BooleanLit),
    Float(FloatLit<'a>),
    Integer(VariantToken<'a, IntegerLit<'a>>),
}

/// Represents either `true` or `false`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BooleanLit(bool);

impl<'a> Parse<'a> for BooleanLit {
    parser!(nom::combinator::map(
        nom::branch::alt((
            nom::combinator::value(true, weedle!(term!(true))),
            nom::combinator::value(false, weedle!(term!(false))),
        )),
        BooleanLit
    ));
}

/// Parses `/-?(([0-9]+\.[0-9]*|[0-9]*\.[0-9]+)([Ee][+-]?[0-9]+)?|[0-9]+[Ee][+-]?[0-9]+)/`
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FloatValueLit<'a>(pub &'a str);

impl<'a> FloatValueLit<'a> {
    lexer!(nom::combinator::map(
        nom::combinator::recognize(nom::sequence::tuple((
            nom::combinator::opt(nom::character::complete::char('-')),
            nom::branch::alt((
                nom::combinator::value(
                    (),
                    nom::sequence::tuple((
                        // (?:[0-9]+\.[0-9]*|[0-9]*\.[0-9]+)
                        nom::branch::alt((
                            nom::sequence::tuple((
                                nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit),
                                nom::character::complete::char('.'),
                                nom::bytes::complete::take_while(nom::AsChar::is_dec_digit),
                            )),
                            nom::sequence::tuple((
                                nom::bytes::complete::take_while(nom::AsChar::is_dec_digit),
                                nom::character::complete::char('.'),
                                nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit),
                            )),
                        )),
                        // (?:[Ee][+-]?[0-9]+)?
                        nom::combinator::opt(nom::sequence::tuple((
                            nom::character::complete::one_of("eE"),
                            nom::combinator::opt(nom::character::complete::one_of("+-")),
                            nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit),
                        ))),
                    )),
                ),
                // [0-9]+[Ee][+-]?[0-9]+
                nom::combinator::value(
                    (),
                    nom::sequence::tuple((
                        nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit),
                        nom::character::complete::one_of("eE"),
                        nom::combinator::opt(nom::character::complete::one_of("+-")),
                        nom::bytes::complete::take_while1(nom::AsChar::is_dec_digit),
                    )),
                ),
            )),
        ))),
        FloatValueLit,
    ));
}

impl<'a> Parse<'a> for VariantToken<'a, FloatValueLit<'a>> {
    parser!(eat!(Decimal));
}

/// Represents a floating point value, `NaN`, `Infinity`, '+Infinity`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatLit<'a> {
    Value(VariantToken<'a, FloatValueLit<'a>>),
    NegInfinity(term!(-Infinity)),
    Infinity(term!(Infinity)),
    NaN(term!(NaN)),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::eat::VariantToken;
    use crate::term::Null;
    use crate::Parse;

    test_match!(should_parse_integer { "45" =>
        "";
        IntegerLit => IntegerLit::Dec(DecLit("45"))
    });

    test!(should_parse_integer_surrounding_with_spaces { "  123123  " =>
        "";
        VariantToken<IntegerLit> => VariantToken { variant: IntegerLit::Dec(DecLit("123123")), trivia: "  " }
    });

    test_match!(should_parse_integer_preceding_others { "3453 string" =>
        " string";
        IntegerLit => IntegerLit::Dec(DecLit("3453"))
    });

    test_match!(should_parse_neg_integer { "-435" =>
        "";
        IntegerLit => IntegerLit::Dec(DecLit("-435"))
    });

    test_match!(should_parse_hex_number { "0X08" =>
        "";
        IntegerLit => IntegerLit::Hex(HexLit("0X08"))
    });

    test_match!(should_parse_hex_large_number { "0xA" =>
        "";
        IntegerLit => IntegerLit::Hex(HexLit("0xA"))
    });

    test_match!(should_parse_zero { "0" =>
        "";
        IntegerLit => IntegerLit::Oct(OctLit("0"))
    });

    test_match!(should_parse_oct_number { "-07561" =>
        "";
        IntegerLit => IntegerLit::Oct(OctLit("-07561"))
    });

    test!(should_parse_float { "45.434" =>
        "";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("45.434"), trivia: "" })
    });

    test!(should_parse_float_surrounding_with_spaces { "  2345.2345  " =>
        "";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("2345.2345"), trivia: "  " })
    });

    test!(should_parse_float_preceding_others { "3453.32334 string" =>
        "string";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("3453.32334"), trivia: "" })
    });

    test!(should_parse_neg_float { "-435.3435" =>
        "";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("-435.3435"), trivia: "" })
    });

    test!(should_parse_float_exp { "3e23" =>
        "";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("3e23"), trivia: "" })
    });

    test!(should_parse_float_exp_with_decimal { "5.3434e23" =>
        "";
        FloatLit => FloatLit::Value(VariantToken { variant: FloatValueLit("5.3434e23"), trivia: "" })
    });

    test!(should_parse_neg_infinity { "-Infinity" =>
        "";
        FloatLit => FloatLit::NegInfinity(VariantToken {
            variant: crate::term::NegInfinity,
            trivia: "",
        })
    });

    test!(should_parse_infinity { "Infinity" =>
        "";
        FloatLit => FloatLit::Infinity(VariantToken {
            variant: crate::term::Infinity,
            trivia: "",
        })
    });

    test_match!(should_parse_string { r#""this is a string""# =>
        "";
        StringLit => StringLit("this is a string")
    });

    test!(should_parse_string_surround_with_spaces { r#"  "this is a string"  "# =>
        "";
        VariantToken<StringLit> => VariantToken { variant: StringLit("this is a string"), trivia: "  " }
    });

    test!(should_parse_string_followed_by_string { r#" "this is first"  "this is second" "# =>
        r#""this is second" "#;
        VariantToken<StringLit> => VariantToken { variant: StringLit("this is first"), trivia: " " }
    });

    test!(should_parse_string_with_spaces { r#"  "  this is a string  "  "# =>
        "";
        VariantToken<StringLit> => VariantToken { variant: StringLit("  this is a string  "), trivia: "  " }
    });

    test!(should_parse_string_with_comment { r#"  "// this is still a string"
     "# =>
        "";
        VariantToken<StringLit> => VariantToken { variant: StringLit("// this is still a string"), trivia: "  " }
    });

    test!(should_parse_string_with_multiline_comment { r#"  "/*"  "*/"  "# =>
        r#""*/"  "#;
        VariantToken<StringLit> => VariantToken { variant: StringLit("/*"), trivia: "  " }
    });

    test!(should_parse_null { "null" =>
        "";
        VariantToken<Null> => VariantToken::default()
    });

    test!(should_parse_empty_array { "[]" =>
        "";
        EmptyArrayLit => Default::default()
    });

    test!(should_parse_bool_true { "true" =>
        "";
        BooleanLit => BooleanLit(true)
    });

    test!(should_parse_bool_false { "false" =>
        "";
        BooleanLit => BooleanLit(false)
    });
}

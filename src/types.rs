use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Generics, Identifier, Parenthesized, Punctuated};
use crate::lex_term;
use crate::lexer::keywords;
use crate::parser::eat::VariantToken;
use crate::Parse;

/// Parses a union of types
pub type UnionType<'a> = Parenthesized<'a, Punctuated<UnionMemberType<'a>, lex_term!(or)>>;

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SingleType<'a> {
    Any(lex_term!(any)),
    NonAny(NonAnyType<'a>),
}

/// Parses either single type or a union type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Single(SingleType<'a>),
    Union(MayBeNull<'a, UnionType<'a>>),
}

// Parses any single non-any type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NonAnyType<'a> {
    Promise(PromiseType<'a>),
    Integer(MayBeNull<'a, IntegerType<'a>>),
    FloatingPoint(MayBeNull<'a, FloatingPointType<'a>>),
    Boolean(MayBeNull<'a, lex_term!(boolean)>),
    Byte(MayBeNull<'a, lex_term!(byte)>),
    Octet(MayBeNull<'a, lex_term!(octet)>),
    ByteString(MayBeNull<'a, lex_term!(ByteString)>),
    DOMString(MayBeNull<'a, lex_term!(DOMString)>),
    USVString(MayBeNull<'a, lex_term!(USVString)>),
    Sequence(MayBeNull<'a, SequenceType<'a>>),
    Object(MayBeNull<'a, lex_term!(object)>),
    Symbol(MayBeNull<'a, lex_term!(symbol)>),
    ArrayBuffer(MayBeNull<'a, lex_term!(ArrayBuffer)>),
    DataView(MayBeNull<'a, lex_term!(DataView)>),
    Int8Array(MayBeNull<'a, lex_term!(Int8Array)>),
    Int16Array(MayBeNull<'a, lex_term!(Int16Array)>),
    Int32Array(MayBeNull<'a, lex_term!(Int32Array)>),
    Uint8Array(MayBeNull<'a, lex_term!(Uint8Array)>),
    Uint16Array(MayBeNull<'a, lex_term!(Uint16Array)>),
    Uint32Array(MayBeNull<'a, lex_term!(Uint32Array)>),
    Uint8ClampedArray(MayBeNull<'a, lex_term!(Uint8ClampedArray)>),
    Float32Array(MayBeNull<'a, lex_term!(Float32Array)>),
    Float64Array(MayBeNull<'a, lex_term!(Float64Array)>),
    FrozenArrayType(MayBeNull<'a, FrozenArrayType<'a>>),
    ObservableArrayType(MayBeNull<'a, ObservableArrayType<'a>>),
    RecordType(MayBeNull<'a, RecordType<'a>>),
    Identifier(MayBeNull<'a, Identifier<'a>>),
}

/// Parses `sequence<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SequenceType<'a> {
    pub sequence: lex_term!(sequence),
    pub generics: Generics<'a, Box<AttributedType<'a>>>,
}

/// Parses `FrozenArray<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrozenArrayType<'a> {
    pub frozen_array: lex_term!(FrozenArray),
    pub generics: Generics<'a, Box<AttributedType<'a>>>,
}

/// Parses `ObservableArray<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObservableArrayType<'a> {
    pub observable_array: lex_term!(ObservableArray),
    pub generics: Generics<'a, Box<AttributedType<'a>>>,
}

/// Parses a nullable type. Ex: `object | object??`
///
/// `??` means an actual ? not an optional requirement
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct MayBeNull<'a, T> {
    pub type_: T,
    pub q_mark: Option<VariantToken<'a, keywords::QuestionMark<'a>>>,
}

/// Parses a `Promise<Type|undefined>` type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PromiseType<'a> {
    pub promise: lex_term!(Promise),
    pub generics: Generics<'a, Box<ReturnType<'a>>>,
}

/// Parses `unsigned? long long`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongLongType<'a> {
    pub unsigned: Option<lex_term!(unsigned)>,
    pub long_long: (lex_term!(long), lex_term!(long)),
}

/// Parses `unsigned? long`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongType<'a> {
    pub unsigned: Option<lex_term!(unsigned)>,
    pub long: lex_term!(long),
}

/// Parses `unsigned? short`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShortType<'a> {
    pub unsigned: Option<lex_term!(unsigned)>,
    pub short: lex_term!(short),
}

/// Parses `unsigned? short|long|(long long)`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerType<'a> {
    LongLong(LongLongType<'a>),
    Long(LongType<'a>),
    Short(ShortType<'a>),
}

/// Parses `unrestricted? float`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FloatType<'a> {
    pub unrestricted: Option<lex_term!(unrestricted)>,
    pub float: lex_term!(float),
}

/// Parses `unrestricted? double`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleType<'a> {
    pub unrestricted: Option<lex_term!(unrestricted)>,
    pub double: lex_term!(double),
}

/// Parses `unrestricted? float|double`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatingPointType<'a> {
    Float(FloatType<'a>),
    Double(DoubleType<'a>),
}

/// Parses `record<StringType, Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordType<'a> {
    pub record: lex_term!(record),
    pub generics: Generics<
        'a,
        (
            Box<RecordKeyType<'a>>,
            lex_term!(,),
            Box<AttributedType<'a>>,
        ),
    >,
}

/// Parses one of the string types `ByteString|DOMString|USVString` or any other type.
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RecordKeyType<'a> {
    Byte(lex_term!(ByteString)),
    DOM(lex_term!(DOMString)),
    USV(lex_term!(USVString)),
    NonAny(NonAnyType<'a>),
}

/// Parses one of the member of a union type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum UnionMemberType<'a> {
    Single(AttributedNonAnyType<'a>),
    Union(MayBeNull<'a, UnionType<'a>>),
}

/// Parses a const type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstType<'a> {
    Integer(MayBeNull<'a, IntegerType<'a>>),
    FloatingPoint(MayBeNull<'a, FloatingPointType<'a>>),
    Boolean(MayBeNull<'a, lex_term!(boolean)>),
    Byte(MayBeNull<'a, lex_term!(byte)>),
    Octet(MayBeNull<'a, lex_term!(octet)>),
    Identifier(MayBeNull<'a, Identifier<'a>>),
}

/// Parses the return type which may be `undefined` or any given Type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ReturnType<'a> {
    Undefined(lex_term!(undefined)),
    Type(Type<'a>),
}

/// Parses `[attributes]? type`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributedType<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub type_: Type<'a>,
}

/// Parses `[attributes]? type` where the type is a single non-any type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributedNonAnyType<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub type_: NonAnyType<'a>,
}

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_may_be_null { "short" =>
        "";
        MayBeNull<crate::types::IntegerType>;
        q_mark.is_none();
    });

    test!(should_parse_nullable { "short?" =>
        "";
        MayBeNull<crate::types::IntegerType>;
        q_mark.is_some();
    });

    test_variants!(
        ReturnType {
            Undefined == "undefined",
            Type == "any",
        }
    );

    test_variants!(
        ConstType {
            Integer == "short",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            Identifier == "name",
        }
    );

    test_variants!(
        NonAnyType {
            Promise == "Promise<long>",
            Integer == "long",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            ByteString == "ByteString",
            DOMString == "DOMString",
            USVString == "USVString",
            Sequence == "sequence<short>",
            Object == "object",
            Symbol == "symbol",
            ArrayBuffer == "ArrayBuffer",
            DataView == "DataView",
            Int8Array == "Int8Array",
            Int16Array == "Int16Array",
            Int32Array == "Int32Array",
            Uint8Array == "Uint8Array",
            Uint16Array == "Uint16Array",
            Uint32Array == "Uint32Array",
            Uint8ClampedArray == "Uint8ClampedArray",
            Float32Array == "Float32Array",
            Float64Array == "Float64Array",
            FrozenArrayType == "FrozenArray<short>",
            RecordType == "record<DOMString, short>",
            Identifier == "mango"
        }
    );

    test_variants!(
        UnionMemberType {
            Single == "byte",
            Union == "([Clamp] unsigned long or byte)"
        }
    );

    test_variants!(
        RecordKeyType {
            DOM == "DOMString",
            USV == "USVString",
            Byte == "ByteString"
        }
    );

    test!(should_parse_record_type { "record<DOMString, short>" =>
        "";
        RecordType;
    });

    test!(should_parse_record_type_alt_types { "record<u64, short>" =>
        "";
        RecordType;
    });

    test!(should_parse_double_type { "double" =>
        "";
        DoubleType;
    });

    test!(should_parse_float_type { "float" =>
        "";
        FloatType;
    });

    test_variants!(
        FloatingPointType {
            Float == "float",
            Double == "double"
        }
    );

    test!(should_parse_long_long_type { "long long" =>
        "";
        LongLongType;
    });

    test!(should_parse_long_type { "long" =>
        "";
        LongType;
    });

    test!(should_parse_short_type { "short" =>
        "";
        ShortType;
    });

    test_variants!(
        IntegerType {
            Short == "short",
            Long == "long",
            LongLong == "long long"
        }
    );

    test!(should_parse_promise_type { "Promise<short>" =>
        "";
        PromiseType;
    });

    test!(should_parse_frozen_array_type { "FrozenArray<short>" =>
        "";
        FrozenArrayType;
    });

    test!(should_parse_sequence_type { "sequence<short>" =>
        "";
        SequenceType;
    });

    test_variants!(
        SingleType {
            Any == "any",
            NonAny == "Promise<short>",
        }
    );

    test_variants!(
        Type {
            Single == "short",
            Union == "(short or float)"
        }
    );

    test!(should_parse_attributed_type { "[Named] short" =>
        "";
        AttributedType;
        attributes.is_some();
    });

    test!(should_parse_type_as_identifier { "DOMStringMap" =>
        // if type is not parsed as identifier, it is parsed as `DOMString` and 'Map' is left
        "";
        crate::types::Type;
    });

    #[test]
    fn should_parse_union_member_type_attributed_union() {
        use crate::types::UnionMemberType;
        let (rem, parsed) = UnionMemberType::parse("([Clamp] byte or [Named] byte)").unwrap();
        assert_eq!(rem, "");
        match parsed {
            UnionMemberType::Union(MayBeNull {
                type_:
                    Parenthesized {
                        body: Punctuated { list, .. },
                        ..
                    },
                ..
            }) => {
                assert_eq!(list.len(), 2);

                match list[0] {
                    UnionMemberType::Single(AttributedNonAnyType { ref attributes, .. }) => {
                        assert!(attributes.is_some());
                    }

                    _ => {
                        panic!("Failed to parse list[0] attributes");
                    }
                };

                match list[1] {
                    UnionMemberType::Single(AttributedNonAnyType { ref attributes, .. }) => {
                        assert!(attributes.is_some());
                    }

                    _ => {
                        panic!("Failed to parse list[1] attributes");
                    }
                };
            }

            _ => {
                panic!("Failed to parse");
            }
        }
    }
}

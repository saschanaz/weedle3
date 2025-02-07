use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Generics, Identifier, Parenthesized, Punctuated};
use crate::tokens::{contextful_cut, Tokens};
use crate::Parse;
use crate::{term, VerboseResult};

/// Parses a union of types
pub type UnionType<'a> = Parenthesized<Punctuated<UnionMemberType<'a>, term!(or)>>;

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct UnionTypeMultiple<'a> {
    open_paren: term!(OpenParen),
    first: UnionMemberType<'a>,
    #[weedle(cut = "A union type needs at least two member types")]
    or: term!(or),
    more: Punctuated<UnionMemberType<'a>, term!(or)>,
    #[weedle(cut = "Expected a union member type")]
    close_paren: term!(CloseParen),
}

impl<'a> From<UnionTypeMultiple<'a>> for UnionType<'a> {
    fn from(value: UnionTypeMultiple<'a>) -> Self {
        // XXX: request nom::multi::separated_list_m_n?
        let mut list = vec![value.first];
        list.extend(value.more.list);
        Self {
            open_paren: value.open_paren,
            body: Punctuated {
                list,
                separator: term!(or),
            },
            close_paren: value.close_paren,
        }
    }
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SingleType<'a> {
    #[weedle(post_check = "prevent_unexpected_nullable")]
    Any(term!(any)),
    #[weedle(post_check = "prevent_unexpected_nullable")]
    Promise(PromiseType<'a>),
    Distinguishable(DistinguishableType<'a>),
}

fn prevent_unexpected_nullable<'slice, 'a>(
    input: Tokens<'slice, 'a>,
) -> VerboseResult<Tokens<'slice, 'a>, ()> {
    contextful_cut(
        "`any` and Promise cannot be nullable",
        nom::combinator::not(nom::combinator::peek(eat_key!(QMark))),
    )(input)
}

/// Parses either single type or a union type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Type<'a> {
    Single(SingleType<'a>),
    #[weedle(from = "MayBeNull<UnionTypeMultiple<'a>>", generic_into)]
    Union(MayBeNull<UnionType<'a>>),
}

// Parses any single non-any type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DistinguishableType<'a> {
    Integer(MayBeNull<IntegerType>),
    FloatingPoint(MayBeNull<FloatingPointType>),
    Boolean(MayBeNull<term!(boolean)>),
    Byte(MayBeNull<term!(byte)>),
    Octet(MayBeNull<term!(octet)>),
    Bigint(MayBeNull<term!(bigint)>),
    ByteString(MayBeNull<term!(ByteString)>),
    DOMString(MayBeNull<term!(DOMString)>),
    USVString(MayBeNull<term!(USVString)>),
    Sequence(MayBeNull<SequenceType<'a>>),
    Object(MayBeNull<term!(object)>),
    Symbol(MayBeNull<term!(symbol)>),
    ArrayBuffer(MayBeNull<term!(ArrayBuffer)>),
    DataView(MayBeNull<term!(DataView)>),
    Int8Array(MayBeNull<term!(Int8Array)>),
    Int16Array(MayBeNull<term!(Int16Array)>),
    Int32Array(MayBeNull<term!(Int32Array)>),
    Uint8Array(MayBeNull<term!(Uint8Array)>),
    Uint16Array(MayBeNull<term!(Uint16Array)>),
    Uint32Array(MayBeNull<term!(Uint32Array)>),
    Uint8ClampedArray(MayBeNull<term!(Uint8ClampedArray)>),
    BigInt64Array(MayBeNull<term!(BigInt64Array)>),
    BigUint64Array(MayBeNull<term!(BigUint64Array)>),
    Float32Array(MayBeNull<term!(Float32Array)>),
    Float64Array(MayBeNull<term!(Float64Array)>),
    FrozenArrayType(MayBeNull<FrozenArrayType<'a>>),
    ObservableArrayType(MayBeNull<ObservableArrayType<'a>>),
    RecordType(MayBeNull<RecordType<'a>>),
    Undefined(MayBeNull<term!(undefined)>),
    Identifier(MayBeNull<Identifier<'a>>),
}

/// Parses `sequence<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SequenceType<'a> {
    pub sequence: term!(sequence),
    pub generics: Generics<Box<AttributedType<'a>>>,
}

/// Parses `FrozenArray<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrozenArrayType<'a> {
    pub frozen_array: term!(FrozenArray),
    pub generics: Generics<Box<AttributedType<'a>>>,
}

/// Parses `ObservableArray<Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ObservableArrayType<'a> {
    pub observable_array: term!(ObservableArray),
    pub generics: Generics<Box<AttributedType<'a>>>,
}

/// Parses a nullable type. Ex: `object | object??`
///
/// `??` means an actual ? not an optional requirement
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(impl_bound = "where T: Parse<'a>")]
pub struct MayBeNull<T> {
    pub type_: T,
    pub q_mark: Option<term::QMark>,
}

impl<T> MayBeNull<T> {
    pub fn generic_into<S: From<T>>(self) -> MayBeNull<S> {
        let MayBeNull { type_, q_mark } = self;
        MayBeNull {
            type_: type_.into(),
            q_mark,
        }
    }
}

/// Parses a `Promise<Type|undefined>` type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PromiseType<'a> {
    pub promise: term!(Promise),
    pub generics: Generics<Box<Type<'a>>>,
}

/// Parses `unsigned? long long`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongLongType {
    pub unsigned: Option<term!(unsigned)>,
    pub long_long: (term!(long), term!(long)),
}

/// Parses `unsigned? long`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct LongType {
    pub unsigned: Option<term!(unsigned)>,
    pub long: term!(long),
}

/// Parses `unsigned? short`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ShortType {
    pub unsigned: Option<term!(unsigned)>,
    pub short: term!(short),
}

/// Parses `unsigned? short|long|(long long)`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntegerType {
    LongLong(LongLongType),
    Long(LongType),
    Short(ShortType),
}

/// Parses `unrestricted? float`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FloatType {
    pub unrestricted: Option<term!(unrestricted)>,
    pub float: term!(float),
}

/// Parses `unrestricted? double`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleType {
    pub unrestricted: Option<term!(unrestricted)>,
    pub double: term!(double),
}

/// Parses `unrestricted? float|double`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FloatingPointType {
    Float(FloatType),
    Double(DoubleType),
}

/// Parses `record<StringType, Type>`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RecordType<'a> {
    pub record: term!(record),
    pub generics: Generics<(RecordKeyType, term!(,), Box<AttributedType<'a>>)>,
}

/// Parses one of the string types `ByteString|DOMString|USVString` or any other type.
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(
    cut = "Record key must be one of: ByteString, DOMString, USVString with no extended attributes"
)]
pub enum RecordKeyType {
    Byte(term!(ByteString)),
    DOM(term!(DOMString)),
    USV(term!(USVString)),
}

/// Parses one of the member of a union type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(cut = "Expected a union member type that is not an `any` or `Promise`")]
pub enum UnionMemberType<'a> {
    Single(AttributedNonAnyType<'a>),
    Union(MayBeNull<UnionType<'a>>),
}

/// Parses a const type
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ConstType<'a> {
    Integer(IntegerType),
    FloatingPoint(FloatingPointType),
    Boolean(term!(boolean)),
    Byte(term!(byte)),
    Octet(term!(octet)),
    Bigint(term!(bigint)),
    Identifier(Identifier<'a>),
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
    pub type_: DistinguishableType<'a>,
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
        ConstType {
            Integer == "short",
            FloatingPoint == "float",
            Boolean == "boolean",
            Byte == "byte",
            Octet == "octet",
            Bigint == "bigint",
            Identifier == "name",
        }
    );

    test_variants!(
        DistinguishableType {
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
            Undefined == "undefined",
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

    test!(err should_not_parse_record_type_alt_types { "record<u64, short>" =>
        RecordType
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
            Promise == "Promise<long>",
            Distinguishable == "record<DOMString, short>",
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

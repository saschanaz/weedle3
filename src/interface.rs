use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Generics, Identifier, Parenthesized};
use crate::members::{
    AttributeInterfaceMember, ConstMember, OperationInterfaceMember, RegularOperationMember,
};
use crate::parser::eat::VariantToken;
use crate::types::AttributedType;

/// Parses interface members
pub type InterfaceMembers<'a> = Vec<InterfaceMember<'a>>;
pub type CallbackInterfaceMembers<'a> = Vec<CallbackInterfaceMember<'a>>;

/// Parses inheritance clause `: identifier`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inheritance<'a> {
    pub colon: term!(:),
    #[weedle(cut = "Missing name for inheritance")]
    pub identifier: VariantToken<'a, Identifier<'a>>,
}

/// Parses `[attributes]? constructor(( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct ConstructorInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub constructor: term!(constructor),
    pub args: Parenthesized<'a, ArgumentList<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleTypedIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub iterable: term!(iterable),
    pub generics: Generics<'a, AttributedType<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype, attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleTypedIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub iterable: term!(iterable),
    pub generics: Generics<'a, (AttributedType<'a>, term!(,), AttributedType<'a>)>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? (iterable<attributedtype> | iterable<attributedtype, attributedtype>) ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub enum IterableInterfaceMember<'a> {
    Single(SingleTypedIterable<'a>),
    Double(DoubleTypedIterable<'a>),
}

/// Parses an async iterable declaration `[attributes]? async iterable<attributedtype> (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleTypedAsyncIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub async_iterable: (term!(async), term!(iterable)),
    pub generics: Generics<'a, AttributedType<'a>>,
    pub args: Option<Parenthesized<'a, ArgumentList<'a>>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses an async iterable declaration `[attributes]? async iterable<attributedtype, attributedtype> (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleTypedAsyncIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub async_iterable: (term!(async), term!(iterable)),
    pub generics: Generics<'a, (AttributedType<'a>, term!(,), AttributedType<'a>)>,
    pub args: Option<Parenthesized<'a, ArgumentList<'a>>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses an async iterable declaration `[attributes]? async (iterable<attributedtype> | iterable<attributedtype, attributedtype>) (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub enum AsyncIterableInterfaceMember<'a> {
    Single(SingleTypedAsyncIterable<'a>),
    Double(DoubleTypedAsyncIterable<'a>),
}

/// Parses an maplike declaration `[attributes]? readonly? maplike<attributedtype, attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct MaplikeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: Option<term!(readonly)>,
    pub maplike: term!(maplike),
    pub generics: Generics<'a, (AttributedType<'a>, term!(,), AttributedType<'a>)>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct SetlikeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: Option<term!(readonly)>,
    pub setlike: term!(setlike),
    pub generics: Generics<'a, AttributedType<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `stringifier;`
#[derive(Weedle, Default, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct StringifierMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: term!(stringifier),
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses one of the interface member variants
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InterfaceMember<'a> {
    Const(ConstMember<'a>),
    Attribute(AttributeInterfaceMember<'a>),
    Constructor(ConstructorInterfaceMember<'a>),
    Operation(OperationInterfaceMember<'a>),
    Iterable(IterableInterfaceMember<'a>),
    AsyncIterable(AsyncIterableInterfaceMember<'a>),
    Maplike(MaplikeInterfaceMember<'a>),
    Setlike(SetlikeInterfaceMember<'a>),
    Stringifier(StringifierMember<'a>),
}

/// Parses one of the interface member variants
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CallbackInterfaceMember<'a> {
    Const(ConstMember<'a>),
    Operation(RegularOperationMember<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{parser::eat::VariantToken, Parse};

    test!(should_parse_stringifier_member { "stringifier;" =>
        "";
        StringifierMember;
    });

    test!(should_parse_setlike_interface_member { "readonly setlike<long>;" =>
        "";
        SetlikeInterfaceMember;
        attributes.is_none();
        readonly == Some(VariantToken::default());
    });

    test!(should_parse_maplike_interface_member { "readonly maplike<long, short>;" =>
        "";
        MaplikeInterfaceMember;
        attributes.is_none();
        readonly == Some(VariantToken::default());
    });

    test!(should_parse_double_typed_iterable { "iterable<long, long>;" =>
        "";
        DoubleTypedIterable;
        attributes.is_none();
    });

    test!(should_parse_single_typed_iterable { "iterable<long>;" =>
        "";
        SingleTypedIterable;
        attributes.is_none();
    });

    test!(should_parse_double_typed_async_iterable { "async iterable<long, long>;" =>
        "";
        DoubleTypedAsyncIterable;
        attributes.is_none();
        args.is_none();
    });

    test!(should_parse_double_typed_async_iterable_with_args { "async iterable<long, long>(long a);" =>
        "";
        DoubleTypedAsyncIterable;
        attributes.is_none();
        args.is_some();
    });

    test!(should_parse_single_typed_async_iterable { "async iterable<long>;" =>
        "";
        SingleTypedAsyncIterable;
        attributes.is_none();
        args.is_none();
    });

    test!(should_parse_single_typed_async_iterable_with_args { "async iterable<long>(long a);" =>
        "";
        SingleTypedAsyncIterable;
        attributes.is_none();
        args.is_some();
    });

    test!(should_parse_constructor_interface_member { "constructor(long a);" =>
        "";
        ConstructorInterfaceMember;
        attributes.is_none();
    });
}

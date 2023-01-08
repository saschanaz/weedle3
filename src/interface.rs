use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Generics, Identifier, Parenthesized};
use crate::literal::ConstValue;
use crate::types::{AttributedType, ConstType, ReturnType};
use crate::{eat, term};

/// Parses interface members
pub type InterfaceMembers<'a> = Vec<InterfaceMember<'a>>;

/// Parses inheritance clause `: identifier`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Inheritance<'a> {
    pub colon: term!(:),
    pub identifier: Identifier<'a>,
}

/// Parses a const interface member `[attributes]? const type identifier = value;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub const_: term!(const),
    pub const_type: ConstType<'a>,
    pub identifier: Identifier<'a>,
    pub assign: term!(=),
    pub const_value: ConstValue<'a>,
    pub semi_colon: term!(;),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeName<'a>(&'a str);

macro_rules! try_eat_attr {
    ($input:ident, $variant:ident) => {
        if let Ok((tokens, result)) = crate::eat_key!($variant)($input) {
            return Ok((tokens, AttributeName(result.value())));
        }
    };
}

impl<'slice, 'a> crate::Parse<'slice, 'a> for AttributeName<'a> {
    fn parse(
        input: crate::parser::Tokens<'slice, 'a>,
    ) -> nom::IResult<crate::parser::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Id)(input) {
            return Ok((tokens, AttributeName(result.0)));
        }
        try_eat_attr!(input, Async);
        try_eat_attr!(input, Required);

        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Char,
        }))
    }
}

/// Parses `[attributes]? (stringifier|inherit|static)? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<StringifierOrInheritOrStatic>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    pub identifier: AttributeName<'a>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? constructor(( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstructorInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub constructor: term!(constructor),
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationName<'a>(&'a str);

macro_rules! try_eat_op {
    ($input:ident, $variant:ident) => {
        if let Ok((tokens, result)) = crate::eat_key!($variant)($input) {
            return Ok((tokens, OperationName(result.value())));
        }
    };
}

impl<'slice, 'a> crate::Parse<'slice, 'a> for OperationName<'a> {
    fn parse(
        input: crate::parser::Tokens<'slice, 'a>,
    ) -> nom::IResult<crate::parser::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Id)(input) {
            return Ok((tokens, OperationName(result.0)));
        }
        try_eat_op!(input, Includes);

        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Char,
        }))
    }
}

/// Parses `[attributes]? (stringifier|static)? special? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<StringifierOrStatic>,
    pub special: Option<Special>,
    pub return_type: ReturnType<'a>,
    pub identifier: Option<OperationName<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleTypedIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub iterable: term!(iterable),
    pub generics: Generics<AttributedType<'a>>,
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? iterable<attributedtype, attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleTypedIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub iterable: term!(iterable),
    pub generics: Generics<(AttributedType<'a>, term!(,), AttributedType<'a>)>,
    pub semi_colon: term!(;),
}

/// Parses an iterable declaration `[attributes]? (iterable<attributedtype> | iterable<attributedtype, attributedtype>) ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IterableInterfaceMember<'a> {
    Single(SingleTypedIterable<'a>),
    Double(DoubleTypedIterable<'a>),
}

/// Parses an async iterable declaration `[attributes]? async iterable<attributedtype> (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleTypedAsyncIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub async_iterable: (term!(async), term!(iterable)),
    pub generics: Generics<AttributedType<'a>>,
    pub args: Option<Parenthesized<ArgumentList<'a>>>,
    pub semi_colon: term!(;),
}

/// Parses an async iterable declaration `[attributes]? async iterable<attributedtype, attributedtype> (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DoubleTypedAsyncIterable<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub async_iterable: (term!(async), term!(iterable)),
    pub generics: Generics<(AttributedType<'a>, term!(,), AttributedType<'a>)>,
    pub args: Option<Parenthesized<ArgumentList<'a>>>,
    pub semi_colon: term!(;),
}

/// Parses an async iterable declaration `[attributes]? async (iterable<attributedtype> | iterable<attributedtype, attributedtype>) (( args ))? ;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AsyncIterableInterfaceMember<'a> {
    Single(SingleTypedAsyncIterable<'a>),
    Double(DoubleTypedAsyncIterable<'a>),
}

/// Parses an maplike declaration `[attributes]? readonly? maplike<attributedtype, attributedtype>;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MaplikeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: Option<term!(readonly)>,
    pub maplike: term!(maplike),
    pub generics: Generics<(AttributedType<'a>, term!(,), AttributedType<'a>)>,
    pub semi_colon: term!(;),
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SetlikeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: Option<term!(readonly)>,
    pub setlike: term!(setlike),
    pub generics: Generics<AttributedType<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `stringifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringifierMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: term!(stringifier),
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

/// Parses one of the special keyword `getter|setter|deleter`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Special {
    Getter(term!(getter)),
    Setter(term!(setter)),
    Deleter(term!(deleter)),
}

/// Parses `stringifier|inherit|static`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringifierOrInheritOrStatic {
    Stringifier(term!(stringifier)),
    Inherit(term!(inherit)),
    Static(term!(static)),
}

/// Parses `stringifier|static`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringifierOrStatic {
    Stringifier(term!(stringifier)),
    Static(term!(static)),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{lexer::keywords, Parse};

    test!(should_parse_stringifier_member { "stringifier;" =>
        "";
        StringifierMember;
    });

    test!(should_parse_stringifier_or_static { "static" =>
        "";
        StringifierOrStatic;
    });

    test!(should_parse_stringifier_or_inherit_or_static { "inherit" =>
        "";
        StringifierOrInheritOrStatic;
    });

    test!(should_parse_setlike_interface_member { "readonly setlike<long>;" =>
        "";
        SetlikeInterfaceMember;
        attributes.is_none();
        readonly == Some(keywords::ReadOnly);
    });

    test!(should_parse_maplike_interface_member { "readonly maplike<long, short>;" =>
        "";
        MaplikeInterfaceMember;
        attributes.is_none();
        readonly == Some(keywords::ReadOnly);
    });

    test!(should_parse_attribute_interface_member { "readonly attribute unsigned long width;" =>
        "";
        AttributeInterfaceMember;
        attributes.is_none();
        readonly == Some(keywords::ReadOnly);
        identifier == AttributeName("width");
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

    test!(should_parse_operation_interface_member { "undefined readString(long a, long b);" =>
        "";
        OperationInterfaceMember;
        attributes.is_none();
        modifier.is_none();
        special.is_none();
        identifier.is_some();
    });

    test!(should_parse_const_member { "const long name = 5;" =>
        "";
        ConstMember;
        attributes.is_none();
        identifier.0 == "name";
    });
}

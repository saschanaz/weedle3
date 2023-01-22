use weedle_derive::Weedle;

use crate::{
    argument::ArgumentList,
    attribute::ExtendedAttributeList,
    common::{Identifier, Parenthesized},
    literal::ConstValue,
    tokens::{contextful_cut, Tokens},
    types::{AttributedType, ConstType, Type},
    VerboseResult,
};

/// Parses a const interface member `[attributes]? const type identifier = value;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct ConstMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub const_: term!(const),
    pub const_type: ConstType<'a>,
    pub identifier: Identifier<'a>,
    pub assign: term!(=),
    pub const_value: ConstValue<'a>,
    pub semi_colon: term!(;),
}

/// Parses `stringifier|inherit|static`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringifierOrInheritOrStatic {
    Stringifier(term!(stringifier)),
    #[weedle(post_check = "prevent_inherit_readonly")]
    Inherit(term!(inherit)),
    Static(term!(static)),
}

fn prevent_inherit_readonly<'slice, 'a>(
    input: Tokens<'slice, 'a>,
) -> VerboseResult<Tokens<'slice, 'a>, ()> {
    contextful_cut(
        "Inherited attributes cannot be read-only, as this form is only used to override the setter of the ancestor's attribute",
        nom::combinator::not(nom::combinator::peek(eat_key!(ReadOnly))),
    )(input)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct AttributeName<'a>(&'a str);

impl<'a> crate::Parse<'a> for AttributeName<'a> {
    fn parse_tokens<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, AttributeName(result.0)));
        }
        try_eat_keys!(AttributeName, input, Async, Required);
        nom::combinator::fail(input)
    }
}

impl<'a> From<AttributeName<'a>> for Identifier<'a> {
    fn from(value: AttributeName<'a>) -> Self {
        Self(value.0)
    }
}

/// Parses `[attributes]? (stringifier|inherit|static)? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct AttributeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<StringifierOrInheritOrStatic>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    #[weedle(from = "AttributeName")]
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? stringifier? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct AttributeMixinMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: Option<term!(stringifier)>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    #[weedle(from = "AttributeName")]
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses `[attribute]? readonly attributetype type identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct AttributeNamespaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: term!(readonly),
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    #[weedle(from = "AttributeName")]
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses one of the special keyword `getter|setter|deleter` or `static`.
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Modifier {
    Getter(term!(getter)),
    Setter(term!(setter)),
    Deleter(term!(deleter)),
    Static(term!(static)),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct OperationName<'a>(&'a str);

impl<'a> crate::Parse<'a> for OperationName<'a> {
    fn parse_tokens<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, OperationName(result.0)));
        }
        try_eat_keys!(OperationName, input, Includes);
        nom::combinator::fail(input)
    }
}

impl<'a> From<OperationName<'a>> for Identifier<'a> {
    fn from(value: OperationName<'a>) -> Self {
        Self(value.0)
    }
}

/// Parses `[attributes]? (stringifier|static)? special? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct OperationInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<Modifier>,
    pub return_type: Type<'a>,
    #[weedle(from = "OperationName", opt)]
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct RegularOperationMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub return_type: Type<'a>,
    #[weedle(from = "OperationName", opt)]
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_const_member { "const long name = 5;" =>
        "";
        ConstMember;
        attributes.is_none();
        identifier.0 == "name";
    });

    test!(should_parse_stringifier_or_inherit_or_static { "inherit" =>
        "";
        StringifierOrInheritOrStatic;
    });

    test!(should_parse_attribute_interface_member { "static attribute unsigned long width;" =>
        "";
        AttributeInterfaceMember;
        attributes.is_none();
        modifier == Some(StringifierOrInheritOrStatic::Static(term!(static)));
        identifier.0 == "width";
    });

    test!(should_parse_attribute_mixin_member { "stringifier readonly attribute short name;" =>
        "";
        AttributeMixinMember;
        attributes.is_none();
        stringifier.is_some();
        readonly.is_some();
        identifier.0 == "name";
    });

    test!(should_parse_attribute_namespace_member { "readonly attribute short name;" =>
        "";
        AttributeNamespaceMember;
        attributes.is_none();
        identifier.0 == "name";
    });

    test!(should_parse_modifier { "static" =>
        "";
        Modifier;
    });

    test!(should_parse_operation_interface_member { "undefined readString(long a, long b);" =>
        "";
        OperationInterfaceMember;
        attributes.is_none();
        modifier.is_none();
        identifier.is_some();
    });

    test!(should_parse_regular_operation_member { "short (long a, long b);" =>
        "";
        RegularOperationMember;
        attributes.is_none();
        identifier.is_none();
    });
}

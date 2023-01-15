use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Identifier, Parenthesized};
use crate::members::ConstMember;
use crate::types::{AttributedType, Type};

/// Parses namespace members declaration
pub type NamespaceMembers<'a> = Vec<NamespaceMember<'a>>;

/// Parses `[attributes]? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationNamespaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub return_type: Type<'a>,
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `[attribute]? readonly attributetype type identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeNamespaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: term!(readonly),
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses namespace member declaration
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NamespaceMember<'a> {
    Operation(OperationNamespaceMember<'a>),
    Attribute(AttributeNamespaceMember<'a>),
    Const(ConstMember<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_attribute_namespace_member { "readonly attribute short name;" =>
        "";
        AttributeNamespaceMember;
        attributes.is_none();
        identifier.0 == "name";
    });

    test!(should_parse_operation_namespace_member { "short (long a, long b);" =>
        "";
        OperationNamespaceMember;
        attributes.is_none();
        identifier.is_none();
    });
}

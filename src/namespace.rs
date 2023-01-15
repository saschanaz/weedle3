use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Identifier, Parenthesized};
use crate::members::ConstMember;
use crate::types::{AttributedType, Type};

/// Parses namespace members declaration
pub type NamespaceMembers<'a> = Vec<NamespaceMember<'a>>;

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

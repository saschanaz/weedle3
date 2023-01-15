use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Identifier, Parenthesized};
use crate::interface::StringifierMember;
use crate::members::ConstMember;
use crate::types::{AttributedType, Type};

/// Parses the members declarations of a mixin
pub type MixinMembers<'a> = Vec<MixinMember<'a>>;

/// Parses one of the variants of a mixin member
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MixinMember<'a> {
    Const(ConstMember<'a>),
    Operation(OperationMixinMember<'a>),
    Attribute(AttributeMixinMember<'a>),
    Stringifier(StringifierMember<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_attribute_mixin_member { "stringifier readonly attribute short name;" =>
        "";
        AttributeMixinMember;
        attributes.is_none();
        stringifier.is_some();
        readonly.is_some();
        identifier.0 == "name";
    });

    test!(should_parse_operation_mixin_member { "short fnName(long a);" =>
        "";
        OperationMixinMember;
        attributes.is_none();
        identifier.is_some();
    });
}

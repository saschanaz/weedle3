use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::attribute::ExtendedAttributeList;
use crate::common::{Identifier, Parenthesized};
use crate::interface::{ConstMember, StringifierMember};
use crate::lex_term;
use crate::types::{AttributedType, ReturnType};

/// Parses the members declarations of a mixin
pub type MixinMembers<'a> = Vec<MixinMember<'a>>;

/// Parses `[attributes]? stringifier? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationMixinMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: Option<lex_term!(stringifier)>,
    pub return_type: ReturnType<'a>,
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<'a, ArgumentList<'a>>,
    pub semi_colon: lex_term!(;),
}

/// Parses `[attributes]? stringifier? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeMixinMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: Option<lex_term!(stringifier)>,
    pub readonly: Option<lex_term!(readonly)>,
    pub attribute: lex_term!(attribute),
    pub type_: AttributedType<'a>,
    pub identifier: Identifier<'a>,
    pub semi_colon: lex_term!(;),
}

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
        stringifier.is_none();
        identifier.is_some();
    });
}

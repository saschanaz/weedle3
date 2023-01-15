use weedle_derive::Weedle;

use crate::members::{AttributeNamespaceMember, ConstMember, RegularOperationMember};

/// Parses namespace members declaration
pub type NamespaceMembers<'a> = Vec<NamespaceMember<'a>>;

/// Parses namespace member declaration
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NamespaceMember<'a> {
    Operation(RegularOperationMember<'a>),
    Attribute(AttributeNamespaceMember<'a>),
    Const(ConstMember<'a>),
}

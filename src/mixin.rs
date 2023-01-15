use weedle_derive::Weedle;

use crate::interface::StringifierMember;
use crate::members::{AttributeMixinMember, ConstMember, RegularOperationMember};

/// Parses the members declarations of a mixin
pub type MixinMembers<'a> = Vec<MixinMember<'a>>;

/// Parses one of the variants of a mixin member
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MixinMember<'a> {
    Const(ConstMember<'a>),
    Operation(RegularOperationMember<'a>),
    Attribute(AttributeMixinMember<'a>),
    Stringifier(StringifierMember<'a>),
}

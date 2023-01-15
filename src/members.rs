use weedle_derive::Weedle;

use crate::{
    attribute::ExtendedAttributeList, common::Identifier, literal::ConstValue, types::ConstType,
};

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

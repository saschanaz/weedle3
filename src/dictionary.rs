use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier};
use crate::parser::eat::VariantToken;
use crate::term;
use crate::types::Type;

/// Parses dictionary members
pub type DictionaryMembers<'a> = Vec<DictionaryMember<'a>>;

/// Parses dictionary member `[attributes]? required? type identifier ( = default )?;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub required: Option<term!(required)>,
    pub type_: Type<'a>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub default: Option<Default<'a>>,
    pub semi_colon: term!(;),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_dictionary_member { "required long num = 5;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_some();
        identifier.variant.0 == "num";
        default.is_some();
    });
}

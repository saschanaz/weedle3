use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier};
use crate::term;
use crate::term::Token;
use crate::types::Type;

/// Parses dictionary members
pub type DictionaryMembers<'a> = Vec<DictionaryMember<'a>>;

/// Parses dictionary member `[attributes]? required? type identifier ( = default )?;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct DictionaryMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub required: Option<term!(required)>,
    pub type_: Type<'a>,
    #[weedle(cut = "Missing name")]
    pub identifier: Token<'a, Identifier<'a>>,
    #[weedle(cond = "required.is_none()")]
    pub default: Option<Default<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Parse;

    test!(should_parse_dictionary_member { "required long num;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_some();
        identifier.value.0 == "num";
        default.is_none();
    });

    test!(should_parse_required_dictionary_member { "long num = 5;" =>
        "";
        DictionaryMember;
        attributes.is_none();
        required.is_none();
        identifier.value.0 == "num";
        default.is_some();
    });
}

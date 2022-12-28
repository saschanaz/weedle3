// https://webidl.spec.whatwg.org/#prod-StringType

use nom::{IResult, Parser};

use crate::lexer::keywords;
use crate::parser::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum StringType<'a> {
    Byte(VariantToken<'a, keywords::ByteString<'a>>),
    Dom(VariantToken<'a, keywords::DOMString<'a>>),
    Usv(VariantToken<'a, keywords::USVString<'a>>),
}

impl StringType<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, StringType<'token>> {
        nom::branch::alt((
            eat_key!(ByteString).map(StringType::Byte),
            eat_key!(DOMString).map(StringType::Dom),
            eat_key!(USVString).map(StringType::Usv),
        ))(tokens)
    }
}

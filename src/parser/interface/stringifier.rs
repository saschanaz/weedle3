// https://webidl.spec.whatwg.org/#prod-Stringifier

use nom::IResult;

use crate::{
    lexer::keywords,
    parser::{eat::VariantToken, impl_nom_traits::Tokens},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StringifierOperation<'a> {
    stringifier: VariantToken<'a, keywords::Stringifier<'a>>,
    semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl StringifierOperation<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, StringifierOperation<'token>> {
        let (tokens, (stringifier, semi_colon)) =
            nom::sequence::tuple((eat_key!(Stringifier), eat_key!(SemiColon)))(tokens)?;

        Ok((
            tokens,
            StringifierOperation {
                stringifier,
                semi_colon,
            },
        ))
    }
}

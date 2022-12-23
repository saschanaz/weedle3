use nom::IResult;

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::{
    common::Identifier,
    lexer::{keywords::Keyword, Tag, Token},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IncludesStatement<'a> {
    pub target: VariantToken<'a, Identifier<'a>>,
    pub includes: VariantToken<'a, &'a str>,
    pub mixin: VariantToken<'a, Identifier<'a>>,
    pub termination: VariantToken<'a, &'a str>,
}

pub fn includes_statement<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IncludesStatement<'token>> {
    let (remaining, (target, includes, mixin, termination)) =
        nom::sequence::tuple((eat!(Id), eatKey!(Includes), eat!(Id), eatKey!(SemiColon)))(tokens)?;

    Ok((
        remaining,
        IncludesStatement {
            target,
            includes,
            mixin,
            termination,
        },
    ))
}

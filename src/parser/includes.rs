use nom::IResult;

use super::impl_nom_traits::Tokens;
use crate::lexer::{keywords::Keyword, Tag, Token};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IncludesStatement<'a> {
    pub target: Token<'a>,
    pub includes: Token<'a>,
    pub mixin: Token<'a>,
    pub termination: Token<'a>,
}

pub fn includes_statement<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IncludesStatement<'token>> {
    let (remaining, (target, includes, mixin, termination)) = nom::sequence::tuple((
        eat!(Tag::Id(_)),
        eat!(Tag::Kw(Keyword::Includes(_))),
        eat!(Tag::Id(_)),
        eat!(Tag::Kw(Keyword::SemiColon(_))),
    ))(tokens)?;

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

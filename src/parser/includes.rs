use nom::IResult;

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::{
    common::Identifier,
    lexer::{
        keywords::{Includes, SemiColon},
        Token,
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IncludesStatement<'a> {
    pub target: VariantToken<'a, Identifier<'a>>,
    pub includes: VariantToken<'a, Includes<'a>>,
    pub mixin: VariantToken<'a, Identifier<'a>>,
    pub termination: VariantToken<'a, SemiColon<'a>>,
}

pub fn includes_statement<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IncludesStatement<'token>> {
    let (remaining, (target, includes, mixin, termination)) =
        nom::sequence::tuple((eat!(Id), eat_key!(Includes), eat!(Id), eat_key!(SemiColon)))(
            tokens,
        )?;

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

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        interface_mixin,
        includes_statement,
        "Foo includes Bar;",
        IncludesStatement {
            target: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            mixin: VariantToken {
                variant: Identifier("Bar"),
                ..
            },
            ..
        }
    );
}

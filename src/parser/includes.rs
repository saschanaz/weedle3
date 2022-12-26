use nom::IResult;

use super::{eat::VariantToken, impl_nom_traits::Tokens};
use crate::{
    common::Identifier,
    lexer::keywords::{Includes, SemiColon},
};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IncludesStatementDefinition<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub includes: VariantToken<'a, Includes<'a>>,
    pub rhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub semi_colon: VariantToken<'a, SemiColon<'a>>,
}

pub fn includes_statement<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, IncludesStatementDefinition<'token>> {
    let (remaining, (lhs_identifier, includes, rhs_identifier, termination)) =
        nom::sequence::tuple((eat!(Id), eat_key!(Includes), eat!(Id), eat_key!(SemiColon)))(
            tokens,
        )?;

    Ok((
        remaining,
        IncludesStatementDefinition {
            lhs_identifier,
            includes,
            rhs_identifier,
            semi_colon: termination,
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
        IncludesStatementDefinition {
            lhs_identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            rhs_identifier: VariantToken {
                variant: Identifier("Bar"),
                ..
            },
            ..
        }
    );
}

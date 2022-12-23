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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        lexer::{lex, Tag},
        parser::impl_nom_traits::Tokens,
    };

    #[test]
    fn interface_mixin() {
        let tokens = lex("Foo includes Bar;").unwrap();
        let (unread, result) = includes_statement(Tokens(&tokens[..])).unwrap();

        assert!(matches!(unread.0[0].tag, Tag::Eof(_)));
        assert_eq!(result.target.variant.0, "Foo");
        assert_eq!(result.includes.variant.0, "includes");
        assert_eq!(result.mixin.variant.0, "Bar");
        assert_eq!(result.termination.variant.0, ";");
    }
}

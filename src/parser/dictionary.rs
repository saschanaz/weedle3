use nom::IResult;

use crate::{common::Identifier, lexer::keywords};

use super::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Dictionary<'a> {
    base: VariantToken<'a, keywords::Dictionary<'a>>,
    name: VariantToken<'a, Identifier<'a>>,
    open: VariantToken<'a, keywords::OpenBrace<'a>>,
    close: VariantToken<'a, keywords::CloseBrace<'a>>,
    termination: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn dictionary<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, Dictionary<'token>> {
    // TODO: fill more things
    let (tokens, (base, name, open, close, termination)) = nom::sequence::tuple((
        eat_key!(Dictionary),
        eat!(Id),
        eat_key!(OpenBrace),
        eat_key!(CloseBrace),
        eat_key!(SemiColon),
    ))(tokens)?;

    Ok((
        tokens,
        Dictionary {
            base,
            name,
            open,
            close,
            termination,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        empty_dictionary,
        dictionary,
        "dictionary Foo {};",
        Dictionary {
            name: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );
}

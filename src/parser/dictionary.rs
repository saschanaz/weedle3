use nom::IResult;

use crate::{common::Identifier, lexer::keywords};

use super::{eat::VariantToken, impl_nom_traits::Tokens};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DictionaryDefinition<'a> {
    dictionary: VariantToken<'a, keywords::Dictionary<'a>>,
    identifier: VariantToken<'a, Identifier<'a>>,
    open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
    semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn dictionary<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, DictionaryDefinition<'token>> {
    // TODO: fill more things
    let (tokens, (dictionary, identifier, open_brace, close_brace, semi_colon)) =
        nom::sequence::tuple((
            eat_key!(Dictionary),
            eat!(Id),
            eat_key!(OpenBrace),
            eat_key!(CloseBrace),
            eat_key!(SemiColon),
        ))(tokens)?;

    Ok((
        tokens,
        DictionaryDefinition {
            dictionary,
            identifier,
            open_brace,
            close_brace,
            semi_colon,
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
        DictionaryDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );
}

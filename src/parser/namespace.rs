// https://webidl.spec.whatwg.org/#prod-Namespace

use nom::{IResult, Parser};

use crate::{common::Identifier, lexer::keywords};

use super::{
    eat::VariantToken,
    extended_attributes::ExtendedAttributeList,
    impl_nom_traits::Tokens,
    interface::{const_member, ConstMember},
};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum NamespaceMember<'a> {
    Const(ConstMember<'a>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NamespaceDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub namespace: VariantToken<'a, keywords::Namespace<'a>>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    pub body: Vec<NamespaceMember<'a>>,
    pub close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

pub fn namespace_member<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, NamespaceMember<'token>> {
    nom::branch::alt((const_member.map(NamespaceMember::Const),))(tokens)
}

pub fn namespace<'slice, 'token>(
    tokens: Tokens<'slice, 'token>,
) -> IResult<Tokens<'slice, 'token>, NamespaceDefinition<'token>> {
    // TODO: fill more things
    let (tokens, (namespace, identifier, open_brace, members, semi_colon)) =
        nom::sequence::tuple((
            eat_key!(Namespace),
            nom::combinator::cut(eat!(Id)),
            nom::combinator::cut(eat_key!(OpenBrace)),
            nom::multi::many_till(namespace_member, eat_key!(CloseBrace)),
            nom::combinator::cut(eat_key!(SemiColon)),
        ))(tokens)?;

    Ok((
        tokens,
        NamespaceDefinition {
            ext_attrs: None,
            namespace,
            identifier,
            open_brace,
            body: members.0,
            close_brace: members.1,
            semi_colon,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        empty,
        namespace,
        "namespace Foo {};",
        NamespaceDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        single_member,
        namespace,
        "namespace Foo {
          const short bar = 42;
        };",
        NamespaceDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            body,
            ..
        } if matches!(&body[..], [NamespaceMember::Const(_)])
    );
}

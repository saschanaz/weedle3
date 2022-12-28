// https://webidl.spec.whatwg.org/#prod-InterfaceRest

mod const_member;
pub use const_member::ConstMember;
mod stringifier;
pub use stringifier::StringifierOperation;

use nom::{IResult, Parser};

use crate::{common::Identifier, lexer::keywords};

use super::{
    eat::VariantToken, extended_attributes::ExtendedAttributeList, impl_nom_traits::Tokens,
};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InterfaceMember<'a> {
    Const(ConstMember<'a>),
    Stringifier(StringifierOperation<'a>),
}

impl InterfaceMember<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, InterfaceMember<'token>> {
        nom::branch::alt((
            ConstMember::parse.map(InterfaceMember::Const),
            StringifierOperation::parse.map(InterfaceMember::Stringifier),
        ))(tokens)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InterfaceDefinition<'a> {
    pub ext_attrs: Option<ExtendedAttributeList<'a>>,
    pub interface: VariantToken<'a, keywords::Interface<'a>>,
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub open_brace: VariantToken<'a, keywords::OpenBrace<'a>>,
    pub body: Vec<InterfaceMember<'a>>,
    pub close_brace: VariantToken<'a, keywords::CloseBrace<'a>>,
    pub semi_colon: VariantToken<'a, keywords::SemiColon<'a>>,
}

impl InterfaceDefinition<'_> {
    pub fn parse<'slice, 'token>(
        tokens: Tokens<'slice, 'token>,
    ) -> IResult<Tokens<'slice, 'token>, InterfaceDefinition<'token>> {
        // TODO: fill more things
        let (tokens, (interface, identifier, open_brace, members, semi_colon)) =
            nom::sequence::tuple((
                eat_key!(Interface),
                nom::combinator::cut(eat!(Id)),
                nom::combinator::cut(eat_key!(OpenBrace)),
                nom::multi::many_till(InterfaceMember::parse, eat_key!(CloseBrace)),
                nom::combinator::cut(eat_key!(SemiColon)),
            ))(tokens)?;

        Ok((
            tokens,
            InterfaceDefinition {
                ext_attrs: None,
                interface,
                identifier,
                open_brace,
                body: members.0,
                close_brace: members.1,
                semi_colon,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_match!(
        empty_interface,
        InterfaceDefinition::parse,
        "interface Foo {};",
        InterfaceDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            ..
        }
    );

    test_match!(
        single_member_interface,
        InterfaceDefinition::parse,
        "interface Foo { stringifier; };",
        InterfaceDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            body,
            ..
        } if matches!(&body[..], [InterfaceMember::Stringifier(_)])
    );

    test_match!(
        double_member_interface,
        InterfaceDefinition::parse,
        "interface Foo {
          const short bar = 42;
          stringifier;
        };",
        InterfaceDefinition {
            identifier: VariantToken {
                variant: Identifier("Foo"),
                ..
            },
            body,
            ..
        } if matches!(&body[..], [InterfaceMember::Const(_), InterfaceMember::Stringifier(_)])
    );

    test_result_match!(
        semi_colon_less,
        InterfaceDefinition::parse,
        "interface Foo {}",
        Err(nom::Err::Failure(_))
    );

    test_result_match!(
        body_less,
        InterfaceDefinition::parse,
        "interface Foo",
        Err(nom::Err::Failure(_))
    );
}

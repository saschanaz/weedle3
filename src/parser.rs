mod impl_nom_traits;
use impl_nom_traits::Tokens;

#[macro_use]
mod eat;
#[macro_use]
mod generate_match_test;

mod extended_attributes;
mod r#type;

mod callback;
mod dictionary;
mod enumeration;
mod includes;
mod interface;
mod namespace;
mod typedef;

use nom::{IResult, InputIter, Parser};

use crate::{
    lexer::{lex, Token},
    parser::{enumeration::r#enum, extended_attributes::extended_attribute_list},
};

use self::{
    callback::{callback, CallbackDefinition},
    dictionary::{dictionary, DictionaryDefinition},
    eat::VariantToken,
    enumeration::EnumDefinition,
    extended_attributes::ExtendedAttributeList,
    includes::{includes_statement, IncludesStatementDefinition},
    interface::{interface, InterfaceDefinition},
    namespace::{namespace, NamespaceDefinition},
    typedef::{typedef, TypedefDefinition},
};

#[derive(Debug)]
pub enum ErrorKind<'a> {
    Lexer(nom::Err<nom::error::Error<&'a str>>),
    Parser(nom::Err<nom::error::Error<Vec<Token<'a>>>>),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Definition<'a> {
    Callback(CallbackDefinition<'a>),
    Interface(InterfaceDefinition<'a>),
    Namespace(NamespaceDefinition<'a>),
    Dictionary(DictionaryDefinition<'a>),
    Enum(EnumDefinition<'a>),
    Typedef(TypedefDefinition<'a>),
    IncludesStatement(IncludesStatementDefinition<'a>),
    Eof(VariantToken<'a, ()>),
}

fn set_ext_attr<'a>(
    (ext_attrs, mut def): (Option<ExtendedAttributeList<'a>>, Definition<'a>),
) -> Definition<'a> {
    match &mut def {
        Definition::Callback(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Interface(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Namespace(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Dictionary(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Enum(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Typedef(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::IncludesStatement(d) => {
            d.ext_attrs = ext_attrs;
        }
        Definition::Eof(_) => panic!("Unexpected EOF"),
    }
    def
}

pub fn parse(input: &str) -> Result<Vec<Definition>, ErrorKind> {
    let tokens = lex(input).map_err(ErrorKind::Lexer)?;

    let (unread, (mut defs, eof)) = nom::sequence::tuple((
        nom::multi::many0(
            nom::sequence::tuple((
                nom::combinator::opt(extended_attribute_list),
                nom::branch::alt((
                    callback.map(Definition::Callback),
                    interface.map(Definition::Interface),
                    namespace.map(Definition::Namespace),
                    dictionary.map(Definition::Dictionary),
                    r#enum.map(Definition::Enum),
                    typedef.map(Definition::Typedef),
                    includes_statement.map(Definition::IncludesStatement),
                )),
            ))
            .map(set_ext_attr),
        ),
        nom::combinator::map(eat!(Eof), Definition::Eof),
    ))(Tokens(&tokens[..]))
    .map_err(|err| match err {
        nom::Err::Incomplete(need) => ErrorKind::Parser(nom::Err::Incomplete(need)),
        nom::Err::Error(err) => ErrorKind::Parser(nom::Err::Error(nom::error::Error {
            code: err.code,
            input: err.input.iter_elements().collect(),
        })),
        nom::Err::Failure(err) => ErrorKind::Parser(nom::Err::Failure(nom::error::Error {
            code: err.code,
            input: err.input.iter_elements().collect(),
        })),
    })?;

    // Cannot be empty here since eof would fail then
    assert!(unread.0.is_empty());

    defs.push(eof);

    Ok(defs)
}

#[cfg(test)]
mod tests {
    use crate::{
        common::Identifier,
        lexer::Tag,
        parser::extended_attributes::{ExtendedAttribute, ExtendedAttributeNoArgs},
    };

    use super::{impl_nom_traits::Tokens, *};

    #[test]
    fn test() {
        let (remaining, (id1, id2)) = nom::sequence::tuple((eat!(Id), eat!(Id)))(Tokens(&[
            Token {
                tag: Tag::Id(crate::common::Identifier("foo")),
                trivia: "",
            },
            Token {
                tag: Tag::Id(crate::common::Identifier("bar")),
                trivia: "",
            },
        ]))
        .unwrap();

        assert!(remaining.0.is_empty());
        assert_eq!(id1.variant.0, "foo", "id1 should be foo");
        assert_eq!(id2.variant.0, "bar", "id2 should be bar");
    }

    #[test]
    fn parse() {
        let result = super::parse("Foo includes Bar;dictionary Foo {};").unwrap();

        assert!(matches!(
            &result[..],
            [
                Definition::IncludesStatement(_),
                Definition::Dictionary(_),
                Definition::Eof(_),
            ]
        ));
    }

    #[test]
    fn parse_with_ext_attrs() {
        let result = super::parse("[Bar] dictionary Foo {};").unwrap();

        assert!(matches!(
            &result[..],
            [
                Definition::Dictionary(DictionaryDefinition {
                    ext_attrs: Some(ExtendedAttributeList {
                        body: ext_attrs,
                        ..
                    }),
                    identifier: VariantToken{
                        variant: Identifier("Foo"),
                        ..
                    },
                    ..
                }),
                Definition::Eof(_),
            ] if matches!(&ext_attrs[..], [
                ExtendedAttribute::NoArgs(ExtendedAttributeNoArgs(VariantToken {
                    variant: Identifier("Bar"),
                    ..
                }))
            ])
        ));
    }
}

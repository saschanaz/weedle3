use weedle_derive::Weedle;

use crate::{
    attribute::ExtendedAttributeList, common::Identifier, literal::ConstValue, types::ConstType,
};

/// Parses a const interface member `[attributes]? const type identifier = value;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub const_: term!(const),
    pub const_type: ConstType<'a>,
    pub identifier: Identifier<'a>,
    pub assign: term!(=),
    pub const_value: ConstValue<'a>,
    pub semi_colon: term!(;),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct AttributeName<'a>(&'a str);

impl<'a> crate::Parse<'a> for AttributeName<'a> {
    fn parse_tokens<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, AttributeName(result.0)));
        }
        try_eat_keys!(AttributeName, input, Async, Required);
        nom::combinator::fail(input)
    }
}

impl<'a> AttributeName<'a> {
    fn parse_to_id<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Identifier<'a>> {
        let (input, name) = weedle!(AttributeName)(input)?;
        Ok((input, Identifier(name.0)))
    }
}

/// Parses `[attributes]? (stringifier|inherit|static)? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<StringifierOrInheritOrStatic>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    #[weedle(parser = "AttributeName::parse_to_id")]
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? constructor(( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ConstructorInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub constructor: term!(constructor),
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct OperationName<'a>(&'a str);

impl<'a> crate::Parse<'a> for OperationName<'a> {
    fn parse_tokens<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, OperationName(result.0)));
        }
        try_eat_keys!(OperationName, input, Includes);
        nom::combinator::fail(input)
    }
}

impl<'a> OperationName<'a> {
    fn parse_to_id_opt<'slice>(
        input: crate::tokens::Tokens<'slice, 'a>,
    ) -> VerboseResult<crate::tokens::Tokens<'slice, 'a>, Option<Identifier<'a>>> {
        let (input, name) = weedle!(Option<OperationName>)(input)?;
        Ok((input, name.map(|n| Identifier(n.0))))
    }
}

/// Parses `[attributes]? (stringifier|static)? special? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationInterfaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub modifier: Option<StringifierOrStatic>,
    pub special: Option<Special>,
    pub return_type: Type<'a>,
    #[weedle(parser = "OperationName::parse_to_id_opt")]
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? stringifier? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationMixinMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: Option<term!(stringifier)>,
    pub return_type: Type<'a>,
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? stringifier? readonly? attribute attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeMixinMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub stringifier: Option<term!(stringifier)>,
    pub readonly: Option<term!(readonly)>,
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? returntype identifier? (( args ));`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OperationNamespaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub return_type: Type<'a>,
    pub identifier: Option<Identifier<'a>>,
    pub args: Parenthesized<ArgumentList<'a>>,
    pub semi_colon: term!(;),
}

/// Parses `[attribute]? readonly attributetype type identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttributeNamespaceMember<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub readonly: term!(readonly),
    pub attribute: term!(attribute),
    pub type_: AttributedType<'a>,
    pub identifier: Identifier<'a>,
    pub semi_colon: term!(;),
}

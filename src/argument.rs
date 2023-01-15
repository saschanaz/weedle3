use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier, Punctuated};
use crate::tokens::Tokens;
use crate::types::{AttributedType, Type};
use crate::{Parse, VerboseResult};

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList<'a> = Punctuated<Argument<'a>, term!(,)>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ArgumentName<'a>(&'a str);

impl<'a> Parse<'a> for ArgumentName<'a> {
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, ArgumentName(result.0)));
        }
        try_eat_keys!(
            ArgumentName,
            input,
            Async,
            Attribute,
            Callback,
            Const,
            Constructor,
            Deleter,
            Dictionary,
            Enum,
            Getter,
            Includes,
            Inherit,
            Interface,
            Iterable,
            Maplike,
            Mixin,
            Namespace,
            Partial,
            ReadOnly,
            Required,
            Setlike,
            Setter,
            Static,
            Stringifier,
            Typedef,
            Unrestricted
        );
        nom::combinator::fail(input)
    }
}

impl<'a> From<ArgumentName<'a>> for Identifier<'a> {
    fn from(value: ArgumentName<'a>) -> Self {
        Self(value.0)
    }
}

/// Parses `[attributes]? optional? attributedtype identifier ( = default )?`
///
/// Note: `= default` is only allowed if `optional` is present
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleArgument<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub optional: Option<term!(optional)>,
    pub type_: AttributedType<'a>,
    pub identifier: Identifier<'a>,
    pub default: Option<Default<'a>>,
}

impl<'a> Parse<'a> for SingleArgument<'a> {
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        let (input, (attributes, optional, type_, identifier)) = nom::sequence::tuple((
            weedle!(Option<ExtendedAttributeList<'a>>),
            weedle!(Option<term!(optional)>),
            weedle!(AttributedType<'a>),
            nom::combinator::into(weedle!(ArgumentName<'a>)),
        ))(input)?;
        let (input, default) = nom::combinator::map(
            nom::combinator::cond(optional.is_some(), weedle!(Option<Default<'a>>)),
            |default| default.unwrap_or(None),
        )(input)?;
        Ok((
            input,
            Self {
                attributes,
                optional,
                type_,
                identifier,
                default,
            },
        ))
    }
}

/// Parses `[attributes]? type... identifier`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VariadicArgument<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub type_: Type<'a>,
    pub ellipsis: term!(...),
    pub identifier: Identifier<'a>,
}

/// Parses an argument. Ex: `double v1|double... v1s`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Argument<'a> {
    Single(SingleArgument<'a>),
    Variadic(VariadicArgument<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::literal::{DecLit, DefaultValue, IntegerLit};
    use crate::Parse;

    test!(should_parse_single_argument { "short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_none();
        identifier.0 == "a";
        default.is_none();
    });

    test!(should_parse_variadic_argument { "short... a" =>
        "";
        VariadicArgument;
        attributes.is_none();
        identifier.0 == "a";
    });

    test!(should_parse_optional_single_argument { "optional short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier.0 == "a";
        default.is_none();
    });

    test!(should_parse_optional_single_argument_with_default { "optional short a = 5" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier.0 == "a";
        default == Some(Default {
            assign: term!(=),
            value: DefaultValue::Integer(IntegerLit::Dec(DecLit("5"))),
        });
    });

    test!(should_not_parse_default_if_not_optional { "short a = 5" =>
        "= 5";
        SingleArgument;
    });
}

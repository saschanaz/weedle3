use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier, Punctuated};
use crate::parser::Tokens;
use crate::types::{AttributedType, Type};
use crate::{eat, Parse};

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList<'a> = Punctuated<Argument<'a>, term!(,)>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArgumentName<'a>(&'a str);

macro_rules! try_eat_arg {
    ($input:ident, $variant:ident) => {
        if let Ok((tokens, result)) = crate::eat_key!($variant)($input) {
            return Ok((tokens, ArgumentName(result.value())));
        }
    };
}

impl<'slice, 'a> Parse<'slice, 'a> for ArgumentName<'a> {
    fn parse(input: Tokens<'slice, 'a>) -> nom::IResult<Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Id)(input) {
            return Ok((tokens, ArgumentName(result.0)));
        }
        try_eat_arg!(input, Async);
        try_eat_arg!(input, Attribute);
        try_eat_arg!(input, Callback);
        try_eat_arg!(input, Const);
        try_eat_arg!(input, Constructor);
        try_eat_arg!(input, Deleter);
        try_eat_arg!(input, Dictionary);
        try_eat_arg!(input, Enum);
        try_eat_arg!(input, Getter);
        try_eat_arg!(input, Includes);
        try_eat_arg!(input, Inherit);
        try_eat_arg!(input, Interface);
        try_eat_arg!(input, Iterable);
        try_eat_arg!(input, Maplike);
        try_eat_arg!(input, Mixin);
        try_eat_arg!(input, Namespace);
        try_eat_arg!(input, Partial);
        try_eat_arg!(input, ReadOnly);
        try_eat_arg!(input, Required);
        try_eat_arg!(input, Setlike);
        try_eat_arg!(input, Setter);
        try_eat_arg!(input, Static);
        try_eat_arg!(input, Stringifier);
        try_eat_arg!(input, Typedef);
        try_eat_arg!(input, Unrestricted);

        Err(nom::Err::Error(nom::error::Error {
            input,
            code: nom::error::ErrorKind::Char,
        }))
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
    pub identifier: ArgumentName<'a>,
    pub default: Option<Default<'a>>,
}

impl<'slice, 'a> Parse<'slice, 'a> for SingleArgument<'a> {
    fn parse(input: Tokens<'slice, 'a>) -> crate::IResult<Tokens<'slice, 'a>, Self> {
        let (input, (attributes, optional, type_, identifier)) = nom::sequence::tuple((
            weedle!(Option<ExtendedAttributeList<'a>>),
            weedle!(Option<term!(optional)>),
            weedle!(AttributedType<'a>),
            weedle!(ArgumentName<'a>),
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
        " = 5";
        SingleArgument;
    });
}

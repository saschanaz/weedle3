use weedle_derive::Weedle;

use crate::attribute::ExtendedAttributeList;
use crate::common::{Default, Identifier, Punctuated};
use crate::parser::eat::VariantToken;
use crate::tokens::Tokens;
use crate::types::{AttributedType, Type};
use crate::{Parse, VerboseResult};

/// Parses a list of argument. Ex: `double v1, double v2, double v3, optional double alpha`
pub type ArgumentList<'a> = Punctuated<Argument<'a>, term!(,)>;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ArgumentName<'a>(&'a str, &'a str);

impl<'a> Parse<'a> for ArgumentName<'a> {
    fn parse_tokens<'slice>(input: Tokens<'slice, 'a>) -> VerboseResult<Tokens<'slice, 'a>, Self> {
        if let Ok((tokens, result)) = eat!(Identifier)(input) {
            return Ok((tokens, ArgumentName(result.trivia, result.variant.0)));
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

impl<'a> From<ArgumentName<'a>> for VariantToken<'a, Identifier<'a>> {
    fn from(value: ArgumentName<'a>) -> Self {
        Self {
            trivia: value.0,
            variant: Identifier(value.1),
        }
    }
}

/// Parses `[attributes]? optional? attributedtype identifier ( = default )?`
///
/// Note: `= default` is only allowed if `optional` is present
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SingleArgument<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub optional: Option<term!(optional)>,
    pub type_: AttributedType<'a>,
    #[weedle(from = "ArgumentName")]
    pub identifier: VariantToken<'a, Identifier<'a>>,
    #[weedle(cond = "optional.is_some()")]
    pub default: Option<Default<'a>>,
}

/// Parses `[attributes]? type... identifier`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VariadicArgument<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub type_: Type<'a>,
    pub ellipsis: term!(...),
    pub identifier: VariantToken<'a, Identifier<'a>>,
}

/// Parses an argument. Ex: `double v1|double... v1s`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub enum Argument<'a> {
    Single(SingleArgument<'a>),
    Variadic(VariadicArgument<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::literal::{DecLit, DefaultValue, IntegerLit};
    use crate::parser::eat::VariantToken;
    use crate::Parse;

    test!(should_parse_single_argument { "short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_none();
        identifier == VariantToken { variant: Identifier("a"), trivia: " " };
        default.is_none();
    });

    test!(should_parse_variadic_argument { "short... a" =>
        "";
        VariadicArgument;
        attributes.is_none();
        identifier.variant.0 == "a";
    });

    test!(should_parse_optional_single_argument { "optional short a" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier == VariantToken { variant: Identifier("a"), trivia: " " };
        default.is_none();
    });

    test!(should_parse_optional_single_argument_with_default { "optional short a = 5" =>
        "";
        SingleArgument;
        attributes.is_none();
        optional.is_some();
        identifier == VariantToken { variant: Identifier("a"), trivia: " " };
        default == Some(Default {
            assign: VariantToken { variant: core::default::Default::default(), trivia: " " },
            value: DefaultValue::Integer(VariantToken { variant: IntegerLit::Dec(DecLit("5")), trivia: " " }),
        });
    });

    test!(should_not_parse_default_if_not_optional { "short a = 5" =>
        "= 5";
        SingleArgument;
    });
}

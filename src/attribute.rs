use weedle_derive::Weedle;

use crate::argument::ArgumentList;
use crate::common::{Bracketed, Identifier, Parenthesized, Punctuated};
use crate::literal::{FloatLit, IntegerLit, StringLit};
use crate::parser::eat::VariantToken;
use crate::term;

/// Parses a list of attributes. Ex: `[ attribute1, attribute2 ]`
pub type ExtendedAttributeList<'a> = Bracketed<'a, Punctuated<ExtendedAttribute<'a>, term!(,)>>;

/// Matches comma separated identifier list
pub type IdentifierList<'a> = Punctuated<VariantToken<'a, Identifier<'a>>, term!(,)>;
pub type StringList<'a> = Punctuated<VariantToken<'a, StringLit<'a>>, term!(,)>;
pub type FloatList<'a> = Punctuated<FloatLit<'a>, term!(,)>;
pub type IntegerList<'a> = Punctuated<VariantToken<'a, IntegerLit<'a>>, term!(,)>;

/// Parses an argument list. Ex: `Constructor((double x, double y))`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeArgList<'a> {
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub args: Parenthesized<'a, ArgumentList<'a>>,
}

/// Parses a named argument list. Ex: `NamedConstructor=Image((DOMString src))`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeNamedArgList<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub rhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub args: Parenthesized<'a, ArgumentList<'a>>,
}

/// Parses an identifier list. Ex: `Exposed=((Window,Worker))`
///
/// (( )) means ( ) chars
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeIdentList<'a> {
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub list: Parenthesized<'a, IdentifierList<'a>>,
}

/// Parses an attribute with an identifier. Ex: `PutForwards=name`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeIdent<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub rhs: VariantToken<'a, Identifier<'a>>,
}

/// Parses an attribute with a wildcard. Ex: `Exposed=*`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeWildcard<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub wildcard: term!(*),
}

// Things that are not used by the standard Web IDL, but still allowed
// and used by others e.g. Blink and JSDOM
// https://github.com/w3c/webidl2.js/issues/256
// https://github.com/w3c/webidl2.js/issues/455

/// Parses an attribute with a string. E: `ReflectOnly="on"`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeString<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub rhs: VariantToken<'a, StringLit<'a>>,
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeStringList<'a> {
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub list: Parenthesized<'a, StringList<'a>>,
}
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeFloat<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub rhs: FloatLit<'a>,
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeFloatList<'a> {
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub list: Parenthesized<'a, FloatList<'a>>,
}

#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeInteger<'a> {
    pub lhs_identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub rhs: VariantToken<'a, IntegerLit<'a>>,
}

#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeIntegerList<'a> {
    pub identifier: VariantToken<'a, Identifier<'a>>,
    pub assign: term!(=),
    pub list: Parenthesized<'a, IntegerList<'a>>,
}

/// Parses a plain attribute. Ex: `Replaceable`
#[derive(Weedle, Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExtendedAttributeNoArgs<'a>(pub VariantToken<'a, Identifier<'a>>);

/// Parses on of the forms of attribute
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExtendedAttribute<'a> {
    ArgList(ExtendedAttributeArgList<'a>),
    NamedArgList(ExtendedAttributeNamedArgList<'a>),
    IdentList(ExtendedAttributeIdentList<'a>),
    Ident(ExtendedAttributeIdent<'a>),
    Wildcard(ExtendedAttributeWildcard<'a>),
    String(ExtendedAttributeString<'a>),
    StringList(ExtendedAttributeStringList<'a>),
    Float(ExtendedAttributeFloat<'a>),
    FloatList(ExtendedAttributeFloatList<'a>),
    Integer(ExtendedAttributeInteger<'a>),
    IntegerList(ExtendedAttributeIntegerList<'a>),
    NoArgs(ExtendedAttributeNoArgs<'a>),
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{literal::FloatValueLit, Parse};

    test!(should_parse_attribute_no_args { "Replaceable" =>
        "";
        ExtendedAttributeNoArgs => ExtendedAttributeNoArgs(VariantToken {
            variant: Identifier("Replaceable"),
            trivia: "",
        })
    });

    test!(should_parse_attribute_arg_list { "Constructor(double x, double y)" =>
        "";
        ExtendedAttributeArgList;
        identifier.variant.0 == "Constructor";
        args.body.list.len() == 2;
    });

    test!(should_parse_attribute_ident { "PutForwards=name" =>
        "";
        ExtendedAttributeIdent;
        lhs_identifier.variant.0 == "PutForwards";
        rhs == VariantToken { variant: Identifier("name"), trivia: "" };
    });

    test!(should_parse_ident_list { "Exposed=(Window,Worker)" =>
        "";
        ExtendedAttributeIdentList;
        identifier.variant.0 == "Exposed";
        list.body.list.len() == 2;
    });

    test!(should_parse_named_arg_list { "NamedConstructor=Image(DOMString src)" =>
        "";
        ExtendedAttributeNamedArgList;
        lhs_identifier.variant.0 == "NamedConstructor";
        rhs_identifier.variant.0 == "Image";
        args.body.list.len() == 1;
    });

    test!(should_parse_string { "ReflectOnly=\"on\"" =>
        "";
        ExtendedAttributeString;
        lhs_identifier.variant.0 == "ReflectOnly";
        rhs.variant.0 == "on";
    });

    test!(should_parse_float { "FloatAttr=3.14" =>
        "";
        ExtendedAttributeFloat;
        lhs_identifier.variant.0 == "FloatAttr";
        rhs == FloatLit::Value(VariantToken { variant: FloatValueLit("3.14"), trivia: "" });
    });

    test!(should_parse_extattr_list { "[IntAttr=0, FloatAttr=3.14]" =>
        "";
        ExtendedAttributeList;
        body.list.len() == 2;
    });
}

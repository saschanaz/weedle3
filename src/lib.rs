//! Weedle - A WebIDL Parser
//!
//! Parses valid WebIDL definitions & produces a data structure starting from
//! [`Definitions`](struct.Definitions.html).
//!
//! ### Example
//!
//! ```
//! extern crate weedle;
//!
//! let parsed = weedle::parse("
//!     interface Window {
//!         readonly attribute Storage sessionStorage;
//!     };
//! ").unwrap();
//! println!("{:?}", parsed);
//! ```
//!
//! Note:
//! This parser follows the grammar given at [WebIDL](https://heycam.github.io/webidl).
//!
//! If any flaws found when parsing string with a valid grammar, create an issue.

use crate::tokens::contextful_cut;

use self::argument::ArgumentList;
use self::attribute::ExtendedAttributeList;
use self::common::{Braced, Identifier, Parenthesized, PunctuatedNonEmpty};
use self::dictionary::DictionaryMembers;
use self::interface::{CallbackInterfaceMembers, Inheritance, InterfaceMembers};
use self::literal::StringLit;
use self::mixin::MixinMembers;
use self::namespace::NamespaceMembers;
use self::types::{AttributedType, Type};
use weedle_derive::Weedle;

#[macro_use]
mod macros;
mod whitespace;
#[macro_use]
pub mod term;
pub mod argument;
pub mod attribute;
pub mod common;
pub mod dictionary;
pub mod interface;
pub mod literal;
pub mod members;
pub mod mixin;
pub mod namespace;
pub mod types;

mod lexer;
mod tokens;

use lexer::lex;
use tokens::Tokens;

type VerboseResult<I, O> = nom::IResult<I, O, nom::error::VerboseError<I>>;

/// A convenient parse function
///
/// ### Example
///
/// ```
/// extern crate weedle;
///
/// let parsed = weedle::parse("
///     interface Window {
///         readonly attribute Storage sessionStorage;
///     };
/// ").unwrap();
///
/// println!("{:?}", parsed);
/// ```
pub fn parse(
    input: &'_ str,
) -> Result<Definitions<'_>, nom::Err<nom::error::VerboseError<&'_ str>>> {
    let tokens = lex(input)?;
    let (unread, (defs, _eof)) = nom::sequence::tuple((
        Definitions::parse_tokens,
        contextful_cut("Unrecognized tokens", eat!(Eof)),
    ))(Tokens(&tokens[..]))
    .map_err(tokens::nom_error_into)?;

    // Cannot be empty here since eof would fail then
    assert!(unread.0.is_empty());

    Ok(defs)
}

pub trait Parse<'token>: Sized {
    fn parse_tokens<'slice>(
        input: Tokens<'slice, 'token>,
    ) -> VerboseResult<Tokens<'slice, 'token>, Self>;

    fn parse(input: &'token str) -> VerboseResult<&'token str, Self> {
        let (input, _) = whitespace::sp(input)?;
        let tokens = lex(input)?;
        let (unread, def) =
            Self::parse_tokens(Tokens(&tokens[..])).map_err(tokens::nom_error_into)?;
        let (unread, _) = whitespace::sp(unread.into())?;
        Ok((unread, def))
    }
}

/// Parses WebIDL definitions. It is the root struct for a complete WebIDL definition.
///
/// ### Example
/// ```
/// use weedle::{Definitions, Parse};
///
/// let (_, parsed) = Definitions::parse("
///     interface Window {
///         readonly attribute Storage sessionStorage;
///     };
/// ").unwrap();
///
/// println!("{:?}", parsed);
/// ```
///
/// It is recommended to use [`parse`](fn.parse.html) instead.
pub type Definitions<'a> = Vec<Definition<'a>>;

/// Parses `[attributes]? callback identifier = type ( (arg1, arg2, ..., argN)? );`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct CallbackDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub callback: term!(callback),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    #[weedle(cut = "Missing equal sign")]
    pub assign: term!(=),
    #[weedle(cut = "Unrecognized return type")]
    pub return_type: Type<'a>,
    #[weedle(cut = "Missing argument list")]
    pub arguments: Parenthesized<ArgumentList<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? callback interface identifier ( : inheritance )? { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct CallbackInterfaceDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub callback: term!(callback),
    pub interface: term!(interface),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<CallbackInterfaceMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? interface identifier ( : inheritance )? { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct InterfaceDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub interface: term!(interface),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub inheritance: Option<Inheritance<'a>>,
    pub members: Braced<InterfaceMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? interface mixin identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct InterfaceMixinDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub interface: term!(interface),
    pub mixin: term!(mixin),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<MixinMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? namespace identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct NamespaceDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub namespace: term!(namespace),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<NamespaceMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? dictionary identifier ( : inheritance )? { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct DictionaryDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub dictionary: term!(dictionary),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub inheritance: Option<Inheritance<'a>>,
    pub members: Braced<DictionaryMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? partial interface identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct PartialInterfaceDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub partial: term!(partial),
    pub interface: term!(interface),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<InterfaceMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? partial interface mixin identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct PartialInterfaceMixinDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub partial: term!(partial),
    pub interface: term!(interface),
    pub mixin: term!(mixin),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<MixinMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? partial dictionary identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct PartialDictionaryDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub partial: term!(partial),
    pub dictionary: term!(dictionary),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<DictionaryMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? partial namespace identifier { members };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct PartialNamespaceDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub partial: term!(partial),
    pub namespace: term!(namespace),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub members: Braced<NamespaceMembers<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? enum identifier { values };`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct EnumDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub enum_: term!(enum),
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    pub values: Braced<EnumValueList<'a>>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? typedef attributedtype identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct TypedefDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub typedef: term!(typedef),
    #[weedle(cut = "Unrecognized type")]
    pub type_: AttributedType<'a>,
    #[weedle(cut = "Missing name")]
    pub identifier: Identifier<'a>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses `[attributes]? identifier includes identifier;`
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[weedle(context)]
pub struct IncludesStatementDefinition<'a> {
    pub attributes: Option<ExtendedAttributeList<'a>>,
    pub lhs_identifier: Identifier<'a>,
    pub includes: term!(includes),
    pub rhs_identifier: Identifier<'a>,
    #[weedle(cut = "Missing semicolon")]
    pub semi_colon: term!(;),
}

/// Parses a definition
#[derive(Weedle, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Definition<'a> {
    CallbackInterface(CallbackInterfaceDefinition<'a>),
    Callback(CallbackDefinition<'a>),
    InterfaceMixin(InterfaceMixinDefinition<'a>),
    Interface(InterfaceDefinition<'a>),
    Namespace(NamespaceDefinition<'a>),
    Dictionary(DictionaryDefinition<'a>),
    PartialInterfaceMixin(PartialInterfaceMixinDefinition<'a>),
    PartialInterface(PartialInterfaceDefinition<'a>),
    PartialDictionary(PartialDictionaryDefinition<'a>),
    PartialNamespace(PartialNamespaceDefinition<'a>),
    Enum(EnumDefinition<'a>),
    Typedef(TypedefDefinition<'a>),
    IncludesStatement(IncludesStatementDefinition<'a>),
}

/// Parses a non-empty enum value list
pub type EnumValueList<'a> = PunctuatedNonEmpty<StringLit<'a>, term!(,)>;

#[cfg(test)]
mod test {
    use super::*;

    test!(should_parse_includes_statement { "first includes second;" =>
        "";
        IncludesStatementDefinition;
        attributes.is_none();
        lhs_identifier.0 == "first";
        rhs_identifier.0 == "second";
    });

    test!(should_parse_typedef { "typedef short Short;" =>
        "";
        TypedefDefinition;
        attributes.is_none();
        identifier.0 == "Short";
    });

    test!(should_parse_enum { r#"enum name { "first", "second" }; "# =>
        "";
        EnumDefinition;
        attributes.is_none();
        identifier.0 == "name";
        values.body.list.len() == 2;
    });

    test!(should_parse_dictionary { "dictionary A { long c; long g; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.0 == "A";
        inheritance.is_none();
        members.body.len() == 2;
    });

    test!(should_parse_dictionary_inherited { "dictionary C : B { long e; long f; };" =>
        "";
        DictionaryDefinition;
        attributes.is_none();
        identifier.0 == "C";
        inheritance.is_some();
        members.body.len() == 2;
    });

    test!(should_parse_partial_namespace { "
        partial namespace VectorUtils {
            readonly attribute Vector unit;
            double dotProduct(Vector x, Vector y);
            Vector crossProduct(Vector x, Vector y);
        };
    " =>
        "";
        PartialNamespaceDefinition;
        attributes.is_none();
        identifier.0 == "VectorUtils";
        members.body.len() == 3;
    });

    test!(should_parse_partial_dictionary { "partial dictionary C { long e; long f; };" =>
        "";
        PartialDictionaryDefinition;
        attributes.is_none();
        identifier.0 == "C";
        members.body.len() == 2;
    });

    test!(should_parse_partial_interface_mixin { "
        partial interface mixin WindowSessionStorage {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        PartialInterfaceMixinDefinition;
        attributes.is_none();
        identifier.0 == "WindowSessionStorage";
        members.body.len() == 1;
    });

    test!(should_parse_partial_interface { "
        partial interface Window {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        PartialInterfaceDefinition;
        attributes.is_none();
        identifier.0 == "Window";
        members.body.len() == 1;
    });

    test!(should_parse_namespace { "
        namespace VectorUtils {
          readonly attribute Vector unit;
          double dotProduct(Vector x, Vector y);
          Vector crossProduct(Vector x, Vector y);
        };
    " =>
        "";
        NamespaceDefinition;
        attributes.is_none();
        identifier.0 == "VectorUtils";
        members.body.len() == 3;
    });

    test!(should_parse_interface_mixin { "
        interface mixin WindowSessionStorage {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        InterfaceMixinDefinition;
        attributes.is_none();
        identifier.0 == "WindowSessionStorage";
        members.body.len() == 1;
    });

    test!(should_parse_interface { "
        interface Window {
          readonly attribute Storage sessionStorage;
        };
    " =>
        "";
        InterfaceDefinition;
        attributes.is_none();
        identifier.0 == "Window";
        members.body.len() == 1;
    });

    test!(should_parse_callback_interface {"
        callback interface Options {
          const int CONST = 3;
          undefined foo();
          undefined bar();
        };
    " =>
        "";
        CallbackInterfaceDefinition;
        attributes.is_none();
        identifier.0 == "Options";
        members.body.len() == 3;
    });

    test!(err should_not_parse_callback_interface_inheritance { "
        callback interface Options : Parent {};
    " =>
        CallbackInterfaceDefinition
    });

    test!(should_parse_callback { "callback AsyncOperationCallback = undefined (DOMString status);" =>
        "";
        CallbackDefinition;
        attributes.is_none();
        identifier.0 == "AsyncOperationCallback";
        arguments.body.list.len() == 1;
    });

    test!(should_parse_with_line_comments { "
        // This is a comment
        callback AsyncOperationCallback = undefined (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });

    test!(should_parse_with_block_comments { "
        /* This is a comment */
        callback AsyncOperationCallback = undefined (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });

    test!(should_parse_with_multiple_comments { "
        // This is a comment
        // This is a comment
        // This is a comment

        // This is a comment
        callback AsyncOperationCallback = undefined (DOMString status);
    " =>
        "";
        CallbackDefinition;
    });
}

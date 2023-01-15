use nom::{
    error::{ContextError, ParseError},
    IResult,
};

pub(crate) fn sp<'a, E>(input: &'a str) -> IResult<&'a str, &'a str, E>
where
    E: ParseError<&'a str> + ContextError<&'a str>,
{
    nom::combinator::recognize(nom::multi::many0(nom::branch::alt((
        // ignores line comments
        nom::combinator::value(
            (),
            nom::sequence::tuple((
                nom::bytes::complete::tag("//"),
                nom::bytes::complete::take_until("\n"),
                nom::bytes::complete::tag("\n"),
            )),
        ),
        // ignores whitespace
        nom::combinator::value((), nom::character::complete::multispace1),
        // ignores block comments
        nom::combinator::value(
            (),
            nom::sequence::tuple((
                nom::bytes::complete::tag("/*"),
                nom::bytes::complete::take_until("*/"),
                nom::bytes::complete::tag("*/"),
            )),
        ),
    ))))(input)
}

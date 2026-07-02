use nom::{IResult, Parser, character::complete, combinator::cut, multi::many0};

use crate::parser::{Parameter, errors::GcodeParseError, param_parsers::flag_parser::Flag};

pub enum FlagOrParam {
    Flag(Flag),
    Param(Parameter),
}

/// Parses G-code parameters, ignoring duplicates.
///
/// Note: This parser does not error if there are duplicate parameter keys (e.g., multiple `X` parameters);
/// both occurrences will occur in the resulting collection.
pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Vec<FlagOrParam>, GcodeParseError<'a>> {
    cut(many0(parse_param_or_flag)).parse_complete(input)
}

/// Parses a single gcode parameter or flag
///
/// ie: " X1" or " X"
fn parse_param_or_flag<'a>(input: &'a str) -> IResult<&'a str, FlagOrParam, GcodeParseError<'a>> {
    let (input, _) = complete::space1(input)?;

    let param_result: IResult<_, _, GcodeParseError> = Parameter::parse(input);

    if let Ok((remaining, param)) = param_result {
        return Ok((remaining, FlagOrParam::Param(param)));
    }

    let (remaining, flag) = Flag::parse(input)?;

    Ok((remaining, FlagOrParam::Flag(flag)))
}

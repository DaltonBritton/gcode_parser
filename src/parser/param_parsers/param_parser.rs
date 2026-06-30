use nom::combinator::cut;
use nom::{IResult, Parser, character::complete, multi::many0, sequence::preceded};

use crate::parser::{Parameter, errors::GcodeParseError};

/// Parses G-code parameters, ignoring duplicates.
///
/// Note: This parser does not error if there are duplicate parameter keys (e.g., multiple `X` parameters);
/// both occurrences will occur resulting collection.
pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Vec<Parameter>, GcodeParseError<'a>> {
    let (remaining, params) = cut(many0(parse_param)).parse_complete(input)?;

    Ok((remaining, params))
}

/// Parses a single gcode parameter
///
/// ie: " X1"
pub fn parse_param<'a>(input: &'a str) -> IResult<&'a str, Parameter, GcodeParseError<'a>> {
    preceded(complete::space1, Parameter::parse).parse_complete(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_params() {
        let test = "";

        let test = parse(test).expect("Unable to parse params").1;

        assert_eq!(test.len(), 0);
    }

    #[test]
    fn many_params() {
        let test = " X1 Y2 Z3 E4 F5";

        let test = parse(test).unwrap().1;

        let expected = vec![
            Parameter::new('X', 1.),
            Parameter::new('Y', 2.),
            Parameter::new('Z', 3.),
            Parameter::new('E', 4.),
            Parameter::new('F', 5.),
        ];

        assert_eq!(test, expected);
    }

    #[test]
    fn some_params() {
        let test = " X1 Z3 F5";

        let test = parse(test).unwrap().1;

        let expected = vec![
            Parameter::new('X', 1.),
            Parameter::new('Z', 3.),
            Parameter::new('F', 5.),
        ];

        assert_eq!(test, expected);
    }

    #[test]
    fn params_with_comment_1() {
        let test_str = " X1 Y2; my comment ";

        let (remaining, result) = parse(test_str).unwrap();

        let expected = vec![Parameter::new('X', 1.), Parameter::new('Y', 2.)];

        assert_eq!(result, expected);

        let expected_remaining = "; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn params_with_comment_2() {
        let test_str = " X1 Y2     ; my comment ";

        let (remaining, result) = parse(test_str).unwrap();

        let expected = vec![Parameter::new('X', 1.), Parameter::new('Y', 2.)];

        assert_eq!(result, expected);

        let expected_remaining = "     ; my comment ";
        assert_eq!(remaining, expected_remaining);
    }
}

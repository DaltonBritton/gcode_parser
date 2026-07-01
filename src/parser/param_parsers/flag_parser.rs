use nom::character::satisfy;
use nom::combinator::cut;
use nom::error::ParseError;
use nom::{IResult, Parser, character::complete, multi::many0, sequence::preceded};

use crate::parser::errors::GcodeParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flag {
    key: char,
}

impl Flag {
    pub fn new(key: char) -> Self {
        let key = key.to_ascii_uppercase();

        Flag { key }
    }

    pub fn parse<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, E> {
        let (input, key) = satisfy(|c| c.is_alphabetic()).parse(input)?;

        Ok((input, Self::new(key)))
    }
}

/// Parses G-code parameters, ignoring duplicates.
///
/// Note: This parser does not error if there are duplicate parameter keys (e.g., multiple `X` parameters);
/// both occurrences will occur in the resulting collection.
pub fn parse<'a>(input: &'a str) -> IResult<&'a str, Vec<Flag>, GcodeParseError<'a>> {
    cut(many0(parse_flag)).parse_complete(input)
}

/// Parses a single gcode parameter
///
/// ie: " X"
pub fn parse_flag<'a>(input: &'a str) -> IResult<&'a str, Flag, GcodeParseError<'a>> {
    preceded(complete::space1, Flag::parse).parse_complete(input)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn no_flags() {
        let test = "";

        let test = parse(test).expect("Unable to parse flags").1;

        assert_eq!(test.len(), 0);
    }

    #[test]
    fn many_flags() {
        let test = " X Y Z E F";

        let test = parse(test).unwrap().1;

        let expected = vec![
            Flag::new('X'),
            Flag::new('Y'),
            Flag::new('Z'),
            Flag::new('E'),
            Flag::new('F'),
        ];

        assert_eq!(test, expected);
    }

    #[test]
    fn some_flags() {
        let test = " X Z F";

        let test = parse(test).unwrap().1;

        let expected = vec![Flag::new('X'), Flag::new('Z'), Flag::new('F')];

        assert_eq!(test, expected);
    }

    #[test]
    fn flags_with_comment_1() {
        let test_str = " X Y; my comment ";

        let (remaining, result) = parse(test_str).unwrap();

        let expected = vec![Flag::new('X'), Flag::new('Y')];

        assert_eq!(result, expected);

        let expected_remaining = "; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn flags_with_comment_2() {
        let test_str = " X Y     ; my comment ";

        let (remaining, result) = parse(test_str).unwrap();

        let expected = vec![Flag::new('X'), Flag::new('Y')];

        assert_eq!(result, expected);

        let expected_remaining = "     ; my comment ";
        assert_eq!(remaining, expected_remaining);
    }
}

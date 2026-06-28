use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::space0;
use nom::combinator::{cut, eof, peek};
use nom::{IResult, Parser, sequence::preceded};

use crate::parser::Commands;
use crate::parser::errors::GcodeParseError;

pub fn parse_params<'a>(input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    let (remaining, _) =
        cut(preceded(space0, peek(alt((tag(";"), tag("\n"), eof))))).parse_complete(input)?;

    Ok((remaining, Commands::G90))
}

#[cfg(test)]
mod tests {

    use crate::parser::Commands;

    use super::parse_params;

    #[test]
    fn g90_empty() {
        let test = "";

        let test = parse_params(test)
            .expect("Unable to parse empty g1 command")
            .1;

        assert_eq!(test, Commands::G90);
    }

    #[test]
    fn g90_with_comment_1() {
        let test = "; my comment";

        let (remaining, result) = parse_params(test).expect("Unable to parse empty g90");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, Commands::G90);
    }

    #[test]
    fn g90_with_comment_2() {
        let test = "          ; my comment";

        let (remaining, result) = parse_params(test).expect("Unable to parse empty g90");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, Commands::G90);
    }

    #[test]
    fn g90_invalid_param() {
        let test = " X1";

        let test = parse_params(test);

        assert!(test.is_err());
    }
}

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::space0,
    combinator::{cut, eof, peek},
    sequence::preceded,
};

use crate::parser::errors::GcodeParseError;

pub fn parse_params<'a>(input: &'a str) -> IResult<&'a str, (), GcodeParseError<'a>> {
    let (remaining, _) =
        cut(preceded(space0, peek(alt((tag(";"), tag("\n"), eof))))).parse_complete(input)?;

    Ok((remaining, ()))
}

#[cfg(test)]
mod tests {

    

    use super::parse_params;

    #[test]
    fn parameterless_parser_empty() {
        let test = "";

        let test = parse_params(test)
            .expect("Unable to parse empty g1 command")
            .1;

        assert_eq!(test, ());
    }

    #[test]
    fn parameterless_parser_with_comment_1() {
        let test = "; my comment";

        let (remaining, result) =
            parse_params(test).expect("Unable to parse empty parameterless_parser");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, ());
    }

    #[test]
    fn parameterless_parser_with_comment_2() {
        let test = "          ; my comment";

        let (remaining, result) =
            parse_params(test).expect("Unable to parse empty parameterless_parser");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, ());
    }

    #[test]
    fn parameterless_parser_invalid_param() {
        let test = " X1";

        let test = parse_params(test);

        assert!(test.is_err());
    }
}

use nom::IResult;

use crate::parser::Commands;
use crate::parser::errors::GcodeParseError;
use crate::parser::param_parsers::parameterless_parser;

pub fn parse_params<'a>(input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    let (remaining, _) = parameterless_parser::parse_params(input)?;

    Ok((remaining, Commands::G91))
}

#[cfg(test)]
mod tests {
    use crate::parser::Commands;

    use super::parse_params;

    #[test]
    fn g91_empty() {
        let test = "";

        let test = parse_params(test).expect("Unable to parse empty command").1;

        assert_eq!(test, Commands::G91);
    }

    #[test]
    fn g91_with_comment_1() {
        let test = "; my comment";

        let (remaining, result) =
            parse_params(test).expect("Unable to parse empty command with comment");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, Commands::G91);
    }

    #[test]
    fn g91_with_comment_2() {
        let test = "          ; my comment";

        let (remaining, result) = parse_params(test).expect("Unable to parse empty command");

        assert_eq!(remaining, "; my comment");
        assert_eq!(result, Commands::G91);
    }

    #[test]
    fn g91_invalid_param() {
        let test = " X1";

        let test = parse_params(test);

        assert!(test.is_err());
    }
}

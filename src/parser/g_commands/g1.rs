use nom::{IResult, Parser, character::complete, multi::many0, sequence::preceded};

use crate::parser::errors::Reason;
use crate::parser::{Commands, Parameter, errors::GcodeParseError};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G1Params {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    e: Option<f64>,

    f: Option<f64>, // Requested movement rate in mm/min
}

impl G1Params {
    pub fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            e: None,
            f: None,
        }
    }
}

pub fn parse_params<'a>(input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    let (remaining, params) =
        many0(preceded(complete::space1, Parameter::parse)).parse_complete(input)?;

    let mut g1_params = G1Params::new();

    for param in params {
        let param_value: &mut Option<f64>;

        match param.key {
            'X' => param_value = &mut g1_params.x,
            'Y' => param_value = &mut g1_params.y,
            'Z' => param_value = &mut g1_params.z,
            'E' => param_value = &mut g1_params.e,
            'F' => param_value = &mut g1_params.f,
            _ => {
                return Err(nom::Err::Failure(GcodeParseError::new(
                    input,
                    Reason::InvalidParam(param.key),
                )));
            }
        }

        if param_value.is_some() {
            return Err(nom::Err::Failure(GcodeParseError::new(
                input,
                Reason::DuplicateParam(param.key),
            )));
        }

        *param_value = Some(param.value);
    }

    let g1_command = Commands::G1(g1_params);

    Ok((remaining, g1_command))
}

#[cfg(test)]
mod tests {

    use crate::parser::{
        Commands,
        g_commands::g1::{G1Params, parse_params},
    };

    #[test]
    fn g1_empty() {
        let test = "";

        let test = parse_params(test)
            .expect("Unable to parse empty g1 command")
            .1;

        assert_eq!(test, Commands::G1(G1Params::new()));
    }

    #[test]
    fn g1_all_params() {
        let test = " X1 Y2 Z3 E4 F5";

        let test = parse_params(test).expect("Unable to parse G1 command").1;

        let expected = Commands::G1(G1Params {
            x: Some(1.),
            y: Some(2.),
            z: Some(3.),
            e: Some(4.),
            f: Some(5.),
        });

        assert_eq!(test, expected);
    }

    #[test]
    fn g1_some_params() {
        let test = " X1 Z3 F5";

        let test = parse_params(test).expect("Unable to parse G1 command").1;

        let expected = Commands::G1(G1Params {
            x: Some(1.),
            y: None,
            z: Some(3.),
            e: None,
            f: Some(5.),
        });

        assert_eq!(test, expected);
    }

    #[test]
    fn g1_with_comment_1() {
        let test_str = " X1 Y2; my comment ";

        let (remaining, result) = parse_params(test_str).expect("Unable to parse G1 command");

        let expected = Commands::G1(G1Params {
            x: Some(1.),
            y: Some(2.),
            z: None,
            e: None,
            f: None,
        });

        assert_eq!(result, expected);

        let expected_remaining = "; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn g1_with_comment_2() {
        let test_str = " X1 Y2     ; my comment ";

        let (remaining, result) = parse_params(test_str).expect("Unable to parse G1 command");

        let expected = Commands::G1(G1Params {
            x: Some(1.),
            y: Some(2.),
            z: None,
            e: None,
            f: None,
        });

        assert_eq!(result, expected);

        let expected_remaining = "     ; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn g1_invalid_param() {
        let test = " Q3";

        let test = parse_params(test);

        assert!(test.is_err());
    }

    #[test]
    fn g1_duplicate_param() {
        let test = " X1 X2";

        let test = parse_params(test);

        assert!(test.is_err());
    }
}

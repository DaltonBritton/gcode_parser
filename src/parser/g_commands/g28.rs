use nom::IResult;

use crate::parser::errors::Reason;
use crate::parser::param_parsers::flag_param_parser;
use crate::parser::param_parsers::flag_param_parser::FlagOrParam;
use crate::parser::{Commands, errors::GcodeParseError};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G28Params {
    x: bool,
    y: bool,
    z: bool,

    // TODO: L Flag
    l: Option<bool>, // Flag to restore bed leveling state after homing
    r: Option<f64>,  // Distance to raise the nozzle before leveling
}

impl G28Params {
    fn new() -> Self {
        Self {
            x: false,
            y: false,
            z: false,
            l: None,
            r: None,
        }
    }
}

impl Default for G28Params {
    fn default() -> Self {
        Self {
            x: true,
            y: true,
            z: true,
            l: Default::default(),
            r: Default::default(),
        }
    }
}

pub fn parse_params<'a>(input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    let (remaining, params) = flag_param_parser::parse(input)?;

    if params.is_empty() {
        return Ok((remaining, Commands::G28(G28Params::default())));
    }

    let mut g28_params = G28Params::new();

    for param in params {
        match param {
            FlagOrParam::Flag(flag) => {
                let param_value = match flag.key {
                    'X' => &mut g28_params.x,
                    'Y' => &mut g28_params.y,
                    'Z' => &mut g28_params.z,
                    _ => {
                        return Err(nom::Err::Failure(GcodeParseError::new(
                            input,
                            Reason::InvalidParam(flag.key),
                        )));
                    }
                };

                if *param_value {
                    return Err(nom::Err::Failure(GcodeParseError::new(
                        input,
                        Reason::DuplicateParam(flag.key),
                    )));
                }

                *param_value = true;
            }
            FlagOrParam::Param(param) => match param.key {
                'R' => {
                    if g28_params.r.is_some() {
                        return Err(nom::Err::Failure(GcodeParseError::new(
                            input,
                            Reason::DuplicateParam(param.key),
                        )));
                    }

                    g28_params.r = Some(param.value);
                }
                'L' => {
                    g28_params.l = match param.value {
                        0. => Some(false),
                        1. => Some(true),
                        _ => {
                            return Err(nom::Err::Failure(GcodeParseError::new(
                                input,
                                Reason::InvalidParam(param.key),
                            )));
                        }
                    };
                }
                _ => {
                    return Err(nom::Err::Failure(GcodeParseError::new(
                        input,
                        Reason::InvalidParam(param.key),
                    )));
                }
            },
        }
    }

    let command = Commands::G28(g28_params);

    Ok((remaining, command))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn g28_empty() {
        let test = "";

        let test = parse_params(test)
            .expect("Unable to parse empty g1 command")
            .1;

        assert_eq!(test, Commands::G28(G28Params::default()));
    }

    #[test]
    fn g28_flags() {
        let test = " X Y Z";

        let test = parse_params(test).expect("Unable to parse G1 command").1;

        let expected = Commands::G28(G28Params {
            x: true,
            y: true,
            z: true,
            l: None,
            r: None,
        });

        assert_eq!(test, expected);
    }

    #[test]
    fn g28_all_params() {
        let test = " X Y Z L1 R5";

        let test = parse_params(test).expect("Unable to parse G1 command").1;

        let expected = Commands::G28(G28Params {
            x: true,
            y: true,
            z: true,
            l: Some(true),
            r: Some(5.),
        });

        assert_eq!(test, expected);
    }

    #[test]
    fn g28_some_flags() {
        let test = " X Z";

        let test = parse_params(test).expect("Unable to parse G1 command").1;

        let expected = Commands::G28(G28Params {
            x: true,
            y: false,
            z: true,
            l: None,
            r: None,
        });

        assert_eq!(test, expected);
    }

    #[test]
    fn g28_with_comment_1() {
        let test_str = " X Y; my comment ";

        let (remaining, result) = parse_params(test_str).expect("Unable to parse G1 command");

        let expected = Commands::G28(G28Params {
            x: true,
            y: true,
            z: false,
            l: None,
            r: None,
        });

        assert_eq!(result, expected);

        let expected_remaining = "; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn g28_with_comment_2() {
        let test_str = " X Y     ; my comment ";

        let (remaining, result) = parse_params(test_str).expect("Unable to parse G1 command");

        let expected = Commands::G28(G28Params {
            x: true,
            y: true,
            z: false,
            l: None,
            r: None,
        });

        assert_eq!(result, expected);

        let expected_remaining = "     ; my comment ";
        assert_eq!(remaining, expected_remaining);
    }

    #[test]
    fn g28_invalid_param() {
        let test = " Q3";

        let test = parse_params(test);

        assert!(test.is_err());
    }

    #[test]
    fn g28_duplicate_param() {
        let test = " X X";

        let test = parse_params(test);

        assert!(test.is_err());
    }
}

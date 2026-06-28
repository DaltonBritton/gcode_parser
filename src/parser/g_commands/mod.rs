use nom::{
    IResult, Parser,
    character::complete::{self},
    multi::many0,
    sequence::preceded,
};

use crate::parser::{Commands, Parameter};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G1Params {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    e: Option<f64>,

    f: Option<f64>, // Requested movement rate in mm/min
}

impl G1Params {
    fn new() -> Self {
        Self {
            x: None,
            y: None,
            z: None,
            e: None,
            f: None,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G28Params {
    x: bool,
    y: bool,
    z: bool,

    l: Option<bool>, // Flag to restore bed leveling state after homing
    r: Option<f64>,  // Distance to raise the nozzle before leveling
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct G92Params {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    e: Option<f64>,
}

pub fn parse_g1_params<'a>(input: &'a str) -> IResult<&'a str, Commands> {
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
            _ => todo!("Error: Unknown param"),
        }

        if param_value.is_some() {
            todo!("Error: Dupicate key")
        }

        *param_value = Some(param.value);
    }

    let g1_command = Commands::G1(g1_params);

    Ok((remaining, g1_command))
}

pub fn parse_g28<'a>(_input: &'a str) -> IResult<&'a str, Commands> {
    todo!()
}

pub fn parse_g29<'a>(_input: &'a str) -> IResult<&'a str, Commands> {
    todo!()
}

pub fn parse_g90<'a>(_input: &'a str) -> IResult<&'a str, Commands> {
    todo!()
}

pub fn parse_g91<'a>(_input: &'a str) -> IResult<&'a str, Commands> {
    todo!()
}

#[cfg(test)]
mod tests {

    use crate::parser::{
        Commands,
        g_commands::{G1Params, parse_g1_params},
    };

    #[test]
    fn g1_empty() {
        let test = "";

        let test = parse_g1_params(test)
            .expect("Unable to parse empty g1 command")
            .1;

        assert_eq!(
            test,
            Commands::G1(crate::parser::g_commands::G1Params::new())
        );
    }

    #[test]
    fn g1_all_params() {
        let test = " X1 Y2 Z3 E4 F5";

        let test = parse_g1_params(test).expect("Unable to parse G1 command").1;

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

        let test = parse_g1_params(test).expect("Unable to parse G1 command").1;

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
    fn g1_with_comment() {
        let test_str = " X1 Y2; my comment ";

        let (remaining, result) = parse_g1_params(test_str).expect("Unable to parse G1 command");

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
    #[should_panic]
    fn g1_invalid_param() {
        let test = " Q3";

        let test = parse_g1_params(test);

        assert!(test.is_err());
    }

    #[test]
    #[should_panic]
    fn g1_duplicate_param() {
        let test = " X1 X2";

        let test = parse_g1_params(test);

        assert!(test.is_err());
    }
}

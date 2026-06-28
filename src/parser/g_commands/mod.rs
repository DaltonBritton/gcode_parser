use nom::IResult;

use crate::parser::{Commands, errors::GcodeParseError};

pub mod g1;

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

pub fn parse_g28<'a>(_input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    todo!()
}

pub fn parse_g29<'a>(_input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    todo!()
}

pub fn parse_g90<'a>(_input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    todo!()
}

pub fn parse_g91<'a>(_input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    todo!()
}

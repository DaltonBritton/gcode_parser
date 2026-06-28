use nom::IResult;
use nom::Parser;
use nom::error::ParseError;
use nom::number::complete::double;

use nom::character::complete::u32;
use nom::character::satisfy;

use crate::parser::errors::GcodeParseError;
use crate::parser::g_commands::G28Params;
use crate::parser::g_commands::G92Params;
use crate::parser::g_commands::g1::G1Params;

pub mod errors;
pub mod g_commands;
pub mod m_commands;

struct CommandCode {
    key: char,
    value: u32,
}

impl CommandCode {
    fn new(key: char, value: u32) -> Self {
        let key = key.to_ascii_uppercase();

        CommandCode { key, value }
    }

    fn parse<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, E> {
        let (input, key) = satisfy(|c| c.is_alphabetic()).parse(input)?;

        let (input, value) = u32(input)?;

        Ok((input, Self::new(key, value)))
    }
}

struct Parameter {
    key: char,
    value: f64,
}

impl Parameter {
    fn new(key: char, value: f64) -> Self {
        let key = key.to_ascii_uppercase();

        Parameter { key, value }
    }

    fn parse<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, Self, E> {
        let (input, key) = satisfy(|c| c.is_alphabetic()).parse(input)?;

        let (input, value) = double(input)?;

        Ok((input, Self::new(key, value)))
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Commands {
    G1(G1Params),
    G28(G28Params),
    G29,
    G90,
    G91,
    G92(G92Params),
}

pub fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Commands, GcodeParseError<'a>> {
    let (input, command_code) = CommandCode::parse(input)?;

    match command_code {
        CommandCode {
            key: 'G',
            value: 0 | 1,
        } => g_commands::g1::parse_params(input),
        CommandCode {
            key: 'G',
            value: 28,
        } => g_commands::parse_g28(input),
        CommandCode {
            key: 'G',
            value: 29,
        } => g_commands::parse_g29(input),
        CommandCode {
            key: 'G',
            value: 90,
        } => g_commands::g90::parse_params(input),
        CommandCode {
            key: 'G',
            value: 91,
        } => g_commands::parse_g91(input),
        _ => Err(nom::Err::Error(GcodeParseError {
            input,
            reason: errors::Reason::UnreconizedCommand,
        })),
    }
}

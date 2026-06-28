use nom::IResult;
use nom::Parser;
use nom::number::complete::double;

use nom::character::complete::u32;
use nom::character::satisfy;

use crate::parser::g_commands::G1Params;
use crate::parser::g_commands::G28Params;
use crate::parser::g_commands::G92Params;

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

    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
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

    fn parse<'a>(input: &'a str) -> IResult<&'a str, Self> {
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

pub fn parse_command<'a>(input: &'a str) -> IResult<&'a str, Commands> {
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
        } => g_commands::parse_g90(input),
        CommandCode {
            key: 'G',
            value: 91,
        } => g_commands::parse_g91(input),
        _ => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn g_command_header_test0() {
        let raw_command = "G0 x1 y2 z3";

        let parsed = g_command_header(raw_command).unwrap();

        assert_eq!(parsed.0, " x1 y2 z3");
        assert_eq!(parsed.1, 0);
    }

    #[test]
    fn g_command_header_test1() {
        let raw_command = "G1 x3 z1";

        let parsed = g_command_header(raw_command).unwrap();

        assert_eq!(parsed.0, " x3 z1");
        assert_eq!(parsed.1, 1);
    }
}
 */

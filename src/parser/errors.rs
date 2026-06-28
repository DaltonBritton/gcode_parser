use nom::error::{ErrorKind, ParseError};

pub struct GcodeParseError<'a> {
    pub input: &'a str,
    pub reason: Reason,
}

pub enum Reason {
    DuplicateArg,
    InvalidArg,
    NomError(ErrorKind),
}

impl<'a> GcodeParseError<'a> {
    pub fn new(input: &'a str, reason: Reason) -> Self {
        GcodeParseError { input, reason }
    }
}

impl<'a> ParseError<&'a str> for GcodeParseError<'a> {
    fn from_error_kind(input: &'a str, kind: ErrorKind) -> Self {
        GcodeParseError::new(input, Reason::NomError(kind))
    }

    fn append(_input: &str, _kind: ErrorKind, other: Self) -> Self {
        other
    }
}

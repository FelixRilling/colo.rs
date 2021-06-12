use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingErrorKind<'a> {
    InvalidSyntax {
        details: &'a str
    },
    ConversionFailed,
    PatternMatchingFailed,
}

impl Display for ParsingErrorKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsingErrorKind::InvalidSyntax { details } => f.write_str(details),
            ParsingErrorKind::ConversionFailed => f.write_str("Value conversion failed"),
            ParsingErrorKind::PatternMatchingFailed => f.write_str("Pattern matching failed"),
        }
    }
}

#[derive(Debug)]
pub struct ParsingError<'a> {
    pub(crate) kind: ParsingErrorKind<'a>,
}

impl ParsingError<'_> {
    pub fn kind(&self) -> &ParsingErrorKind {
        &self.kind
    }
}

impl Display for ParsingError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Parsing Error: {}", self.kind))
    }
}

impl Error for ParsingError<'_> {}

impl From<ParseIntError> for ParsingError<'_> {
    fn from(_: ParseIntError) -> Self {
        ParsingError { kind: ParsingErrorKind::ConversionFailed }
    }
}

impl From<ParseFloatError> for ParsingError<'_> {
    fn from(_: ParseFloatError) -> Self {
        ParsingError { kind: ParsingErrorKind::ConversionFailed }
    }
}
impl From<rug::float::ParseFloatError> for ParsingError<'_> {
    fn from(_: rug::float::ParseFloatError) -> Self {
        ParsingError { kind: ParsingErrorKind::ConversionFailed }
    }
}

impl From<regex::Error> for ParsingError<'_> {
    fn from(_: regex::Error) -> Self {
        ParsingError { kind: ParsingErrorKind::PatternMatchingFailed }
    }
}

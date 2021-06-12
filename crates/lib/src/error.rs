use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingErrorKind<'a> {
    InvalidSyntax {
        details: &'a str
    },
    ConversionFailed {
        cause: ParseIntError
    },
}

impl Display for ParsingErrorKind<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsingErrorKind::InvalidSyntax { details } => f.write_str(details),
            ParsingErrorKind::ConversionFailed { cause } => f.write_str(&cause.to_string()),
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
    fn from(e: ParseIntError) -> Self {
        ParsingError { kind: ParsingErrorKind::ConversionFailed { cause: e } }
    }
}

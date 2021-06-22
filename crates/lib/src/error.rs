use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use std::option::Option::None;

/// Kinds of errors than may happen during color parsing.
#[derive(Debug)]
pub enum ParsingError<'a> {
    InvalidSyntax(&'a str),

    NumberConversionFailed(Box<dyn Error>),

    RegexFailed(regex::Error),
}

impl Display for ParsingError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidSyntax(details) => f.write_str(details),
            ParsingError::NumberConversionFailed(err) => write!(f, "Number conversion failed: {}", err),
            ParsingError::RegexFailed(err) => write!(f, "Regex conversion failed: {}", err),
        }
    }
}

impl Error for ParsingError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParsingError::InvalidSyntax(_) => None,
            ParsingError::NumberConversionFailed(err) => Some(err.deref()),
            ParsingError::RegexFailed(err) => Some(err),
        }
    }
}


impl From<std::num::ParseFloatError> for ParsingError<'_> {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

impl From<std::num::ParseIntError> for ParsingError<'_> {
    fn from(err: std::num::ParseIntError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

impl From<rug::float::ParseFloatError> for ParsingError<'_> {
    fn from(err: rug::float::ParseFloatError) -> Self {
        ParsingError::NumberConversionFailed(Box::new(err))
    }
}

impl From<regex::Error> for ParsingError<'_> {
    fn from(err: regex::Error) -> Self {
        ParsingError::RegexFailed(err)
    }
}

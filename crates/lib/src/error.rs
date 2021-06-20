use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::option::Option::None;

/// Kinds of errors than may happen during color parsing.
#[derive(Debug)]
pub enum ParsingError<'a> {
    InvalidSyntax(&'a str),

    IntegerConversionFailed(std::num::ParseIntError),
    FloatConversionFailed(std::num::ParseFloatError),
    ArbitraryPrecisionFloatConversionFailed(rug::float::ParseFloatError),
    RegexFailed(regex::Error),
}

impl Display for ParsingError<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::InvalidSyntax(details) => f.write_str(details),
            ParsingError::IntegerConversionFailed(err) => write!(f, "Integer conversion failed: {}", err),
            ParsingError::FloatConversionFailed(err) => write!(f, "Float conversion failed: {}", err),
            ParsingError::ArbitraryPrecisionFloatConversionFailed(err) => write!(f, "Arbitrary precision float conversion failed: {}", err),
            ParsingError::RegexFailed(err) => write!(f, "Regex conversion failed: {}", err),
        }
    }
}

impl Error for ParsingError<'_> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParsingError::InvalidSyntax(_) => None,
            ParsingError::IntegerConversionFailed(err) => Some(err),
            ParsingError::FloatConversionFailed(err) => Some(err),
            ParsingError::ArbitraryPrecisionFloatConversionFailed(err) => Some(err),
            ParsingError::RegexFailed(err) => Some(err),
        }
    }
}


impl From<std::num::ParseFloatError> for ParsingError<'_> {
    fn from(err: std::num::ParseFloatError) -> Self {
        ParsingError::FloatConversionFailed(err)
    }
}

impl From<std::num::ParseIntError> for ParsingError<'_> {
    fn from(err: std::num::ParseIntError) -> Self {
        ParsingError::IntegerConversionFailed(err)
    }
}

impl From<rug::float::ParseFloatError> for ParsingError<'_> {
    fn from(err: rug::float::ParseFloatError) -> Self {
        ParsingError::ArbitraryPrecisionFloatConversionFailed(err)
    }
}

impl From<regex::Error> for ParsingError<'_> {
    fn from(err: regex::Error) -> Self {
        ParsingError::RegexFailed(err)
    }
}

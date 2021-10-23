//! Public error types.

use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::option::Option::None;

/// Kinds of errors than may happen during color parsing.
#[derive(Debug)]
pub enum ParsingError<'a> {
	InvalidSyntax(&'a str),

	UnsupportedValue(&'a str),

	NumberConversionFailed(Box<dyn Error>),
}

impl Display for ParsingError<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			ParsingError::InvalidSyntax(details) => f.write_str(details),
			ParsingError::UnsupportedValue(details) => f.write_str(details),
			ParsingError::NumberConversionFailed(_) => f.write_str("Number conversion failed"),
		}
	}
}

impl Error for ParsingError<'_> {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		match self {
			ParsingError::InvalidSyntax(_) => None,
			ParsingError::UnsupportedValue(_) => None,
			ParsingError::NumberConversionFailed(err) => Some(&**err),
		}
	}
}

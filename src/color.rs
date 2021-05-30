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
    kind: ParsingErrorKind<'a>,
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


#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    pub(crate) fn red(&self) -> u8 {
        self.r
    }
    pub(crate) fn green(&self) -> u8 {
        self.g
    }
    pub(crate) fn blue(&self) -> u8 {
        self.b
    }
}

impl RGB {
    // https://www.w3.org/TR/css-color-4/#typedef-hex-color
    pub fn from_hex_str(hex_str: &str) -> Result<RGB, ParsingError> {
        if !hex_str.starts_with('#') {
            return Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax { details: "Missing '#'" } });
        }
        let hex_digits = &hex_str[1..];
        match hex_digits.len() {
            3 => {
                // In the shorthand notation, the hex digit is simply repeated, so e.g "F" becomes "FF".
                let r = u8::from_str_radix(&hex_digits[0..1].repeat(2), 16)?;
                let g = u8::from_str_radix(&hex_digits[1..2].repeat(2), 16)?;
                let b = u8::from_str_radix(&hex_digits[2..3].repeat(2), 16)?;

                Ok(RGB { r, g, b })
            }
            6 => {
                let r = u8::from_str_radix(&hex_digits[0..2], 16)?;
                let g = u8::from_str_radix(&hex_digits[2..4], 16)?;
                let b = u8::from_str_radix(&hex_digits[4..6], 16)?;

                Ok(RGB { r, g, b })
            }
            _ => Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax { details: "Unexpected length" } })
        }
    }

    pub fn to_hex_str(&self) -> String {
        format!("#{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_str_errors_for_no_hash() {
        let result = RGB::from_hex_str("112233");

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), &ParsingErrorKind::InvalidSyntax { details: "Missing '#'" });
    }

    #[test]
    fn from_hex_str_invalid_chars() {
        let result = RGB::from_hex_str("#XX2233");

        assert!(result.is_err());
        matches!(result.err().unwrap().kind(), &ParsingErrorKind::ConversionFailed { .. });
    }

    #[test]
    fn from_hex_str_invalid_length() {
        let result_too_long = RGB::from_hex_str("#1111111111111111111111");
        assert!(result_too_long.is_err());
        assert_eq!(result_too_long.err().unwrap().kind(), &ParsingErrorKind::InvalidSyntax { details: "Unexpected length" });

        let result_between_short_and_long = RGB::from_hex_str("#11223");
        assert!(result_between_short_and_long.is_err());
        assert_eq!(result_between_short_and_long.err().unwrap().kind(), &ParsingErrorKind::InvalidSyntax { details: "Unexpected length" });

        let result_between_too_short = RGB::from_hex_str("#11");
        assert!(result_between_too_short.is_err());
        assert_eq!(result_between_too_short.err().unwrap().kind(), &ParsingErrorKind::InvalidSyntax { details: "Unexpected length" });
    }

    #[test]
    fn from_hex_str_long_notation() {
        let color = RGB::from_hex_str("#112233").unwrap();

        assert_eq!(color.r, u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.g, u8::from_str_radix("22", 16).unwrap());
        assert_eq!(color.b, u8::from_str_radix("33", 16).unwrap());
    }

    #[test]
    fn from_hex_str_short_notation() {
        let color = RGB::from_hex_str("#123").unwrap();

        assert_eq!(color.r, u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.g, u8::from_str_radix("22", 16).unwrap());
        assert_eq!(color.b, u8::from_str_radix("33", 16).unwrap());
    }
}

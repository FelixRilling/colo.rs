use std::{fmt, num};
use std::fmt::Display;

#[derive(Debug)]
pub struct ParsingError {
    kind: ParsingErrorKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParsingErrorKind {
    InvalidSyntax,
    ConversionFailed(num::ParseIntError),
}

impl ParsingError {
    pub fn kind(&self) -> &ParsingErrorKind {
        &self.kind
    }
}


#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    // https://www.w3.org/TR/css-color-4/#typedef-hex-color
    pub fn from_hex_str(hex_str: &str) -> Result<RGB, ParsingError> {
        if !hex_str.starts_with('#') {
            return Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax });
        }
        let hex_digits = &hex_str[1..];
        if hex_digits.len() == 6 {
            let r = RGB::parse_hex_str(&hex_digits[0..2])?;
            let g = RGB::parse_hex_str(&hex_digits[2..4])?;
            let b = RGB::parse_hex_str(&hex_digits[4..6])?;

            Ok(RGB { r, g, b })
        } else {
            Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax })
        }
    }

    fn parse_hex_str(hex_digits: &str) -> Result<u8, ParsingError> {
        u8::from_str_radix(hex_digits, 16)
            .map_err(|e| ParsingError { kind: ParsingErrorKind::ConversionFailed(e) })
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
    fn from_hex_str_panics_for_no_hash() {
        let result = RGB::from_hex_str("112233");

        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), &ParsingErrorKind::InvalidSyntax);
    }

    #[test]
    fn from_hex_str_long_notation_invalid_chars() {
        let result = RGB::from_hex_str("#XX2233");

        assert!(result.is_err());
    }

    #[test]
    fn from_hex_str_long_notation() {
        let color = RGB::from_hex_str("#112233").unwrap();

        assert_eq!(color.r, u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.g, u8::from_str_radix("22", 16).unwrap());
        assert_eq!(color.b, u8::from_str_radix("33", 16).unwrap());
    }
}

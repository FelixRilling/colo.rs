use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use regex::Regex;
use rug::Float;

use crate::color::srgb::{SRGB_PRECISION, srgb_to_rgb};
use crate::error::ParsingError;

/// Represents a single RGB color with an alpha channel.
#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl RGB {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> RGB {
        RGB { red, green, blue, alpha }
    }

    /// Creates a RGB instance based on the given values. alpha channel is fully opaque.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> RGB {
        RGB::from_rgba(red, green, blue, u8::MAX)
    }

    /// Parses a CSS-style hexadecimal representation of an RGB color.
    /// For a list of supported formats, see <https://www.w3.org/TR/css-color-4/#hex-notation>.
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Missing the '#' character at the start of the string.
    /// - Non-hexadecimal digits.
    /// - A length of the digit part not equal to 3, 4, 6 or 8.
    pub fn from_hex_str(hex_str: &str) -> Result<RGB, ParsingError> {
        if !hex_str.starts_with('#') {
            return Err(ParsingError::InvalidSyntax("Missing '#'"));
        }
        let hex_digits = &hex_str[1..];
        let len = hex_digits.len();
        match len {
            3 | 4 => {
                // In the shorthand notation, the hex digit is simply repeated, so e.g "F" becomes "FF".
                let red = u8::from_str_radix(&hex_digits[0..1].repeat(2), 16)?;
                let green = u8::from_str_radix(&hex_digits[1..2].repeat(2), 16)?;
                let blue = u8::from_str_radix(&hex_digits[2..3].repeat(2), 16)?;

                match len {
                    3 => Ok(RGB::from_rgb(red, green, blue)),
                    4 => {
                        let alpha = u8::from_str_radix(&hex_digits[3..4].repeat(2), 16)?;
                        Ok(RGB::from_rgba(red, green, blue, alpha))
                    }
                    _ => unreachable!()
                }
            }
            6 | 8 => {
                let red = u8::from_str_radix(&hex_digits[0..2], 16)?;
                let green = u8::from_str_radix(&hex_digits[2..4], 16)?;
                let blue = u8::from_str_radix(&hex_digits[4..6], 16)?;

                match len {
                    6 => Ok(RGB::from_rgb(red, green, blue)),
                    8 => {
                        let alpha = u8::from_str_radix(&hex_digits[6..8], 16)?;
                        Ok(RGB::from_rgba(red, green, blue, alpha))
                    }
                    _ => unreachable!()
                }
            }
            _ => Err(ParsingError::InvalidSyntax("Unexpected length. String must have either 3, 4, 6, or 8 hexadecimal digits"))
        }
    }

    /// Parses a CSS-style RGB color.
    /// For a list of supported formats, see <https://www.w3.org/TR/css-color-4/#rgb-functions>.
    /// Color percentage values are currently not supported.
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Input not matching the shape of an RGB string.
    /// - Out-of-range color values.
    pub fn from_rgb_str(hex_str: &str) -> Result<RGB, ParsingError> {
        let rgb_regex = Regex::new(
            r"^rgb\((?P<red>\d{1,3}) (?P<green>\d{1,3}) (?P<blue>\d{1,3})(?: / (?P<alpha>\d(?:\.\d+)?))?\)$"
        )?;

        match rgb_regex.captures(hex_str) {
            None => Err(ParsingError::InvalidSyntax("String did not match RGB pattern")),
            Some(captures) => {
                let red = u8::from_str(captures.name("red").unwrap().as_str())?;
                let green = u8::from_str(captures.name("green").unwrap().as_str())?;
                let blue = u8::from_str(captures.name("blue").unwrap().as_str())?;

                match captures.name("alpha") {
                    None => Ok(RGB::from_rgb(red, green, blue)),
                    Some(alpha_match) => {
                        let alpha_raw = Float::with_val(SRGB_PRECISION, Float::parse(alpha_match.as_str())?);
                        let alpha = srgb_to_rgb(alpha_raw);
                        Ok(RGB::from_rgba(red, green, blue, alpha))
                    }
                }
            }
        }
    }

    pub fn to_hex_str(&self) -> String {
        // TODO support custom output format (uppercase/lowercase and/or short notation)
        match self.alpha {
            u8::MAX => format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue),
            _ => format!("#{:02X}{:02X}{:02X}{:02X}", self.red, self.green, self.blue, self.alpha),
        }
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
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax(..)))
    }

    #[test]
    fn from_hex_str_invalid_chars() {
        let result = RGB::from_hex_str("#XX2233");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::IntegerConversionFailed ( .. )))
    }

    #[test]
    fn from_hex_str_invalid_length() {
        let result_too_long = RGB::from_hex_str("#1111111111111111111111");
        assert!(result_too_long.is_err());
        assert!(matches!(result_too_long.err().unwrap(), ParsingError::InvalidSyntax ( .. )));

        let result_between_short_and_long = RGB::from_hex_str("#11223");
        assert!(result_between_short_and_long.is_err());
        assert!(matches!(result_between_short_and_long.err().unwrap(), ParsingError::InvalidSyntax ( .. )));

        let result_between_too_short = RGB::from_hex_str("#11");
        assert!(result_between_too_short.is_err());
        assert!(matches!(result_between_too_short.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }


    #[test]
    fn from_hex_str_short_notation() {
        let color = RGB::from_hex_str("#1FA").unwrap();

        assert_eq!(color.red(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue(), u8::from_str_radix("AA", 16).unwrap());
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_hex_str_short_notation_alpha() {
        let color = RGB::from_hex_str("#1FAD").unwrap();

        assert_eq!(color.red(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue(), u8::from_str_radix("AA", 16).unwrap());
        assert_eq!(color.alpha(), u8::from_str_radix("DD", 16).unwrap());
    }

    #[test]
    fn from_hex_str_long_notation() {
        let color = RGB::from_hex_str("#11FF0A").unwrap();

        assert_eq!(color.red(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue(), u8::from_str_radix("0A", 16).unwrap());
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_hex_str_long_notation_alpha() {
        let color = RGB::from_hex_str("#11FF0AD4").unwrap();

        assert_eq!(color.red(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue(), u8::from_str_radix("0A", 16).unwrap());
        assert_eq!(color.alpha(), u8::from_str_radix("D4", 16).unwrap());
    }

    #[test]
    fn from_rgb_str_invalid_syntax() {
        let result = RGB::from_rgb_str("rgb(");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }

    #[test]
    fn from_rgb_str_invalid_numbers() {
        let result = RGB::from_rgb_str("rgb(0 255 999)");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::IntegerConversionFailed ( .. )));
    }

    #[test]
    fn from_rgb_str_regular() {
        let color = RGB::from_rgb_str("rgb(0 255 128)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_with_alpha() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 1)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 255);
    }

    #[test]
    fn from_rgb_str_with_alpha_decimal() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 0.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }
}

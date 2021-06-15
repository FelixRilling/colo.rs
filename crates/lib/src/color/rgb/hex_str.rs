use crate::color::rgb::RGB;
use crate::error::ParsingError;

impl RGB {
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
                        Ok(RGB::from_rgb_with_alpha(red, green, blue, alpha))
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
                        Ok(RGB::from_rgb_with_alpha(red, green, blue, alpha))
                    }
                    _ => unreachable!()
                }
            }
            _ => Err(ParsingError::InvalidSyntax("Unexpected length. String must have either 3, 4, 6, or 8 hexadecimal digits"))
        }
    }

    pub fn to_hex_str(&self) -> String {
        // TODO support custom output format (uppercase/lowercase and/or short notation)
        if self.is_opaque() {
            format!("#{:02X}{:02X}{:02X}", self.red(), self.green(), self.blue())
        } else {
            format!("#{:02X}{:02X}{:02X}{:02X}", self.red(), self.green(), self.blue(), self.alpha())
        }
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
}

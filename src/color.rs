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
            return Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax { details: "Missing '#'" } });
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
            _ => Err(ParsingError { kind: ParsingErrorKind::InvalidSyntax { details: "Unexpected length" } })
        }
    }

    pub fn to_hex_str(&self) -> String {
        format!("#{:X}{:X}{:X}", self.red, self.green, self.blue)
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

use crate::color::rgb::{OmitAlphaChannel, RGB};
use crate::error::ParsingError;

/// Represents case of hexadecimal letters.
#[derive(Debug, PartialEq, Eq)]
pub enum LetterCase {
    Uppercase,
    Lowercase,
}

/// The shorthand (single digit per channel) notation may be used if the double digit notation is the same digit two times.
#[derive(Debug, PartialEq, Eq)]
pub enum ShorthandNotation {
    Never,
    IfPossible,
}

fn can_shorthand_channel(channel_hex_str: &str) -> bool {
    debug_assert!(channel_hex_str.len() == 2);

    channel_hex_str[0..1] == channel_hex_str[1..2]
}

fn shorthand_channel(channel_hex_str: &str) -> String {
    debug_assert!(channel_hex_str.len() == 2);
    debug_assert!(can_shorthand_channel(channel_hex_str));

    String::from(&channel_hex_str[0..1])
}


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

    /// Creates a CSS-style hexadecimal string for this color.
    ///
    /// Note that values more precise than the 255 bit supported for the hexadecimal notation will lose precision in the output.
    /// RGB string notation should be used instead for these.
    pub fn to_hex_str(&self, letter_case: LetterCase, alpha_channel: OmitAlphaChannel, shorthand_notation: ShorthandNotation) -> String {
        let mut red = format!("{:02X}", self.red());
        let mut green = format!("{:02X}", self.green());
        let mut blue = format!("{:02X}", self.blue());
        let mut alpha_opt = if self.is_opaque() && alpha_channel == OmitAlphaChannel::IfOpaque {
            None
        } else {
            Some(format!("{:02X}", self.alpha()))
        };

        if shorthand_notation == ShorthandNotation::IfPossible {
            if can_shorthand_channel(&red)
                && can_shorthand_channel(&green)
                && can_shorthand_channel(&blue) {
                match alpha_opt.as_ref() {
                    Some(alpha) => if can_shorthand_channel(alpha) {
                        red = shorthand_channel(&red);
                        green = shorthand_channel(&green);
                        blue = shorthand_channel(&blue);
                        alpha_opt = Some(shorthand_channel(alpha));
                    },
                    None => {
                        red = shorthand_channel(&red);
                        green = shorthand_channel(&green);
                        blue = shorthand_channel(&blue);
                    }
                }
            }
        }

        let hex_str = alpha_opt.map_or_else(
            || format!("#{}{}{}", red, green, blue),
            |alpha| format!("#{}{}{}{}", red, green, blue, alpha),
        );

        if letter_case == LetterCase::Lowercase { hex_str.to_lowercase() } else { hex_str }
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
    fn to_hex_str_uppercase() {
        let color = RGB::from_hex_str("#11FF0A").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_lowercase() {
        let color = RGB::from_hex_str("#11FF0A").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Lowercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11ff0a");
    }

    #[test]
    fn to_hex_str_omit_alpha_channel_opaque() {
        let color = RGB::from_hex_str("#11FF0AFF").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_omit_alpha_channel_non_opaque() {
        let color = RGB::from_hex_str("#11FF0A99").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11FF0A99");
    }

    #[test]
    fn to_hex_str_omit_alpha_never() {
        let color = RGB::from_hex_str("#11FF0AFF").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::Never,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11FF0AFF");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible() {
        let color = RGB::from_hex_str("#11FF00").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
        );
        assert_eq!(hex_string, "#1F0");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible() {
        let color = RGB::from_hex_str("#1BF701").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
        );
        assert_eq!(hex_string, "#1BF701");
    }

    #[test]
    fn to_hex_str_shorthand_notation_never() {
        let color = RGB::from_hex_str("#11FF00").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
        );
        assert_eq!(hex_string, "#11FF00");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible_alpha() {
        let color = RGB::from_hex_str("#11FF0066").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
        );
        assert_eq!(hex_string, "#1F06");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible_alpha() {
        let color = RGB::from_hex_str("#11FF00AB").unwrap();

        let hex_string = color.to_hex_str(
            LetterCase::Uppercase,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
        );
        assert_eq!(hex_string, "#11FF00AB");
    }
}

use log::trace;

use crate::component::{FloatComponent, SingleByteComponent};
use crate::error::ParsingError;
use crate::rgb::{OmitAlphaChannel, Rgb, RgbChannel};

/// Represents the case of hexadecimal letters.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LetterCase {
    Uppercase,
    Lowercase,
}

/// If shorthand (single digit per channel) notation may be used if the double digit notation is the same digit two times.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ShorthandNotation {
    Never,
    IfPossible,
}

fn can_shorthand_hexadecimal_channel(channel_hex_str: &str) -> bool {
    debug_assert!(channel_hex_str.len() == 2);

    channel_hex_str[0..1] == channel_hex_str[1..2]
}

fn shorthand_hexadecimal_channel(channel_hex_str: &str) -> String {
    debug_assert!(channel_hex_str.len() == 2);
    debug_assert!(can_shorthand_hexadecimal_channel(channel_hex_str));

    String::from(&channel_hex_str[0..1])
}

fn parse_shorthand_hexadecimal_channel(seq: &str) -> Result<RgbChannel, ParsingError> {
    debug_assert!(seq.len() == 1);

    let expanded_seq = seq.repeat(2);
    Ok(RgbChannel::from_u8(u8::from_str_radix(&expanded_seq, 16)?))
}

fn parse_hexadecimal_channel(seq: &str) -> Result<RgbChannel, ParsingError> {
    debug_assert!(seq.len() == 2);

    Ok(RgbChannel::from_u8(u8::from_str_radix(seq, 16)?))
}

impl Rgb {
    /// Parses a CSS-style hex color notation string.
    /// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#hex-notation).
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Missing the '#' character at the start of the string.
    /// - Non-hexadecimal digits.
    /// - A length of the digit part not equal to 3, 4, 6 or 8.
    pub fn from_hex_str(hex_str: &str) -> Result<Rgb, ParsingError> {
        if !hex_str.starts_with('#') {
            return Err(ParsingError::InvalidSyntax("Missing '#'"));
        }
        let hex_digits = &hex_str[1..];
        let len = hex_digits.len();
        let (red, green, blue, alpha_opt) =
            match len {
                3 | 4 => {
                    trace!("Parsing hex color as shorthand notation.");
                    // In the shorthand notation, the hex digit is simply repeated, so e.g "F" becomes "FF".
                    let red = parse_shorthand_hexadecimal_channel(&hex_digits[0..1])?;
                    let green = parse_shorthand_hexadecimal_channel(&hex_digits[1..2])?;
                    let blue = parse_shorthand_hexadecimal_channel(&hex_digits[2..3])?;
                    trace!(
                        "Parsed color channel values r='{}', g='{}', b='{}'.",
                        red.value(),
                        green.value(),
                        blue.value()
                    );

                    let alpha = match len {
                        3 => {
                            trace!("No alpha channel found.");
                            None
                        }
                        4 => {
                            let alpha = parse_shorthand_hexadecimal_channel(&hex_digits[3..4])?;
                            trace!("Parsed alpha channel value a='{}'.", alpha.value());
                            Some(alpha)
                        }
                        _ => unreachable!(),
                    };

                    (red, green, blue, alpha)
                }
                6 | 8 => {
                    trace!("Parsing hex color as full notation.");
                    let red = parse_hexadecimal_channel(&hex_digits[0..2])?;
                    let green = parse_hexadecimal_channel(&hex_digits[2..4])?;
                    let blue = parse_hexadecimal_channel(&hex_digits[4..6])?;
                    trace!(
                        "Parsed color channel values r='{}', g='{}', b='{}'.",
                        red.value(),
                        green.value(),
                        blue.value()
                    );

                    let alpha = match len {
                        6 => {
                            trace!("No alpha channel found.");
                            None
                        }
                        8 => {
                            let alpha = parse_hexadecimal_channel(&hex_digits[6..8])?;
                            trace!("Parsed alpha channel value a='{}'.", alpha.value());
                            Some(alpha)
                        }
                        _ => unreachable!(),
                    };

                    (red, green, blue, alpha)
                }
                _ => return Err(ParsingError::InvalidSyntax(
                    "Unexpected length. String must have either 3, 4, 6, or 8 hexadecimal digits",
                )),
            };

        Ok(match alpha_opt {
            None => {
                let color = Rgb::from_channels(red, green, blue);
                trace!("Created opaque color '{}'.", &color);
                color
            }
            Some(alpha) => {
                let color = Rgb::from_channels_with_alpha(red, green, blue, alpha);
                trace!("Created color '{}'.", &color);
                color
            }
        })
    }

    /// Creates a CSS-style hex color notation string for this color.
    /// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#hex-notation).
    ///
    /// Note that values more precise than the 8 bit supported for the hexadecimal notation will lose precision in the output.
    /// A RGB function string should be used instead for these. See [`channels_fit_in_u8`](#method.channels_fit_in_u8) for details.
    pub fn to_hex_str(
        &self,
        omit_alpha_channel: OmitAlphaChannel,
        shorthand_notation: ShorthandNotation,
        letter_case: LetterCase,
    ) -> String {
        let mut red_str = format!("{:02X}", self.red().to_u8_round());
        let mut green_str = format!("{:02X}", self.green().to_u8_round());
        let mut blue_str = format!("{:02X}", self.blue().to_u8_round());
        trace!(
            "Formatted color channel values r='{}', g='{}', b='{}'.",
            &red_str,
            &green_str,
            &blue_str
        );

        // TODO: also omit alpha if it isn't technically opaque but equals FF after rounding (e.g alpha = 0.999999).
        let mut alpha_str_opt =
            if self.is_opaque() && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
                trace!("Omitting alpha channel from output.");
                None
            } else {
                let alpha_str = format!("{:02X}", self.alpha().to_u8_round());
                trace!("Formatted alpha channel value a='{}'.", &alpha_str);
                Some(alpha_str)
            };

        if shorthand_notation == ShorthandNotation::IfPossible
            && can_shorthand_hexadecimal_channel(&red_str)
            && can_shorthand_hexadecimal_channel(&green_str)
            && can_shorthand_hexadecimal_channel(&blue_str)
        {
            trace!("Color channels support shorthand syntax.");
            if let Some(ref alpha) = alpha_str_opt {
                if can_shorthand_hexadecimal_channel(alpha) {
                    trace!("Alpha channel supports shorthand syntax.");

                    red_str = shorthand_hexadecimal_channel(&red_str);
                    green_str = shorthand_hexadecimal_channel(&green_str);
                    blue_str = shorthand_hexadecimal_channel(&blue_str);
                    trace!(
                        "Shorthanded color channel values r='{}', g='{}', b='{}'.",
                        &red_str,
                        &green_str,
                        &blue_str
                    );

                    let shorthand_alpha_str = shorthand_hexadecimal_channel(alpha);
                    trace!(
                        "Shorthanded alpha channel value a='{}'.",
                        &shorthand_alpha_str
                    );
                    alpha_str_opt = Some(shorthand_alpha_str);
                }
            } else {
                trace!("Alpha channel does not exist, skipping alpha shorthand check.");

                red_str = shorthand_hexadecimal_channel(&red_str);
                green_str = shorthand_hexadecimal_channel(&green_str);
                blue_str = shorthand_hexadecimal_channel(&blue_str);
                trace!(
                    "Shorthanded color channel values r='{}', g='{}', b='{}'.",
                    &red_str,
                    &green_str,
                    &blue_str
                );
            }
        }

        let hex_str = alpha_str_opt.map_or_else(
            || format!("#{}{}{}", &red_str, &green_str, &blue_str),
            |alpha_str| format!("#{}{}{}{}", &red_str, &green_str, &blue_str, &alpha_str),
        );
        trace!("Created hex string '{}'.", &hex_str);

        if letter_case == LetterCase::Lowercase {
            let lowercase_hex_str = hex_str.to_lowercase();
            trace!("Use lowercase hex string '{}'.", &lowercase_hex_str);
            lowercase_hex_str
        } else {
            trace!("Use uppercase hex string '{}'.", &hex_str);
            hex_str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_hex_str_errors_for_no_hash() {
        let result = Rgb::from_hex_str("112233");

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ))
    }

    #[test]
    fn from_hex_str_invalid_chars() {
        let result = Rgb::from_hex_str("#XX2233");

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ParsingError::NumberConversionFailed(..)
        ))
    }

    #[test]
    fn from_hex_str_invalid_length() {
        let result_too_long = Rgb::from_hex_str("#1111111111111111111111");
        assert!(result_too_long.is_err());
        assert!(matches!(
            result_too_long.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ));

        let result_between_short_and_long = Rgb::from_hex_str("#11223");
        assert!(result_between_short_and_long.is_err());
        assert!(matches!(
            result_between_short_and_long.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ));

        let result_between_too_short = Rgb::from_hex_str("#11");
        assert!(result_between_too_short.is_err());
        assert!(matches!(
            result_between_too_short.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ));
    }

    #[test]
    fn from_hex_str_short_notation() {
        let color = Rgb::from_hex_str("#1FA").unwrap();

        assert_eq!(color.red().to_u8_round(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green().to_u8_round(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue().to_u8_round(), u8::from_str_radix("AA", 16).unwrap());
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_hex_str_short_notation_alpha() {
        let color = Rgb::from_hex_str("#1FAD").unwrap();

        assert_eq!(color.red().to_u8_round(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green().to_u8_round(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue().to_u8_round(), u8::from_str_radix("AA", 16).unwrap());
        assert_eq!(color.alpha().to_u8_round(), u8::from_str_radix("DD", 16).unwrap());
    }

    #[test]
    fn from_hex_str_long_notation() {
        let color = Rgb::from_hex_str("#11FF0A").unwrap();

        assert_eq!(color.red().to_u8_round(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green().to_u8_round(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue().to_u8_round(), u8::from_str_radix("0A", 16).unwrap());
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_hex_str_long_notation_alpha() {
        let color = Rgb::from_hex_str("#11FF0AD4").unwrap();

        assert_eq!(color.red().to_u8_round(), u8::from_str_radix("11", 16).unwrap());
        assert_eq!(color.green().to_u8_round(), u8::from_str_radix("FF", 16).unwrap());
        assert_eq!(color.blue().to_u8_round(), u8::from_str_radix("0A", 16).unwrap());
        assert_eq!(color.alpha().to_u8_round(), u8::from_str_radix("D4", 16).unwrap());
    }

    #[test]
    fn to_hex_str_omit_alpha_channel_opaque() {
        let color = Rgb::from_hex_str("#11FF0AFF").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_omit_alpha_channel_non_opaque() {
        let color = Rgb::from_hex_str("#11FF0A99").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A99");
    }

    #[test]
    fn to_hex_str_omit_alpha_never() {
        let color = Rgb::from_hex_str("#11FF0AFF").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::Never,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0AFF");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible() {
        let color = Rgb::from_hex_str("#11FF00").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1F0");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible() {
        let color = Rgb::from_hex_str("#1BF701").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1BF701");
    }

    #[test]
    fn to_hex_str_shorthand_notation_never() {
        let color = Rgb::from_hex_str("#11FF00").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF00");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible_alpha() {
        let color = Rgb::from_hex_str("#11FF0066").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1F06");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible_alpha() {
        let color = Rgb::from_hex_str("#11FF00AB").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF00AB");
    }

    #[test]
    fn to_hex_str_uppercase() {
        let color = Rgb::from_hex_str("#11FF0A").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_lowercase() {
        let color = Rgb::from_hex_str("#11FF0A").unwrap();

        let hex_string = color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Lowercase,
        );
        assert_eq!(hex_string, "#11ff0a");
    }
}

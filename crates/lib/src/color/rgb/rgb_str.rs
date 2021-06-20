use regex::Regex;
use rug::Float;

use crate::color::rgb::DEFAULT_SRGB_PRECISION;
use crate::color::rgb::RGB;
use crate::error::ParsingError;

/// Parses a CSS number (e.g. '1.2' as a float 1.2).
// https://www.w3.org/TR/css-values-3/#number
fn parse_number(seq: &str) -> Result<Float, ParsingError> {
    Ok(Float::with_val(DEFAULT_SRGB_PRECISION, Float::parse(seq)?))
}

/// Checks if something can be parsed as a CSS percentage.
fn is_percentage(seq: &str) -> bool {
    seq.ends_with('%')
}

/// Parses a CSS percentage (e.g. '60%' as a float 0.6).
// https://www.w3.org/TR/css-values-3/#percentage-value
fn parse_percentage(seq: &str) -> Result<Float, ParsingError> {
    debug_assert!(is_percentage(seq));

    let index_of_percentage_sign = seq.rfind('%').unwrap();
    let percentage_number = parse_number(&seq[..index_of_percentage_sign])?;
    Ok(percentage_number / 100)
}

fn parse_color_channel(seq: &str) -> Result<Float, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        channel_val = parse_number(seq)? / u8::MAX;
    }
    Ok(channel_val.clamp(&0, &1))
}

// https://www.w3.org/TR/css-color-4/#typedef-alpha-value
fn parse_alpha_channel(seq: &str) -> Result<Float, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        // When parsing the alpha channel, the value ranges from 0 to 1 already.
        channel_val = parse_number(seq)?;
    }
    Ok(channel_val.clamp(&0, &1))
}


impl RGB {
    /// Parses a CSS-style RGB string representation of an RGB color.
    /// For a list of supported formats, see <https://www.w3.org/TR/css-color-4/#rgb-functions>.
    /// Note that according to the spec, values out-of-range are clamped.
    ///
    /// Note that the legacy syntax with comma or the `rgba` function are *not* supported.
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Input not matching the shape of an RGB string.
    pub fn from_rgb_str(rgb_str: &str) -> Result<RGB, ParsingError> {
        // https://regex101.com/r/MZkxf8/1
        let rgb_regex = Regex::new(
            r"^rgb\((?P<red>[-+]?(?:\d+\.)?\d+%?) (?P<green>[-+]?(?:\d+\.)?\d+%?) (?P<blue>[-+]?(?:\d+\.)?\d+%?)(?: / (?P<alpha>[-+]?(?:\d+\.)?\d+%?))?\)$"
        )?;

        match rgb_regex.captures(rgb_str) {
            None => Err(ParsingError::InvalidSyntax("String did not match RGB pattern")),
            Some(captures) => {
                let red_str = captures.name("red").unwrap().as_str();
                let green_str = captures.name("green").unwrap().as_str();
                let blue_str = captures.name("blue").unwrap().as_str();

                if is_percentage(red_str) != is_percentage(green_str) ||
                    is_percentage(red_str) != is_percentage(blue_str) {
                    return Err(ParsingError::InvalidSyntax("Unexpected combination of percentage and absolute values"));
                }

                let red = parse_color_channel(red_str)?;
                let green = parse_color_channel(green_str)?;
                let blue = parse_color_channel(blue_str)?;

                match captures.name("alpha") {
                    None => Ok(RGB::from_srgb(red, green, blue)),
                    Some(alpha_match) => {
                        let alpha = parse_alpha_channel(alpha_match.as_str())?;
                        Ok(RGB::from_srgb_with_alpha(red, green, blue, alpha))
                    }
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb_str_invalid_syntax() {
        let result = RGB::from_rgb_str("rgb(");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }

    #[test]
    fn from_rgb_str_integer_above_range() {
        let color = RGB::from_rgb_str("rgb(0 255 999)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MAX);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integer_below_range() {
        let color = RGB::from_rgb_str("rgb(0 255 -128)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MIN);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integer() {
        let color = RGB::from_rgb_str("rgb(0 255 128)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integer_decimal() {
        let color = RGB::from_rgb_str("rgb(0 255 127.99)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_above_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 1.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_below_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / -0.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MIN);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 0.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_above_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 150%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_below_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / -50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MIN);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_above_range() {
        let color = RGB::from_rgb_str("rgb(0% 100% 150%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MAX);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_percentage_below_range() {
        let color = RGB::from_rgb_str("rgb(0% 100% -50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MIN);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_percentage() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_percentage_decimal() {
        let color = RGB::from_rgb_str("rgb(0% 100% 49.99%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_decimal() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50% / 0.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_percentage() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50% / 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    fn from_rgb_str_disallow_number_mix() {
        let result = RGB::from_rgb_str("rgb(255 100% 128)");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }
}

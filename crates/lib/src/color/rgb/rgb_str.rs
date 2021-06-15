use std::str::FromStr;

use regex::Regex;
use rug::Float;

use crate::color::rgb::RGB;
use crate::color::srgb::{SRGB_PRECISION, srgb_to_rgb};
use crate::error::ParsingError;

impl RGB {
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
    fn from_rgb_str_integers_with_alpha_decimal_above_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 1.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MIN);
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
    #[ignore]
    fn from_rgb_str_integers_with_alpha_percentage_above_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 150%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_integers_with_alpha_percentage_below_range() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / -50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MIN);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_integers_with_alpha_percentage() {
        let color = RGB::from_rgb_str("rgb(0 255 128 / 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_percentage_above_range() {
        let color = RGB::from_rgb_str("rgb(0% 100% 150%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MAX);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_percentage_below_range() {
        let color = RGB::from_rgb_str("rgb(0% 100% -50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), u8::MIN);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_percentage() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), u8::MAX);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_percentage_with_alpha_decimal() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50% / 0.5)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_percentage_with_alpha_percentage() {
        let color = RGB::from_rgb_str("rgb(0% 100% 50% / 50%)").unwrap();

        assert_eq!(color.red(), 0);
        assert_eq!(color.green(), 255);
        assert_eq!(color.blue(), 128);
        assert_eq!(color.alpha(), 128);
    }

    #[test]
    #[ignore]
    fn from_rgb_str_disallow_number_mix() {
        let result = RGB::from_rgb_str("rgb(255 100% 128)");

        assert!(result.is_err());
    }
}

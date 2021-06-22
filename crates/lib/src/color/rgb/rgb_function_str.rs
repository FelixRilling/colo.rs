use regex::Regex;
use rug::Float;

use crate::color::css_types::{format_number, format_percentage, is_percentage, parse_number, parse_percentage};
use crate::color::rgb::{OmitAlphaChannel, SrgbChannel};
use crate::color::rgb::Rgb;
use crate::color::rgb::srgb::{SRGB_CHANNEL_RANGE, SRGB_SINGLE_BYTE_CHANNEL_RANGE};
use crate::error::ParsingError;

fn clamp_in_channel_range(channel_val: Float) -> Float {
    channel_val.clamp(SRGB_CHANNEL_RANGE.start(), SRGB_CHANNEL_RANGE.end())
}

fn parse_color_channel(seq: &str) -> Result<SrgbChannel, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        channel_val = parse_number(seq)? / SRGB_SINGLE_BYTE_CHANNEL_RANGE.end();
    }
    Ok(SrgbChannel::with_val(clamp_in_channel_range(channel_val)))
}

// https://www.w3.org/TR/css-color-4/#typedef-alpha-value
fn parse_alpha_channel(seq: &str) -> Result<SrgbChannel, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        // When parsing the alpha channel, the value ranges from 0 to 1 already.
        channel_val = parse_number(seq)?;
    }
    Ok(SrgbChannel::with_val(clamp_in_channel_range(channel_val)))
}


fn format_color_channel(color_channel: &SrgbChannel, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(&(color_channel.value().clone() * SRGB_SINGLE_BYTE_CHANNEL_RANGE.end())),
        ChannelUnit::Percentage => format_percentage(color_channel.value())
    }
}

fn format_alpha_channel(alpha_channel: &SrgbChannel, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(alpha_channel.value()),
        ChannelUnit::Percentage => format_percentage(alpha_channel.value())
    }
}


/// Possible CSS types able to represent an sRGB channel value.
#[derive(Debug, PartialEq, Eq)]
pub enum ChannelUnit {
    Number,
    Percentage,
}


impl Rgb {
    /// Parses a CSS-style RGB function string.
    /// For a list of supported formats, see <https://www.w3.org/TR/css-color-4/#rgb-functions>.
    ///
    /// Note that according to the spec, values out-of-range are clamped.
    ///
    /// Note that the legacy syntax with comma or the `rgba` function are *not* supported.
    ///
    /// Note that only the lowercase function name 'rgb' is supported.
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Input not matching the shape of an RGB string.
    pub fn from_rgb_function_str(rgb_str: &str) -> Result<Rgb, ParsingError> {
        // https://regex101.com/r/MZkxf8/1
        let rgb_regex = Regex::new(
            r"^rgb\((?P<red>[-+]?(?:\d+\.)?\d+%?) (?P<green>[-+]?(?:\d+\.)?\d+%?) (?P<blue>[-+]?(?:\d+\.)?\d+%?)(?: / (?P<alpha>[-+]?(?:\d+\.)?\d+%?))?\)$"
        ).expect("Could not build RGB function string pattern.");

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
                    None => Ok(Rgb::from_channels(red, green, blue)),
                    Some(alpha_match) => {
                        let alpha = parse_alpha_channel(alpha_match.as_str())?;
                        Ok(Rgb::from_channels_with_alpha(red, green, blue, alpha))
                    }
                }
            }
        }
    }

    /// Creates a CSS-style RGB function string for this color.
    pub fn to_rgb_function_str(&self, omit_alpha_channel: OmitAlphaChannel, color_channel_unit: ChannelUnit, alpha_channel_unit: ChannelUnit) -> String {
        let red = format_color_channel(self.red(), &color_channel_unit);
        let green = format_color_channel(self.green(), &color_channel_unit);
        let blue = format_color_channel(self.blue(), &color_channel_unit);
        let alpha_opt = if self.is_opaque() && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
            None
        } else {
            Some(format_alpha_channel(self.alpha(), &alpha_channel_unit))
        };

        alpha_opt.map_or_else(
            || format!("rgb({} {} {})", red, green, blue),
            |alpha| format!("rgb({} {} {} / {})", red, green, blue, alpha),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_rgb_str_invalid_syntax() {
        let result = Rgb::from_rgb_function_str("rgb(");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }

    #[test]
    fn from_rgb_str_integer_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 999)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 255);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integer_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 -128)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 0);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integer() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integer_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 127.99)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 1.5)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / -0.5)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 0);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 0.5)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 128);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 150%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / -50%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 0);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 50%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 150%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 255);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% -50%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 0);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 49.99%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50% / 0.5)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50% / 50%)").unwrap();

        assert_eq!(color.red().to_u8(), 0);
        assert_eq!(color.green().to_u8(), 255);
        assert_eq!(color.blue().to_u8(), 128);
        assert_eq!(color.alpha().to_u8(), 128);
    }

    #[test]
    fn from_rgb_str_disallow_number_mix() {
        let result = Rgb::from_rgb_function_str("rgb(255 100% 128)");

        assert!(result.is_err());
        assert!(matches!(result.err().unwrap(), ParsingError::InvalidSyntax ( .. )));
    }


    #[test]
    fn to_rgb_str_omit_alpha_channel_opaque() {
        let color = Rgb::from_channels(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0)");
    }

    #[test]
    fn to_rgb_str_omit_alpha_channel_non_opaque() {
        let color = Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(0),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0 / 0%)");
    }

    #[test]
    fn to_rgb_str_omit_alpha_never() {
        let color = Rgb::from_channels(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::Never,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0 / 100%)");
    }

    #[test]
    fn to_rgb_str_number_color_channel() {
        let color = Rgb::from_channels
            (SrgbChannel::from_u8(128),
             SrgbChannel::from_u8(255),
             SrgbChannel::from_u8(0),
            );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(128 255 0)");
    }

    #[test]
    fn to_rgb_str_number_color_channel_decimals() {
        let color = Rgb::from_channels(
            SrgbChannel::with_val(Float::with_val(64, 0.525)),
            SrgbChannel::with_val(Float::with_val(64, 0.125)),
            SrgbChannel::with_val(Float::with_val(64, 0.901)),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(133.875 31.875 229.755)");
    }

    #[test]
    fn to_rgb_str_percentage_color_channel() {
        let color = Rgb::from_channels(
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0%)");
    }

    #[test]
    fn to_rgb_str_percentage_color_channel_decimals() {
        let color = Rgb::from_channels(
            SrgbChannel::with_val(Float::with_val(64, 0.5)),
            SrgbChannel::with_val(Float::with_val(64, 0.125)),
            SrgbChannel::with_val(Float::with_val(64, 0.901)),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(50% 12.5% 90.1%)");
    }

    #[test]
    fn to_rgb_str_number_alpha_channel() {
        let color = Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::Never,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0% / 1)");
    }

    #[test]
    fn to_rgb_str_percentage_alpha_channel() {
        let color = Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::Never,
            ChannelUnit::Percentage,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0% / 100%)");
    }
}

use lazy_static::lazy_static;
use log::{trace, warn};
use regex::Regex;
use rug::Float;

use crate::component::{FloatComponent, SINGLE_BYTE_COMPONENT_VALUE_RANGE};
use crate::component::FLOAT_COMPONENT_VALUE_RANGE;
use crate::css_types::{
    format_number, format_percentage, is_percentage, parse_number, parse_percentage,
};
use crate::error::ParsingError;
use crate::rgb::{OmitAlphaChannel, RgbChannel};
use crate::rgb::Rgb;

fn clamp_in_channel_range(channel_val: Float) -> Float {
    if !FLOAT_COMPONENT_VALUE_RANGE.contains(&channel_val) {
        warn!(
            "Channel value '{}' is out of RGB component range, it will be clamped.",
            &channel_val
        );
    }
    channel_val.clamp(
        FLOAT_COMPONENT_VALUE_RANGE.start(),
        FLOAT_COMPONENT_VALUE_RANGE.end(),
    )
}

fn parse_color_channel(seq: &str) -> Result<RgbChannel, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        channel_val = parse_number(seq)? / SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();
    }
    Ok(RgbChannel::from_value(clamp_in_channel_range(channel_val)))
}

// https://www.w3.org/TR/css-color-4/#typedef-alpha-value
fn parse_alpha_channel(seq: &str) -> Result<RgbChannel, ParsingError> {
    let channel_val: Float;
    if is_percentage(seq) {
        channel_val = parse_percentage(&seq)?;
    } else {
        // When parsing the alpha channel, the value ranges from 0 to 1 already.
        channel_val = parse_number(seq)?;
    }
    Ok(RgbChannel::from_value(clamp_in_channel_range(channel_val)))
}

fn format_color_channel(color_channel: &RgbChannel, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(
            &(color_channel.value().clone() * SINGLE_BYTE_COMPONENT_VALUE_RANGE.end()),
        ),
        ChannelUnit::Percentage => format_percentage(color_channel.value()),
    }
}

fn format_alpha_channel(alpha_channel: &RgbChannel, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(alpha_channel.value()),
        ChannelUnit::Percentage => format_percentage(alpha_channel.value()),
    }
}

/// Possible CSS types able to represent an RGB component value.
#[derive(Debug, PartialEq, Eq)]
pub enum ChannelUnit {
    Number,
    Percentage,
}

impl Rgb {
    /// Parses a CSS-style RGB function string.
    /// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#rgb-functions).
    ///
    /// Note that the legacy syntax with comma or the `rgba` function are *not* supported.
    ///
    /// # Errors
    /// A malformed input will result in an error. This may include but is not limited to:
    /// - Input not matching the shape of an RGB string.
    pub fn from_rgb_function_str(rgb_str: &str) -> Result<Rgb, ParsingError> {
        // https://regex101.com/r/MZkxf8/1
        lazy_static! {
            static ref RGB_FUNCTION_REGEX: Regex = Regex::new(
                r"(?i)^rgb\((?P<red>[-+]?(?:\d+\.)?\d+%?) (?P<green>[-+]?(?:\d+\.)?\d+%?) (?P<blue>[-+]?(?:\d+\.)?\d+%?)(?: / (?P<alpha>[-+]?(?:\d+\.)?\d+%?))?\)$"
            ).expect("Could not build RGB function string pattern.");
        }

        match RGB_FUNCTION_REGEX.captures(rgb_str) {
            None => Err(ParsingError::InvalidSyntax(
                "String did not match RGB function string pattern",
            )),
            Some(captures) => {
                let red_str = captures.name("red").unwrap().as_str();
                let green_str = captures.name("green").unwrap().as_str();
                let blue_str = captures.name("blue").unwrap().as_str();
                trace!(
                    "Found RGB function string color channel values r='{}', g='{}', b='{}'.",
                    &red_str,
                    &green_str,
                    &blue_str
                );

                if is_percentage(red_str) != is_percentage(green_str)
                    || is_percentage(red_str) != is_percentage(blue_str)
                {
                    return Err(ParsingError::InvalidSyntax(
                        "Unexpected combination of percentage and absolute values",
                    ));
                }

                let red = parse_color_channel(red_str)?;
                let green = parse_color_channel(green_str)?;
                let blue = parse_color_channel(blue_str)?;
                trace!(
                    "Parsed color channel values r='{}', g='{}', b='{}'.",
                    red.value(),
                    green.value(),
                    blue.value()
                );

                match captures.name("alpha") {
                    None => {
                        trace!("No alpha channel found.");
                        let color = Rgb::from_channels(red, green, blue);
                        trace!("Created opaque color '{}'.", &color);
                        Ok(color)
                    }
                    Some(alpha_match) => {
                        let alpha_str = alpha_match.as_str();
                        trace!("Found alpha channel value a='{}'.", &alpha_str);

                        let alpha = parse_alpha_channel(alpha_str)?;
                        trace!("Parsed alpha channel value a='{}'.", alpha.value());

                        let color = Rgb::from_channels_with_alpha(red, green, blue, alpha);
                        trace!("Created color '{}'.", &color);
                        Ok(color)
                    }
                }
            }
        }
    }

    /// Creates a CSS-style RGB function string for this color.
    /// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#rgb-functions).
    pub fn to_rgb_function_str(
        &self,
        omit_alpha_channel: OmitAlphaChannel,
        color_channel_unit: ChannelUnit,
        alpha_channel_unit: ChannelUnit,
    ) -> String {
        let red_str = format_color_channel(self.red(), &color_channel_unit);
        let green_str = format_color_channel(self.green(), &color_channel_unit);
        let blue_str = format_color_channel(self.blue(), &color_channel_unit);
        trace!(
            "Formatted color channel values r='{}', g='{}', b='{}'.",
            &red_str,
            &green_str,
            &blue_str
        );

        let alpha_str_opt = if self.is_opaque() && omit_alpha_channel == OmitAlphaChannel::IfOpaque
        {
            trace!("Omitting alpha channel from output.");
            None
        } else {
            let alpha_str = format_alpha_channel(self.alpha(), &alpha_channel_unit);
            trace!("Formatted alpha channel value a='{}'.", &alpha_str);
            Some(alpha_str)
        };

        let rgb_function_str = alpha_str_opt.map_or_else(
            || format!("rgb({} {} {})", &red_str, &green_str, &blue_str),
            |alpha| {
                format!(
                    "rgb({} {} {} / {})",
                    &red_str, &green_str, &blue_str, &alpha
                )
            },
        );
        trace!("Created RGB function string '{}'.", &rgb_function_str);
        rgb_function_str
    }
}

#[cfg(test)]
mod tests {
    use crate::component::SingleByteComponent;

    use super::*;

    #[test]
    fn from_rgb_str_invalid_syntax() {
        let result = Rgb::from_rgb_function_str("rgb(");

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ));
    }

    #[test]
    fn from_rgb_str_ignores_case() {
        let color = Rgb::from_rgb_function_str("rGB(0 255 128)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integer_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 999)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 255);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integer_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 -128)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 0);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integer() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integer_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 127.99)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 1.5)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / -0.5)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 0);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 0.5)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 128);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 150%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / -50%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 0);
    }

    #[test]
    fn from_rgb_str_integers_with_alpha_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0 255 128 / 50%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_above_range() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 150%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 255);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_below_range() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% -50%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 0);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 49.99%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 255);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_decimal() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50% / 0.5)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 128);
    }

    #[test]
    fn from_rgb_str_percentage_with_alpha_percentage() {
        let color = Rgb::from_rgb_function_str("rgb(0% 100% 50% / 50%)").unwrap();

        assert_eq!(color.red().to_u8_round(), 0);
        assert_eq!(color.green().to_u8_round(), 255);
        assert_eq!(color.blue().to_u8_round(), 128);
        assert_eq!(color.alpha().to_u8_round(), 128);
    }

    #[test]
    fn from_rgb_str_disallow_number_mix() {
        let result = Rgb::from_rgb_function_str("rgb(255 100% 128)");

        assert!(result.is_err());
        assert!(matches!(
            result.err().unwrap(),
            ParsingError::InvalidSyntax(..)
        ));
    }

    #[test]
    fn to_rgb_str_omit_alpha_channel_opaque() {
        let color = Rgb::from_channels(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
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
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(0),
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
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
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
        let color = Rgb::from_channels(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
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
        let red = RgbChannel::from_value(Float::with_val(64, 1) / 1000);
        let green = RgbChannel::from_value(Float::with_val(64, 1) / 10_000);
        let blue = RgbChannel::from_value(Float::with_val(64, 1) / 100_000);
        let color = Rgb::from_channels(red, green, blue);

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0.255 0.0255 0.00255)");
    }

    #[test]
    fn to_rgb_str_percentage_color_channel() {
        let color = Rgb::from_channels(
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
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
        let red = RgbChannel::from_value(Float::with_val(64, 1) / 1000);
        let green = RgbChannel::from_value(Float::with_val(64, 1) / 10_000);
        let blue = RgbChannel::from_value(Float::with_val(64, 1) / 100_000);
        let color = Rgb::from_channels(red, green, blue);

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0.1% 0.01% 0.001%)");
    }

    #[test]
    fn to_rgb_str_number_alpha_channel() {
        let color = Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
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
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
        );

        let rgb_string = color.to_rgb_function_str(
            OmitAlphaChannel::Never,
            ChannelUnit::Percentage,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0% / 100%)");
    }
}

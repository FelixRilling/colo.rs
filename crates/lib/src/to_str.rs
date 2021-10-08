use log::trace;
use palette::Srgba;

use crate::util::is_opaque;

/// If the alpha channel may be omitted if its opaque.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OmitAlphaChannel {
    Never,
    IfOpaque,
}

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

/// Creates a CSS-style hex color notation string for this color.
/// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#hex-notation).
///
/// Note that values more precise than the 8 bit supported for the hexadecimal notation will lose precision in the output.
/// A RGB function string should be used instead for these. See [`channels_fit_in_u8`](#method.channels_fit_in_u8) for details.
pub fn to_hex_str(
    color: &Srgba,
    omit_alpha_channel: OmitAlphaChannel,
    shorthand_notation: ShorthandNotation,
    letter_case: LetterCase,
) -> String {
    let color_u8: Srgba<u8> = color.into_format().into();

    let mut red_str = format!("{:02X}", color_u8.red);
    let mut green_str = format!("{:02X}", color_u8.green);
    let mut blue_str = format!("{:02X}", color_u8.blue);
    trace!(
        "Formatted color channel values r='{}', g='{}', b='{}'.",
        &red_str,
        &green_str,
        &blue_str
    );

    // TODO: also omit alpha if it isn't technically opaque but equals FF after rounding (e.g alpha = 0.999999).
    let mut alpha_str_opt = if is_opaque(color) && omit_alpha_channel == OmitAlphaChannel::IfOpaque
    {
        trace!("Omitting alpha channel from output.");
        None
    } else {
        let alpha_str = format!("{:02X}", color_u8.alpha);
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

/// Formats a float as a CSS number (e.g. `0.6` as `'0.6'`).
fn format_number(val: f32) -> String {
    format!("{}", val)
}

/// Formats a float as a CSS percentage (e.g. `0.6` as `'60%'`).
fn format_percentage(val: f32) -> String {
    format!("{}%", val * 100f32)
}

fn format_color_channel(color_channel: f32, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(color_channel * 255f32),
        ChannelUnit::Percentage => format_percentage(color_channel),
    }
}

fn format_alpha_channel(alpha_channel: f32, unit: &ChannelUnit) -> String {
    match unit {
        ChannelUnit::Number => format_number(alpha_channel),
        ChannelUnit::Percentage => format_percentage(alpha_channel),
    }
}

/// Possible CSS types able to represent an RGB component value.
#[derive(Debug, PartialEq, Eq)]
pub enum ChannelUnit {
    Number,
    Percentage,
}

pub fn to_rgb_function_str(
    color: &Srgba,
    omit_alpha_channel: OmitAlphaChannel,
    color_channel_unit: ChannelUnit,
    alpha_channel_unit: ChannelUnit,
) -> String {
    let red_str = format_color_channel(color.red, &color_channel_unit);
    let green_str = format_color_channel(color.green, &color_channel_unit);
    let blue_str = format_color_channel(color.blue, &color_channel_unit);
    trace!(
        "Formatted color channel values r='{}', g='{}', b='{}'.",
        &red_str,
        &green_str,
        &blue_str
    );

    let alpha_str_opt = if is_opaque(&color) && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
        trace!("Omitting alpha channel from output.");
        None
    } else {
        let alpha_str = format_alpha_channel(color.alpha, &alpha_channel_unit);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_hex_str_omit_alpha_channel_opaque() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_omit_alpha_channel_non_opaque() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0x99).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A99");
    }

    #[test]
    fn to_hex_str_omit_alpha_never() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::Never,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0AFF");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1F0");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible() {
        let color: Srgba = Srgba::<u8>::new(0x1b, 0xf7, 0x01, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1BF701");
    }

    #[test]
    fn to_hex_str_shorthand_notation_never() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF00");
    }

    #[test]
    fn to_hex_str_shorthand_notation_possible_alpha() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x00, 0x66).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#1F06");
    }

    #[test]
    fn to_hex_str_shorthand_notation_not_possible_alpha() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xab).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF00AB");
    }

    #[test]
    fn to_hex_str_uppercase() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Uppercase,
        );
        assert_eq!(hex_string, "#11FF0A");
    }

    #[test]
    fn to_hex_str_lowercase() {
        let color: Srgba = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff).into_format();

        let hex_string = to_hex_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::Never,
            LetterCase::Lowercase,
        );
        assert_eq!(hex_string, "#11ff0a");
    }

    #[test]
    fn to_rgb_str_omit_alpha_channel_opaque() {
        let color: Srgba = Srgba::<u8>::new(128, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0)");
    }

    #[test]
    fn to_rgb_str_omit_alpha_channel_non_opaque() {
        let color: Srgba = Srgba::<u8>::new(128, 255, 0, 0).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0 / 0%)");
    }

    #[test]
    fn to_rgb_str_omit_alpha_never() {
        let color: Srgba = Srgba::<u8>::new(128, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::Never,
            ChannelUnit::Number,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(128 255 0 / 100%)");
    }

    #[test]
    fn to_rgb_str_number_color_channel() {
        let color: Srgba = Srgba::<u8>::new(128, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(128 255 0)");
    }

    #[test]
    fn to_rgb_str_number_color_channel_decimals() {
        let color: Srgba = Srgba::<f32>::new(1f32 / 512f32, 1f32, 0f32, 1f32);

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0.5 0.123 0.9)");
    }

    #[test]
    fn to_rgb_str_percentage_color_channel() {
        let color: Srgba = Srgba::<u8>::new(0, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0%)");
    }

    #[test]
    fn to_rgb_str_percentage_color_channel_decimals() {
        let color: Srgba = Srgba::<f32>::new(0.005f32, 1f32, 0f32, 1f32);

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0.5% 100% 0%)");
    }

    #[test]
    fn to_rgb_str_number_alpha_channel() {
        let color: Srgba = Srgba::<u8>::new(0, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::Never,
            ChannelUnit::Percentage,
            ChannelUnit::Number,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0% / 1)");
    }

    #[test]
    fn to_rgb_str_percentage_alpha_channel() {
        let color: Srgba = Srgba::<u8>::new(0, 255, 0, 255).into_format();

        let rgb_string = to_rgb_function_str(
            &color,
            OmitAlphaChannel::Never,
            ChannelUnit::Percentage,
            ChannelUnit::Percentage,
        );
        assert_eq!(rgb_string, "rgb(0% 100% 0% / 100%)");
    }
}

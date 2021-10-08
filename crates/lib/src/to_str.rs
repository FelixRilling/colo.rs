use lazy_static::lazy_static;
use log::{trace, warn};
use palette::{Alpha, IntoColor, Srgba, WithAlpha};
use palette::rgb::Rgb;

use crate::model::rgb::{ChannelUnit, is_opaque, OmitAlphaChannel};

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



/// Formats a float as a CSS number (e.g. `0.6` as `'0.6'`).
fn format_number(val: f32) -> String {
    val.to_string()
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

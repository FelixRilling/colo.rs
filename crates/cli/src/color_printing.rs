use std::io::Write;

use rug::Float;
use termcolor::{ColorSpec, StandardStream, WriteColor};

use color_utils::component::SingleByteComponent;
use color_utils::contrast::contrast_ratio_val;
use color_utils::rgb::{
    ChannelUnit, LetterCase, OmitAlphaChannel, Rgb, RgbChannel, ShorthandNotation,
};

use crate::color_format::ColorFormat;

fn rgb_as_term_color(color: &Rgb) -> termcolor::Color {
    termcolor::Color::Rgb(
        color.red().to_u8_round(),
        color.green().to_u8_round(),
        color.blue().to_u8_round(),
    )
}

/// Finds and returns the `color_options` value that has the best contrast to `initial_color`.
fn get_best_contrast<'a>(initial_color: &'a Rgb, color_options: &'a [Rgb]) -> &'a Rgb {
    let mut best_contrast_ratio: Float = Float::with_val(1, 0.0);
    // Default value only matters if all options have zero contrast, so they should be the same as initial_color anyways.
    let mut best_contrast_ratio_color: &Rgb = initial_color;

    for color_option in color_options {
        let contrast_ratio = contrast_ratio_val(initial_color, color_option);
        if contrast_ratio > best_contrast_ratio {
            best_contrast_ratio = contrast_ratio;
            best_contrast_ratio_color = color_option;
        }
    }

    best_contrast_ratio_color
}

// TODO: Allow customization of formatting flags.
fn format_color(color: &Rgb, format: &ColorFormat) -> String {
    match format {
        ColorFormat::Auto => color.to_string(),
        ColorFormat::RgbHex => color.to_hex_str(
            OmitAlphaChannel::IfOpaque,
            ShorthandNotation::IfPossible,
            LetterCase::Uppercase,
        ),
        ColorFormat::RgbFunction => color.to_rgb_function_str(
            OmitAlphaChannel::IfOpaque,
            ChannelUnit::Number,
            ChannelUnit::Number,
        ),
    }
}

/// Prints colored color value to stream. Stream color is reset afterwards.
pub fn print_color(
    stdout: &mut StandardStream,
    color: &Rgb,
    format: &ColorFormat,
) -> std::io::Result<()> {
    let black = Rgb::from_channels(
        RgbChannel::from_u8(0),
        RgbChannel::from_u8(0),
        RgbChannel::from_u8(0),
    );
    let white = Rgb::from_channels(
        RgbChannel::from_u8(255),
        RgbChannel::from_u8(255),
        RgbChannel::from_u8(255),
    );
    let foreground_color_options = [black, white];
    let foreground_color = get_best_contrast(color, &foreground_color_options);

    stdout.set_color(
        ColorSpec::new()
            .set_bg(Some(rgb_as_term_color(color)))
            .set_fg(Some(rgb_as_term_color(foreground_color))),
    )?;
    write!(stdout, "{}", format_color(color, format))?;
    stdout.set_color(&ColorSpec::default())
}

#[cfg(test)]
mod tests {
    use color_utils::rgb::Rgb;

    use super::*;

    #[test]
    fn get_best_contrast_finds_result() {
        let black = Rgb::from_channels(
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(0),
        );
        let white = Rgb::from_channels(
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(255),
            RgbChannel::from_u8(255),
        );
        let options = [black.clone(), white.clone()];

        let bright_color = Rgb::from_hex_str("#ABCDEF").unwrap();
        let bright_color_best_contrast_actual = get_best_contrast(&bright_color, &options);
        assert_eq!(*bright_color_best_contrast_actual, black);

        let dark_color = Rgb::from_hex_str("#696969").unwrap();
        let dark_color_best_contrast_actual = get_best_contrast(&dark_color, &options);
        assert_eq!(*dark_color_best_contrast_actual, white);
    }
}

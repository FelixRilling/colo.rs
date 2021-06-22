use std::io::Write;

use rug::Float;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use color_utils::color::rgb::{DEFAULT_SRGB_PRECISION, SrgbChannel};
use color_utils::color::rgb::Rgb;
use color_utils::contrast::contrast_ratio_val;

fn rgb_as_term_color(color: &Rgb) -> Color {
    Color::Rgb(color.red().to_u8(), color.green().to_u8(), color.blue().to_u8())
}

/// Finds and returns the `color_options` value that has the best contrast to `initial_color`.
fn get_best_contrast<'a>
(initial_color: &'a Rgb, color_options: &'a Vec<&Rgb>) -> &'a Rgb {
    let mut best_contrast_ratio: Float = Float::with_val(DEFAULT_SRGB_PRECISION, 0.0);
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

/// Prints colored color value to stream. Stream color is reset afterwards.
pub(crate) fn print_color(stdout: &mut StandardStream, color: &Rgb) {
    let black = Rgb::from_channels(SrgbChannel::from_u8(0), SrgbChannel::from_u8(0), SrgbChannel::from_u8(0));
    let white = Rgb::from_channels(SrgbChannel::from_u8(255), SrgbChannel::from_u8(255), SrgbChannel::from_u8(255));
    let foreground_color_options = vec![&black, &white];
    let foreground_color = get_best_contrast(color, &foreground_color_options);

    stdout.set_color(ColorSpec::new()
        .set_bg(Some(rgb_as_term_color(color)))
        .set_fg(Some(rgb_as_term_color(foreground_color))))
        .expect("Could not set stdout color.");
    write!(stdout, "{}", color).expect("Could not write color to stdout.");
    stdout.set_color(&ColorSpec::default()).expect("Could not reset stdout color.");
}


#[cfg(test)]
mod tests {
    use color_utils::color::rgb::Rgb;

    use super::*;

    #[test]
    fn get_best_contrast_finds_result() {
        let black = Rgb::from_channels(SrgbChannel::from_u8(0), SrgbChannel::from_u8(0), SrgbChannel::from_u8(0));
        let white = Rgb::from_channels(SrgbChannel::from_u8(255), SrgbChannel::from_u8(255), SrgbChannel::from_u8(255));
        let options = vec![&black, &white];

        let bright_color = Rgb::from_hex_str("#ABCDEF").unwrap();
        let bright_color_best_contrast_actual = get_best_contrast(&bright_color, &options);
        assert_eq!(bright_color_best_contrast_actual, &black);

        let dark_color = Rgb::from_hex_str("#696969").unwrap();
        let dark_color_best_contrast_actual = get_best_contrast(&dark_color, &options);
        assert_eq!(dark_color_best_contrast_actual, &white);
    }
}

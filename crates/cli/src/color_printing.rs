use std::io::Write;

use palette::{IntoColor, IntoComponent, RelativeContrast, Srgb, Srgba, WithAlpha};
use termcolor::{ColorSpec, StandardStream, WriteColor};

use color_utils::to_str::{
	ChannelUnit, LetterCase, OmitAlphaChannel, ShorthandNotation, to_hsl_function_str,
	to_rgb_function_str, to_rgb_hex_str,
};

use crate::color_format::ColorFormat;

fn rgb_as_term_color(color: &Srgb) -> termcolor::Color {
	termcolor::Color::Rgb(
		color.red.into_component(),
		color.green.into_component(),
		color.blue.into_component(),
	)
}

/// Finds and returns the `color_options` value that has the best contrast to `initial_color`.
fn get_best_contrast<'a>(initial_color: &'a Srgb, color_options: &'a [Srgb]) -> &'a Srgb {
	let mut best_contrast_ratio: f32 = 0.0;
	// Default value only matters if all options have zero contrast, so they should be the same as initial_color anyway.
	let mut best_contrast_ratio_color: &Srgb = initial_color;

	for color_option in color_options {
		let contrast_ratio = initial_color.get_contrast_ratio(color_option);
		if contrast_ratio > best_contrast_ratio {
			best_contrast_ratio = contrast_ratio;
			best_contrast_ratio_color = color_option;
		}
	}

	best_contrast_ratio_color
}

// TODO: Allow customization of formatting flags.
fn format_color(color: &Srgba, format: ColorFormat) -> String {
	match format {
		ColorFormat::Auto => to_rgb_hex_str(
			&color.into_format(),
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		),
		ColorFormat::RgbHex => to_rgb_hex_str(
			&color.into_format(),
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		),
		ColorFormat::RgbFunction => to_rgb_function_str(
			color,
			OmitAlphaChannel::IfOpaque,
			ChannelUnit::Number,
			ChannelUnit::Number,
		),
		ColorFormat::HslFunction => {
			to_hsl_function_str(&(*color).into_color(), OmitAlphaChannel::IfOpaque,
								ChannelUnit::Number)
		}
	}
}

/// Prints colored color value to stream. Stream color is reset afterwards.
pub fn print_color(
	stdout: &mut StandardStream,
	color: &Srgba,
	format: ColorFormat,
) -> std::io::Result<()> {
	let opaque_color: Srgb = color.without_alpha();

	let black = Srgb::from_components((0.0, 0.0, 0.0));
	let white = Srgb::from_components((1.0, 1.0, 1.0));
	let foreground_color_options = [black, white];
	let foreground_color = get_best_contrast(&opaque_color, &foreground_color_options);

	stdout.set_color(
		ColorSpec::new()
			.set_bg(Some(rgb_as_term_color(&opaque_color)))
			.set_fg(Some(rgb_as_term_color(foreground_color))),
	)?;
	write!(stdout, "{}", format_color(color, format))?;
	stdout.set_color(&ColorSpec::default())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_best_contrast_finds_result() {
		let black = Srgb::from_components((0.0, 0.0, 0.0));
		let white = Srgb::from_components((1.0, 1.0, 1.0));
		let options = [black, white];

		let bright_color = Srgb::from_components((0.9, 0.85, 1.0));
		let bright_color_best_contrast_actual = get_best_contrast(&bright_color, &options);
		assert_eq!(*bright_color_best_contrast_actual, black);

		let dark_color = Srgb::from_components((0.0, 0.1, 0.25));
		let dark_color_best_contrast_actual = get_best_contrast(&dark_color, &options);
		assert_eq!(*dark_color_best_contrast_actual, white);
	}
}

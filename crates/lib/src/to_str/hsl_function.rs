use palette::{Hsla, IntoColor};

use crate::to_str::{ChannelUnit, OmitAlphaChannel};
use crate::to_str::css_types::{format_alpha_value, format_hue, format_percentage};
use crate::util::is_opaque;

/// Creates a CSS-style HSL function string for this color.
/// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#the-hsl-notation).
pub fn to_hsl_function_str(
	color: &Hsla,
	omit_alpha_channel: OmitAlphaChannel,
	alpha_channel_unit: ChannelUnit,
) -> String {
	let hue_str = format_hue(color.hue);
	let saturation_str = format_percentage(color.saturation);
	let lightness_str = format_percentage(color.lightness);

	let alpha_str_opt = if is_opaque(&(*color).into_color())
		&& omit_alpha_channel == OmitAlphaChannel::IfOpaque
	{
		None
	} else {
		Some(format_alpha_value(color.alpha, alpha_channel_unit))
	};

	alpha_str_opt.map_or_else(
		|| {
			format!(
				"hsl({} {} {})",
				&hue_str,
				&saturation_str,
				&lightness_str
			)
		},
		|alpha_str| {
			format!(
				"hsl({} {} {} / {})",
				&hue_str,
				&saturation_str,
				&lightness_str,
				&alpha_str
			)
		},
	)
}

#[cfg(test)]
mod tests {
	use palette::RgbHue;

	use super::*;

	#[test]
	fn to_hsl_function_str_omit_alpha_channel_opaque() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::IfOpaque, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 50% 75%)");
	}

	#[test]
	fn to_hsl_function_str_omit_alpha_channel_non_opaque() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 0.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::IfOpaque, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 50% 75% / 0%)");
	}

	#[test]
	fn to_hsl_function_str_omit_alpha_never() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 50% 75% / 100%)");
	}

	#[test]
	fn to_hsl_function_str_number_alpha_channel() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Number);
		assert_eq!(hsl_string, "hsl(180deg 50% 75% / 1)");
	}

	#[test]
	fn to_hsl_function_str_percentage_alpha_channel() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 50% 75% / 100%)");
	}
}

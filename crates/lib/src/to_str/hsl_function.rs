use log::trace;
use palette::{Hsla, IntoColor};

use crate::to_str::{ChannelUnit, OmitAlphaChannel};
use crate::to_str::css_types::{format_alpha_value, format_hue, format_number};
use crate::util::is_opaque;

pub fn to_hsl_function_str(
	color: &Hsla,
	omit_alpha_channel: OmitAlphaChannel,
	alpha_channel_unit: ChannelUnit,
) -> String {
	let hue_str = format_hue(color.hue);
	let saturation_str = format_number(color.saturation);
	let lightness_str = format_number(color.lightness);
	trace!(
		"Formatted channel values hue='{}', saturation='{}', lightness='{}'.",
		&hue_str,
		&saturation_str,
		&lightness_str
	);

	let alpha_str_opt = if is_opaque(&color.clone().into_color())
		&& omit_alpha_channel == OmitAlphaChannel::IfOpaque
	{
		trace!("Omitting alpha channel from output.");
		None
	} else {
		let alpha_str = format_alpha_value(color.alpha, alpha_channel_unit);
		trace!("Formatted alpha channel value a='{}'.", &alpha_str);
		Some(alpha_str)
	};

	let hsl_function_str = alpha_str_opt.map_or_else(
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
	);
	trace!("Created HSL function string '{}'.", &hsl_function_str);
	hsl_function_str
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
		assert_eq!(hsl_string, "hsl(180deg 0.5 0.75)");
	}

	#[test]
	fn to_hsl_function_str_omit_alpha_channel_non_opaque() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 0.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::IfOpaque, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 0.5 0.75 / 0%)");
	}

	#[test]
	fn to_hsl_function_str_omit_alpha_never() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 0.5 0.75 / 100%)");
	}

	#[test]
	fn to_hsl_function_str_number_alpha_channel() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Number);
		assert_eq!(hsl_string, "hsl(180deg 0.5 0.75 / 1)");
	}

	#[test]
	fn to_hsl_function_str_percentage_alpha_channel() {
		let color: Hsla = Hsla::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hsl_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hsl(180deg 0.5 0.75 / 100%)");
	}
}

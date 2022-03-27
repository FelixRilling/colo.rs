use log::trace;
use palette::{Hwba, IntoColor};

use crate::to_str::{ChannelUnit, OmitAlphaChannel};
use crate::to_str::css_types::{format_alpha_value, format_hue, format_percentage};
use crate::util::is_opaque;

/// Creates a CSS-style HWB function string for this color.
/// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#the-hwb-notation).
pub fn to_hwb_function_str(
	color: &Hwba,
	omit_alpha_channel: OmitAlphaChannel,
	alpha_channel_unit: ChannelUnit,
) -> String {
	let hue_str = format_hue(color.hue);
	let whiteness_str = format_percentage(color.whiteness);
	let blackness_str = format_percentage(color.blackness);
	trace!(
		"Formatted channel values hue='{}', whiteness='{}', blackness='{}'.",
		&hue_str,
		&whiteness_str,
		&blackness_str
	);

	let alpha_str_opt = if is_opaque(&(*color).into_color())
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
				"hwb({} {} {})",
				&hue_str,
				&whiteness_str,
				&blackness_str
			)
		},
		|alpha_str| {
			format!(
				"hwb({} {} {} / {})",
				&hue_str,
				&whiteness_str,
				&blackness_str,
				&alpha_str
			)
		},
	);
	trace!("Created HWB function string '{}'.", &hsl_function_str);
	hsl_function_str
}

#[cfg(test)]
mod tests {
	use palette::RgbHue;

	use super::*;

	#[test]
	fn to_hwb_function_str_omit_alpha_channel_opaque() {
		let color: Hwba = Hwba::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hwb_function_str(&color, OmitAlphaChannel::IfOpaque, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hwb(180deg 50% 75%)");
	}

	#[test]
	fn to_hwb_function_str_omit_alpha_channel_non_opaque() {
		let color: Hwba = Hwba::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 0.0);

		let hsl_string =
			to_hwb_function_str(&color, OmitAlphaChannel::IfOpaque, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hwb(180deg 50% 75% / 0%)");
	}

	#[test]
	fn to_hwb_function_str_omit_alpha_never() {
		let color: Hwba = Hwba::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hwb_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hwb(180deg 50% 75% / 100%)");
	}

	#[test]
	fn to_hwb_function_str_number_alpha_channel() {
		let color: Hwba = Hwba::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hwb_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Number);
		assert_eq!(hsl_string, "hwb(180deg 50% 75% / 1)");
	}

	#[test]
	fn to_hwb_function_str_percentage_alpha_channel() {
		let color: Hwba = Hwba::new(RgbHue::from_degrees(180.0), 0.5, 0.75, 1.0);

		let hsl_string =
			to_hwb_function_str(&color, OmitAlphaChannel::Never, ChannelUnit::Percentage);
		assert_eq!(hsl_string, "hwb(180deg 50% 75% / 100%)");
	}
}

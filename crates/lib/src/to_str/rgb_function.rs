use log::trace;
use palette::Srgba;

use crate::to_str::css_util::{format_alpha_value, format_number, format_percentage};
use crate::to_str::OmitAlphaChannel;
use crate::util::is_opaque;

fn format_color_channel(color_channel: f32, unit: ChannelUnit) -> String {
	match unit {
		ChannelUnit::Number => format_number(color_channel * 255.0),
		ChannelUnit::Percentage => format_percentage(color_channel),
	}
}

/// Possible CSS types able to represent an RGB component value.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChannelUnit {
	Number,
	Percentage,
}

/// Creates a CSS-style RGB function string for this color.
/// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#rgb-functions).
pub fn to_rgb_function_str(
	color: &Srgba,
	omit_alpha_channel: OmitAlphaChannel,
	color_channel_unit: ChannelUnit,
	alpha_channel_unit: ChannelUnit,
) -> String {
	let red_str = format_color_channel(color.red, color_channel_unit);
	let green_str = format_color_channel(color.green, color_channel_unit);
	let blue_str = format_color_channel(color.blue, color_channel_unit);
	trace!(
		"Formatted color channel values r='{}', g='{}', b='{}'.",
		&red_str,
		&green_str,
		&blue_str
	);

	let alpha_str_opt = if is_opaque(color) && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
		trace!("Omitting alpha channel from output.");
		None
	} else {
		let alpha_str = format_alpha_value(color.alpha, alpha_channel_unit);
		trace!("Formatted alpha channel value a='{}'.", &alpha_str);
		Some(alpha_str)
	};

	let rgb_function_str = alpha_str_opt.map_or_else(
		|| format!("rgb({} {} {})", &red_str, &green_str, &blue_str),
		|alpha_str| {
			format!(
				"rgb({} {} {} / {})",
				&red_str, &green_str, &blue_str, &alpha_str
			)
		},
	);
	trace!("Created RGB function string '{}'.", &rgb_function_str);
	rgb_function_str
}

#[cfg(test)]
mod tests {
	use palette::Srgba;

	use super::*;

	#[test]
	fn to_rgb_function_str_omit_alpha_channel_opaque() {
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
	fn to_rgb_function_str_omit_alpha_channel_non_opaque() {
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
	fn to_rgb_function_str_omit_alpha_never() {
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
	fn to_rgb_function_str_number_color_channel() {
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
	fn to_rgb_function_str_number_color_channel_decimals() {
		let color: Srgba = Srgba::<f32>::new(1f32 / 512f32, 1f32, 0f32, 1f32);

		let rgb_string = to_rgb_function_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ChannelUnit::Number,
			ChannelUnit::Number,
		);
		assert_eq!(rgb_string, "rgb(0.5 255 0)");
	}

	#[test]
	fn to_rgb_function_str_percentage_color_channel() {
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
	fn to_rgb_function_str_percentage_color_channel_decimals() {
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
	fn to_rgb_function_str_number_alpha_channel() {
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
	fn to_rgb_function_str_percentage_alpha_channel() {
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

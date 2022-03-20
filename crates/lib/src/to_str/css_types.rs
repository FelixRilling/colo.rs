use palette::RgbHue;

use color_utils_internal::ceil_n_decimals;

use crate::to_str::ChannelUnit;

// 2 decimal places seems to be good enough to avoid most float-related issues and still preserve most information.
const RELEVANT_DECIMAL_PLACES: u8 = 2;

/// Formats a float as a CSS number (e.g. `0.6` as `'0.6'`).
pub(crate) fn format_number(val: f32) -> String {
	format!("{}", ceil_n_decimals(val.into(), RELEVANT_DECIMAL_PLACES))
}

/// Formats a float as a CSS percentage (e.g. `0.6` as `'60%'`).
pub(crate) fn format_percentage(val: f32) -> String {
	format!(
		"{}%",
		ceil_n_decimals((val * 100.0).into(), RELEVANT_DECIMAL_PLACES)
	)
}

pub(crate) fn format_alpha_value(alpha: f32, unit: ChannelUnit) -> String {
	match unit {
		ChannelUnit::Number => format_number(alpha),
		ChannelUnit::Percentage => format_percentage(alpha),
	}
}

pub(crate) fn format_hue(hue: RgbHue) -> String {
	format!("{}deg", format_number(hue.to_positive_degrees()))
}

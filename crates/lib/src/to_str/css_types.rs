use palette::RgbHue;

use crate::to_str::ChannelUnit;

// used over default string formatting to only use decimal places if needed.
fn ceil_two_decimal_places(val: f32) -> f32 {
	(val * 100.0).ceil() / 100.0
}

/// Formats a float as a CSS number (e.g., `0.6` as `'0.6'`).
pub(crate) fn format_number(val: f32) -> String {
	format!("{}", ceil_two_decimal_places(val))
}

/// Formats a float as a CSS percentage (e.g., `0.6` as `'60%'`).
pub(crate) fn format_percentage(val: f32) -> String {
	format!("{}%", ceil_two_decimal_places(val * 100.0))
}

/// Formats a float as an alpha-value.
pub(crate) fn format_alpha_value(alpha: f32, unit: ChannelUnit) -> String {
	match unit {
		ChannelUnit::Number => format_number(alpha),
		ChannelUnit::Percentage => format_percentage(alpha),
	}
}

/// Formats a hue as degrees.
pub(crate) fn format_hue(hue: RgbHue) -> String {
	format!("{}deg", format_number(hue.into_positive_degrees()))
}

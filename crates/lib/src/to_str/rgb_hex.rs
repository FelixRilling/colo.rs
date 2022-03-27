use palette::Srgba;

use crate::to_str::OmitAlphaChannel;
use crate::util::is_opaque;

/// Represents the case of hexadecimal letters.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LetterCase {
	Uppercase,
	Lowercase,
}

/// If the shorthand (single digit per channel) notation may be used if the double-digit notation is
/// the same digit two times.
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

fn format_hex(channel: u8) -> String {
	format!("{:02X}", channel)
}

/// Creates a CSS-style hex color notation string for this color.
/// For details see the [CSS color specification](https://www.w3.org/TR/css-color-4/#hex-notation).
///
/// Note that values more precise than the 8 bit supported for the hexadecimal notation must be cast beforehand, which might be lossy.
pub fn to_rgb_hex_str(
	color: &Srgba<u8>,
	omit_alpha_channel: OmitAlphaChannel,
	shorthand_notation: ShorthandNotation,
	letter_case: LetterCase,
) -> String {
	let mut red_str = format_hex(color.red);
	let mut green_str = format_hex(color.green);
	let mut blue_str = format_hex(color.blue);

	let mut alpha_str_opt =
		if is_opaque(&color.into_format()) && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
			None
		} else {
			let alpha_str = format_hex(color.alpha);
			Some(alpha_str)
		};

	if shorthand_notation == ShorthandNotation::IfPossible
		&& can_shorthand_hexadecimal_channel(&red_str)
		&& can_shorthand_hexadecimal_channel(&green_str)
		&& can_shorthand_hexadecimal_channel(&blue_str)
	{
		if let Some(ref alpha) = alpha_str_opt {
			if can_shorthand_hexadecimal_channel(alpha) {
				red_str = shorthand_hexadecimal_channel(&red_str);
				green_str = shorthand_hexadecimal_channel(&green_str);
				blue_str = shorthand_hexadecimal_channel(&blue_str);

				let shorthand_alpha_str = shorthand_hexadecimal_channel(alpha);
				alpha_str_opt = Some(shorthand_alpha_str);
			}
		} else {
			red_str = shorthand_hexadecimal_channel(&red_str);
			green_str = shorthand_hexadecimal_channel(&green_str);
			blue_str = shorthand_hexadecimal_channel(&blue_str);
		}
	}

	let hex_str = alpha_str_opt.map_or_else(
		|| format!("#{}{}{}", &red_str, &green_str, &blue_str),
		|alpha_str| format!("#{}{}{}{}", &red_str, &green_str, &blue_str, &alpha_str),
	);

	if letter_case == LetterCase::Lowercase {
		hex_str.to_lowercase()
	} else {
		hex_str
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn to_rgb_hex_str_omit_alpha_channel_opaque() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::Never,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF0A");
	}

	#[test]
	fn to_rgb_hex_str_omit_alpha_channel_non_opaque() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0x99);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::Never,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF0A99");
	}

	#[test]
	fn to_rgb_hex_str_omit_alpha_never() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::Never,
			ShorthandNotation::Never,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF0AFF");
	}

	#[test]
	fn to_rgb_hex_str_shorthand_notation_possible() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#1F0");
	}

	#[test]
	fn to_rgb_hex_str_shorthand_notation_not_possible() {
		let color = Srgba::<u8>::new(0x1b, 0xf7, 0x01, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#1BF701");
	}

	#[test]
	fn to_rgb_hex_str_shorthand_notation_never() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::Never,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF00");
	}

	#[test]
	fn to_rgb_hex_str_shorthand_notation_possible_alpha() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x00, 0x66);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#1F06");
	}

	#[test]
	fn to_rgb_hex_str_shorthand_notation_not_possible_alpha() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x00, 0xab);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::IfPossible,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF00AB");
	}

	#[test]
	fn to_rgb_hex_str_uppercase() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::Never,
			LetterCase::Uppercase,
		);
		assert_eq!(result, "#11FF0A");
	}

	#[test]
	fn to_rgb_hex_str_lowercase() {
		let color = Srgba::<u8>::new(0x11, 0xff, 0x0a, 0xff);

		let result = to_rgb_hex_str(
			&color,
			OmitAlphaChannel::IfOpaque,
			ShorthandNotation::Never,
			LetterCase::Lowercase,
		);
		assert_eq!(result, "#11ff0a");
	}
}

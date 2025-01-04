use log::debug;
use palette::Srgba;

use color_utils::error::ParsingError;
use color_utils::to_str::{to_rgb_function_str, ChannelUnit, OmitAlphaChannel};

pub fn parse_color(seq: &str) -> Result<Srgba, ParsingError> {
	debug!("Attempting to parse '{}'.", seq);
	let result = color_utils::parser::parse_color(seq);

	if let Ok(ref color) = result {
		debug!(
			"Parsed '{}' as '{}'.",
			seq,
			to_rgb_function_str(
				color,
				OmitAlphaChannel::Never,
				ChannelUnit::Number,
				ChannelUnit::Number
			)
		);
	}

	result
}

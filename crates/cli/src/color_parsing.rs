use log::debug;
use palette::Srgba;

use color_utils::error::ParsingError;
use color_utils::model::rgb::Rgb;
use color_utils::parser::color_as_string;

pub fn parse_color(seq: &str) -> Result<Srgba, ParsingError> {
    debug!("Attempting to parse '{}'.", seq);
    let result = color_utils::parser::parse_color(seq);

    if let Ok(ref color) = result {
        debug!("Parsed '{}' as '{}'.", seq, color_as_string(color));
    }

    result
}

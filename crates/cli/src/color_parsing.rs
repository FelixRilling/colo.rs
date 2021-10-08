use log::debug;
use palette::Srgba;

use color_utils::error::ParsingError;
use color_utils::model::rgb::Rgb;

pub fn parse_color(seq: &str) -> Result<Srgba, ParsingError> {
    debug!("Attempting to parse '{}'.", seq);
    let result = color_utils::parser::parse_color(seq);

    if let Ok(ref color) = result {
        let rgb: Rgb = color.to_owned().into();
        debug!("Parsed '{}' as '{}'.", seq, rgb);
    }

    result
}

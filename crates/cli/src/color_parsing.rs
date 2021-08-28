use log::debug;

use color_utils::error::ParsingError;
use color_utils::model::rgb::Rgb;

use crate::color_format::ColorFormat;

pub fn parse_color<'a>(seq: &'a str, format: &ColorFormat) -> Result<Rgb, ParsingError<'a>> {
    debug!("Attempting to parse '{}' using format '{}'.", seq, format);
    let result = match format {
        ColorFormat::Auto => parse_color_auto(seq),
        ColorFormat::RgbHex => Rgb::from_hex_str(seq),
        ColorFormat::RgbFunction => Rgb::from_rgb_function_str(seq),
    };

    if let Ok(ref color) = result {
        debug!("Parsed '{}' as '{}' using format '{}'.", seq, color, format);
    }

    result
}

fn parse_color_auto(seq: &str) -> Result<Rgb, ParsingError> {
    match Rgb::from_hex_str(seq) {
        Ok(color) => Ok(color),
        Err(hex_err) => {
            debug!("Could not parse '{}' as hex string: {}.", seq, &hex_err);
            match Rgb::from_rgb_function_str(seq) {
                Ok(color) => Ok(color),
                Err(rgb_function_err) => {
                    debug!(
                        "Could not parse '{}' as RGB function string: {}.",
                        seq, &rgb_function_err
                    );
                    Err(ParsingError::InvalidSyntax(
                        "Could not parse color using any supported format",
                    ))
                }
            }
        }
    }
}

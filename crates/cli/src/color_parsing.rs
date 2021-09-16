use log::debug;
use palette::Srgba;

use color_utils::error::ParsingError;
use color_utils::model::rgb::Rgb;

use crate::color_format::ColorFormat;

pub fn parse_color<'a>(seq: &'a str, format: &ColorFormat) -> Result<Srgba, ParsingError<'a>> {
    debug!("Attempting to parse '{}' using format '{}'.", seq, format);
    let result = match format {
        ColorFormat::Auto => parse_color_auto(seq),
        ColorFormat::RgbHex => Rgb::from_hex_str(seq).map(|rgb| rgb.into()),
        ColorFormat::RgbFunction => Rgb::from_rgb_function_str(seq).map(|rgb| rgb.into()),
    };

    if let Ok(ref color) = result {
        let rgb: Rgb = color.to_owned().into();
        debug!("Parsed '{}' as '{}' using format '{}'.", seq, rgb, format);
    }

    result
}

fn parse_color_auto(seq: &str) -> Result<Srgba, ParsingError> {
    match Rgb::from_hex_str(seq) {
        Ok(color) => Ok(color.into()),
        Err(hex_err) => {
            debug!("Could not parse '{}' as hex string: {}.", seq, &hex_err);
            match Rgb::from_rgb_function_str(seq) {
                Ok(color) => Ok(color.into()),
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

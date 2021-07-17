use std::fmt::{Display, Formatter};

use log::debug;

use color_utils::error::ParsingError;
use color_utils::rgb::Rgb;

#[derive(Debug, PartialEq, Eq)]
pub enum ColorFormat {
    Auto,
    RgbHex,
    RgbFunction,
}

impl Display for ColorFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorFormat::Auto => f.write_str("auto"),
            ColorFormat::RgbHex => f.write_str("rgb-hex"),
            ColorFormat::RgbFunction => f.write_str("rgb-function"),
        }
    }
}

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
            debug!(
                "Could not parse '{}' as hex string color: {}.",
                seq, &hex_err
            );
            match Rgb::from_rgb_function_str(seq) {
                Ok(color) => Ok(color),
                Err(rgb_function_err) => {
                    debug!(
                        "Could not parse '{}' as RGB function string color: {}.",
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

use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
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

impl FromStr for ColorFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorFormat::Auto),
            "rgb-hex" => Ok(ColorFormat::RgbHex),
            "rgb-function" => Ok(ColorFormat::RgbFunction),
            _ => Err(format!("invalid value: {}", s))
        }
    }
}

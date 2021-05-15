use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn from_hex_str(hex_str: &str) -> Result<RGB, std::num::ParseIntError> {
        // https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
        let r: u8 = u8::from_str_radix(&hex_str[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_str[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_str[5..7], 16)?;

        Ok(RGB { r, g, b })
    }

    pub fn to_hex_str(&self) -> String {
        format!("#{:X}{:X}{:X}", self.r, self.g, self.b)
    }
}

impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..'
    fn from_str(hex_str: &str) -> Result<Self, Self::Err> {
        RGB::from_hex_str(hex_str)
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str())
    }
}

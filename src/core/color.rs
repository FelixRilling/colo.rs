use std::str::FromStr;
use std::fmt::Display;
use std::fmt;

#[derive(Debug)]
pub struct RGB {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

// https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
impl FromStr for RGB {
    type Err = std::num::ParseIntError;

    // Parses a color hex code of the form '#rRgGbB..'
    fn from_str(hex_code: &str) -> Result<Self, Self::Err> {

        // u8::from_str_radix(src: &str, radix: u32) converts a string
        // slice in a given base to u8
        let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

        Ok(RGB { r, g, b })
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("#{:x}{:x}{:x}", self.r, self.g, self.b))
    }
}

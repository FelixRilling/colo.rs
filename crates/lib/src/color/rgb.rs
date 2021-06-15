use std::fmt;
use std::fmt::Display;

mod rgb_str;
mod hex_str;

/// Represents a single RGB color with an alpha channel.
#[derive(PartialEq, Eq, Debug)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl RGB {
    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
    }

    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> RGB {
        RGB { red, green, blue, alpha }
    }

    /// Creates a RGB instance based on the given values. alpha channel is fully opaque.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> RGB {
        RGB::from_rgba(red, green, blue, u8::MAX)
    }
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str())
    }
}

use std::fmt;
use std::fmt::Display;

use rug::Float;
use rug::float::Round;

mod rgb_str;
mod hex_str;

pub(crate) const SRGB_PRECISION: u32 = 64;

pub(crate) fn srgb_to_rgb(srgb_val: &Float) -> u8 {
    let clone = srgb_val.clone();
    let rgb_val_float = clone * u8::MAX;
    rgb_val_float.to_f64_round(Round::Up).ceil() as u8
}

pub(crate) fn rgb_to_srgb(rgb_val: u8) -> Float {
    let (rbg_val_float, _) = Float::with_val_round(SRGB_PRECISION, rgb_val, Round::Up);
    rbg_val_float / u8::MAX
}


/// Represents a single RGB color with an alpha channel.
#[derive(Debug)]
pub struct RGB {
    red: Float,
    green: Float,
    blue: Float,
    alpha: Float,
}

impl RGB {
    pub fn red(&self) -> u8 {
        srgb_to_rgb(&self.red)
    }

    pub fn green(&self) -> u8 {
        srgb_to_rgb(&self.green)
    }

    pub fn blue(&self) -> u8 {
        srgb_to_rgb(&self.blue)
    }

    pub fn alpha(&self) -> u8 {
        srgb_to_rgb(&self.alpha)
    }


    pub fn red_srgb(&self) -> &Float {
        &self.red
    }

    pub fn green_srgb(&self) -> &Float {
        &self.green
    }

    pub fn blue_srgb(&self) -> &Float {
        &self.blue
    }

    pub fn alpha_srgb(&self) -> &Float {
        &self.alpha
    }


    pub fn is_opaque(&self) -> bool {
        srgb_to_rgb(&self.alpha) == u8::MAX
    }


    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> RGB {
        RGB {
            red: rgb_to_srgb(red),
            green: rgb_to_srgb(green),
            blue: rgb_to_srgb(blue),
            alpha: rgb_to_srgb(alpha),
        }
    }

    /// Creates a RGB instance based on the given values. alpha channel is fully opaque.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> RGB {
        RGB::from_rgba(red, green, blue, u8::MAX)
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue && self.alpha == other.alpha
    }
}

impl Eq for RGB {}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str())
    }
}

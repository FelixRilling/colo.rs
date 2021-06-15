use std::fmt;
use std::fmt::Display;

use rug::Float;

mod rgb_str;
mod hex_str;

const SRGB_PRECISION: u32 = 64;

fn srgb_to_rgb(srgb_val: &Float) -> u8 {
    let rgb_val_float = srgb_val.clone() * u8::MAX;
    rgb_val_float.to_f64().ceil() as u8
}

fn rgb_to_srgb(rgb_val: u8) -> Float {
    let rbg_val_float = Float::with_val(SRGB_PRECISION, rgb_val);
    rbg_val_float / u8::MAX
}

fn srgb_max() -> Float {
    Float::with_val(SRGB_PRECISION, 1)
}

/// Represents a single RGB color with an alpha channel.
/// Note: internally stores values as sRGB channels which are not limited to 8 bits.
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
        self.alpha == srgb_max()
    }


    /// Creates a RGB instance based on the given values. alpha channel is fully opaque.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> RGB {
        RGB::from_rgb_with_alpha(red, green, blue, u8::MAX)
    }

    /// Creates a RGB instance based on the given values. alpha channel is fully opaque.
    pub fn from_srgb(red: Float, green: Float, blue: Float) -> RGB {
        RGB::from_srgb_with_alpha(red, green, blue, srgb_max())
    }

    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_rgb_with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> RGB {
        RGB::from_srgb_with_alpha(
            rgb_to_srgb(red),
            rgb_to_srgb(green),
            rgb_to_srgb(blue),
            rgb_to_srgb(alpha),
        )
    }

    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_srgb_with_alpha(red: Float, green: Float, blue: Float, alpha: Float) -> RGB {
        RGB {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl PartialEq for RGB {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red &&
            self.green == other.green &&
            self.blue == other.blue &&
            self.alpha == other.alpha
    }
}

impl Eq for RGB {}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str())
    }
}

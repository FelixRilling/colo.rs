use std::fmt;
use std::fmt::Display;

use rug::Float;

pub use crate::color::rgb::hex_str::{LetterCase, ShorthandNotation};

mod rgb_str;
mod hex_str;

pub const DEFAULT_SRGB_PRECISION: u32 = 64;

fn srgb_to_rgb(srgb_val: &Float) -> u8 {
    debug_assert!(srgb_val >= &0 && srgb_val <= &1);

    let rgb_val_float = srgb_val.clone() * u8::MAX;
    rgb_val_float.to_f64().ceil() as u8
}

fn rgb_to_srgb(rgb_val: u8) -> Float {
    let rbg_val_float = Float::with_val(DEFAULT_SRGB_PRECISION, rgb_val);
    rbg_val_float / u8::MAX
}

fn srgb_max() -> Float {
    Float::with_val(DEFAULT_SRGB_PRECISION, 1)
}

/// Represents a single RGB color with an alpha channel.
/// Note: internally stores values as sRGB channels which are not limited to 8 bits.
#[derive(Debug, PartialEq)]
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

    /// Creates a RGB instance based on the given sRGB values. alpha channel is fully opaque.
    ///
    /// # Panics
    /// If channel values are outside range 0 to 1.
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

    /// Creates a RGB instance with custom alpha channel based on the given sRGB values.
    ///
    /// # Panics
    /// If channel values are outside range 0 to 1.
    pub fn from_srgb_with_alpha(red: Float, green: Float, blue: Float, alpha: Float) -> RGB {
        assert!(red >= 0 && red <= 1);
        assert!(green >= 0 && green <= 1);
        assert!(blue >= 0 && blue <= 1);
        assert!(alpha >= 0 && alpha <= 1);

        RGB {
            red,
            green,
            blue,
            alpha,
        }
    }
}

/// The alpha channel can be omitted if its opaque.
#[derive(Debug, PartialEq, Eq)]
pub enum OmitAlphaChannel {
    Never,
    IfOpaque,
}

impl Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str(LetterCase::Uppercase, OmitAlphaChannel::IfOpaque, ShorthandNotation::Never))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outputs_internal_float_as_u8() {
        let color = RGB::from_srgb_with_alpha(
            Float::with_val(64, 0.5),
            Float::with_val(64, 0),
            Float::with_val(64, 1),
            Float::with_val(64, 0.25),
        );

        assert_eq!(color.red(), 128);
        assert_eq!(color.green(), 0);
        assert_eq!(color.blue(), 255);
        assert_eq!(color.alpha(), 64);
    }

    #[test]
    fn is_opaque_true_for_opaque() {
        assert!(RGB::from_rgb(
            128,
            64,
            0,
        ).is_opaque());

        assert!(RGB::from_rgb_with_alpha(
            128,
            64,
            0,
            255,
        ).is_opaque());
    }

    #[test]
    fn is_opaque_false_for_transparent() {
        assert!(!RGB::from_rgb_with_alpha(
            128,
            64,
            0,
            254,
        ).is_opaque());

        assert!(!RGB::from_rgb_with_alpha(
            128,
            64,
            0,
            128,
        ).is_opaque());

        assert!(!RGB::from_rgb_with_alpha(
            128,
            64,
            0,
            0,
        ).is_opaque());
    }
}

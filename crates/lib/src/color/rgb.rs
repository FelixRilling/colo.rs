use std::fmt;
use std::fmt::Display;

use rug::Float;

pub use crate::color::rgb::hex_str::{LetterCase, ShorthandNotation};
pub use crate::color::rgb::rgb_str::ChannelUnit;
use crate::color::rgb::srgb::{RGB_CHANNEL_MAX, SRGB_CHANNEL_MAX, SRGB_CHANNEL_MIN};
pub use crate::color::rgb::srgb::DEFAULT_SRGB_PRECISION;

mod srgb;
mod css_types;
mod rgb_str;
mod hex_str;

/// Represents a single RGB color with an alpha channel.
/// Note: internally stores values as sRGB channels which are not limited to 8 bits.
#[derive(Debug, PartialEq)]
pub struct RGB {
    red: Float,
    green: Float,
    blue: Float,
    alpha: Float,
}

// TODO: Add method to check if color fits in RGB (8bit) channels and a method to round to the nearest one that can.
impl RGB {
    pub fn red(&self) -> u8 {
        srgb::srgb_to_rgb(&self.red)
    }

    pub fn green(&self) -> u8 {
        srgb::srgb_to_rgb(&self.green)
    }

    pub fn blue(&self) -> u8 {
        srgb::srgb_to_rgb(&self.blue)
    }

    pub fn alpha(&self) -> u8 {
        srgb::srgb_to_rgb(&self.alpha)
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
        self.alpha == srgb::srgb_max()
    }


    /// Creates a RGB instance based on the given values. Alpha channel is fully opaque.
    pub fn from_rgb(red: u8, green: u8, blue: u8) -> RGB {
        RGB::from_rgb_with_alpha(red, green, blue, RGB_CHANNEL_MAX)
    }

    /// Creates a RGB instance based on the given sRGB values. Alpha channel is fully opaque.
    ///
    /// # Panics
    /// If channel values are outside range 0 to 1.
    pub fn from_srgb(red: Float, green: Float, blue: Float) -> RGB {
        RGB::from_srgb_with_alpha(red, green, blue, srgb::srgb_max())
    }

    /// Creates a RGB instance with custom alpha channel based on the given values.
    pub fn from_rgb_with_alpha(red: u8, green: u8, blue: u8, alpha: u8) -> RGB {
        RGB::from_srgb_with_alpha(
            srgb::rgb_to_srgb(red),
            srgb::rgb_to_srgb(green),
            srgb::rgb_to_srgb(blue),
            srgb::rgb_to_srgb(alpha),
        )
    }

    /// Creates a RGB instance with custom alpha channel based on the given sRGB values.
    ///
    /// # Panics
    /// If channel values are outside range 0 to 1.
    pub fn from_srgb_with_alpha(red: Float, green: Float, blue: Float, alpha: Float) -> RGB {
        assert!(red >= SRGB_CHANNEL_MIN && red <= SRGB_CHANNEL_MAX);
        assert!(green >= SRGB_CHANNEL_MIN && green <= SRGB_CHANNEL_MAX);
        assert!(blue >= SRGB_CHANNEL_MIN && blue <= SRGB_CHANNEL_MAX);
        assert!(alpha >= SRGB_CHANNEL_MIN && alpha <= SRGB_CHANNEL_MAX);

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
        f.write_str(&self.to_hex_str(OmitAlphaChannel::IfOpaque, ShorthandNotation::Never, LetterCase::Uppercase))
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

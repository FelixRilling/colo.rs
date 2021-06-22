use std::fmt;
use std::fmt::Display;

pub use crate::color::rgb::hex_str::{LetterCase, ShorthandNotation};
pub use crate::color::rgb::rgb_function_str::ChannelUnit;
pub use crate::color::rgb::srgb::{DEFAULT_SRGB_PRECISION, SrgbChannel};
use crate::color::rgb::srgb::srgb_max;

mod srgb;
mod rgb_function_str;
mod hex_str;

/// Represents a single [RGB](https://en.wikipedia.org/wiki/RGB_color_space) color in the RGB color space with an alpha channel.
/// sRGB is used as color space.
#[derive(Debug, PartialEq)]
pub struct Rgb {
    red: SrgbChannel,
    green: SrgbChannel,
    blue: SrgbChannel,
    alpha: SrgbChannel,
}

// TODO: Add method to check if color fits in RGB (8bit) channels and a method to round to the nearest one that can.
impl Rgb {
    pub fn red(&self) -> &SrgbChannel {
        &self.red
    }

    pub fn green(&self) -> &SrgbChannel {
        &self.green
    }

    pub fn blue(&self) -> &SrgbChannel {
        &self.blue
    }

    pub fn alpha(&self) -> &SrgbChannel {
        &self.alpha
    }

    /// Returns if this color is fully opaque.
    pub fn is_opaque(&self) -> bool {
        *self.alpha.value() == srgb::srgb_max()
    }


    /// Creates an opaque color based on the given color channels.
    pub fn from_channels(red: SrgbChannel, green: SrgbChannel, blue: SrgbChannel) -> Rgb {
        Rgb::from_channels_with_alpha(red, green, blue, SrgbChannel::with_val(srgb_max()))
    }

    /// Creates a color based on the given color and alpha channels.
    pub fn from_channels_with_alpha(red: SrgbChannel, green: SrgbChannel, blue: SrgbChannel, alpha: SrgbChannel) -> Rgb {
        Rgb { red, green, blue, alpha }
    }
}

/// The alpha channel may be omitted if its opaque.
#[derive(Debug, PartialEq, Eq)]
pub enum OmitAlphaChannel {
    Never,
    IfOpaque,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex_str(OmitAlphaChannel::IfOpaque, ShorthandNotation::Never, LetterCase::Uppercase))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_opaque_true_for_opaque() {
        assert!(Rgb::from_channels(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
        ).is_opaque());

        assert!(Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
        ).is_opaque());
    }

    #[test]
    fn is_opaque_false_for_transparent() {
        assert!(!Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(254),
        ).is_opaque());

        assert!(!Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(128),
        ).is_opaque());

        assert!(!Rgb::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(0),
        ).is_opaque());
    }
}

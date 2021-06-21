use std::fmt;
use std::fmt::Display;

pub use crate::color::rgb::hex_str::{LetterCase, ShorthandNotation};
pub use crate::color::rgb::rgb_str::ChannelUnit;
pub use crate::color::rgb::srgb::{DEFAULT_SRGB_PRECISION, SrgbChannel};
use crate::color::rgb::srgb::srgb_max;

mod srgb;
mod css_types;
mod rgb_str;
mod hex_str;

/// Represents a single [RGB](https://en.wikipedia.org/wiki/RGB_color_space) color with an alpha channel.
/// Note: internally stores values as sRGB channels and are not limited to 8 bits.
#[derive(Debug, PartialEq)]
pub struct RGB {
    red: SrgbChannel,
    green: SrgbChannel,
    blue: SrgbChannel,
    alpha: SrgbChannel,
}

// TODO: Add method to check if color fits in RGB (8bit) channels and a method to round to the nearest one that can.
impl RGB {
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


    pub fn is_opaque(&self) -> bool {
        *self.alpha.value() == srgb::srgb_max()
    }


    pub fn from_channels(red: SrgbChannel, green: SrgbChannel, blue: SrgbChannel) -> RGB {
        RGB::from_channels_with_alpha(red, green, blue, SrgbChannel::with_val(srgb_max()))
    }

    pub fn from_channels_with_alpha(red: SrgbChannel, green: SrgbChannel, blue: SrgbChannel, alpha: SrgbChannel) -> RGB {
        RGB { red, green, blue, alpha }
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
    fn is_opaque_true_for_opaque() {
        assert!(RGB::from_channels(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
        ).is_opaque());

        assert!(RGB::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(255),
        ).is_opaque());
    }

    #[test]
    fn is_opaque_false_for_transparent() {
        assert!(!RGB::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(254),
        ).is_opaque());

        assert!(!RGB::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(128),
        ).is_opaque());

        assert!(!RGB::from_channels_with_alpha(
            SrgbChannel::from_u8(128),
            SrgbChannel::from_u8(64),
            SrgbChannel::from_u8(0),
            SrgbChannel::from_u8(0),
        ).is_opaque());
    }
}

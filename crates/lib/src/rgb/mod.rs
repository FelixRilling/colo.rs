//! **R**ed **G**reen **B**lue color model.

use std::fmt;
use std::fmt::Display;

use crate::component::{FloatComponent, SingleByteComponent};
pub use crate::rgb::hex_str::{LetterCase, ShorthandNotation};
pub use crate::rgb::rgb_channel::{DEFAULT_RGB_PRECISION, RgbChannel};
use crate::rgb::rgb_channel::value_max;
pub use crate::rgb::rgb_function_str::ChannelUnit;

mod hex_str;
mod rgb_channel;
mod rgb_function_str;

/// Represents a color in the [RGB color model](https://en.wikipedia.org/wiki/RGB_color_model) (with an alpha channel).
#[derive(Debug, PartialEq, Clone)]
pub struct Rgb {
    red: RgbChannel,
    green: RgbChannel,
    blue: RgbChannel,
    alpha: RgbChannel,
}

impl Rgb {
    pub fn red(&self) -> &RgbChannel {
        &self.red
    }

    pub fn green(&self) -> &RgbChannel {
        &self.green
    }

    pub fn blue(&self) -> &RgbChannel {
        &self.blue
    }

    pub fn alpha(&self) -> &RgbChannel {
        &self.alpha
    }

    /// Returns if this color is fully opaque.
    pub fn is_opaque(&self) -> bool {
        *self.alpha.value() == rgb_channel::value_max()
    }

    /// Checks if this color can be fully represented with channels in a range from 0 to 255.
    /// See [`SingleByteComponent::fits_u8`](SingleByteComponent::fits_in_u8) for details.
    pub fn channels_fit_in_u8(&self) -> bool {
        self.red().fits_in_u8()
            && self.blue().fits_in_u8()
            && self.green().fits_in_u8()
            && self.alpha().fits_in_u8()
    }

    /// Creates an opaque color based on the given color channels.
    pub fn from_channels(red: RgbChannel, green: RgbChannel, blue: RgbChannel) -> Rgb {
        Rgb::from_channels_with_alpha(red, green, blue, RgbChannel::from_value(value_max()))
    }

    /// Creates a color based on the given color and alpha channels.
    pub fn from_channels_with_alpha(
        red: RgbChannel,
        green: RgbChannel,
        blue: RgbChannel,
        alpha: RgbChannel,
    ) -> Rgb {
        Rgb {
            red,
            green,
            blue,
            alpha,
        }
    }
}

/// If the alpha channel may be omitted if its opaque.
#[derive(Debug, PartialEq, Eq)]
pub enum OmitAlphaChannel {
    Never,
    IfOpaque,
}

impl Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.channels_fit_in_u8() {
            f.write_str(&self.to_hex_str(
                OmitAlphaChannel::IfOpaque,
                ShorthandNotation::IfPossible,
                LetterCase::Uppercase,
            ))
        } else {
            f.write_str(&self.to_rgb_function_str(
                OmitAlphaChannel::IfOpaque,
                ChannelUnit::Number,
                ChannelUnit::Number,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use rug::Float;

    use super::*;

    #[test]
    fn is_opaque_false_for_transparent() {
        assert!(!Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(254),
        )
            .is_opaque());

        assert!(!Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(128),
        )
            .is_opaque());

        assert!(!Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(0),
        )
            .is_opaque());
    }

    #[test]
    fn is_opaque_true_for_opaque() {
        assert!(Rgb::from_channels(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
        )
            .is_opaque());

        assert!(Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(255),
        )
            .is_opaque());
    }

    #[test]
    fn channels_fit_in_u8_true_if_all_fit() {
        assert!(Rgb::from_channels_with_alpha(
            RgbChannel::from_u8(128),
            RgbChannel::from_u8(64),
            RgbChannel::from_u8(0),
            RgbChannel::from_u8(0),
        )
            .channels_fit_in_u8());
    }

    #[test]
    fn channels_fit_in_u8_false_if_not_all_fit() {
        assert!(!Rgb::from_channels_with_alpha(
            RgbChannel::from_value(Float::with_val(64, 1)),
            RgbChannel::from_value(Float::with_val(64, 1)),
            RgbChannel::from_value(Float::with_val(64, 1)),
            RgbChannel::from_value(Float::with_val(64, 0.00000001)),
        )
            .channels_fit_in_u8());
    }
}

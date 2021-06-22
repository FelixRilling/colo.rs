use std::ops::RangeInclusive;

use rug::Float;

/// Floating point precision used when creating floats internally.
/// Chosen arbitrarily, but the current value seems to work based on most exploration tests.
pub const DEFAULT_SRGB_PRECISION: u32 = 64;

pub(crate) const SRGB_CHANNEL_RANGE: RangeInclusive<f64> = 0.0..=1.0;

/// As stated on <https://www.w3.org/TR/css-color-4/#rgb-functions>:
/// "0 again represents the minimum value for the color channel, but 255 represents the maximum.
/// These values come from the fact that many graphics engines store the color channels internally as a single byte,
/// which can hold integers between 0 and 255."
pub(crate) const SRGB_SINGLE_BYTE_CHANNEL_RANGE: RangeInclusive<u8> = u8::MIN..=u8::MAX;

pub(crate) fn srgb_max() -> Float {
    Float::with_val(DEFAULT_SRGB_PRECISION, SRGB_CHANNEL_RANGE.end())
}

/// [sRGB](https://en.wikipedia.org/wiki/SRGB) channel. Can hold value from 0 to 1.
#[derive(Debug, PartialEq)]
pub struct SrgbChannel {
    value: Float,
}

impl SrgbChannel {
    /// Creates a new channel with the given value. Value must be >= 0 and <= 1.
    ///
    /// # Panics
    /// If value is out of range.
    pub fn with_val(srgb_channel_val: Float) -> SrgbChannel {
        assert!(SRGB_CHANNEL_RANGE.contains(&srgb_channel_val));

        SrgbChannel { value: srgb_channel_val }
    }

    /// Creates a new channel based on the given value in the range 0 to 255.
    pub fn from_u8(rgb_channel_val: u8) -> SrgbChannel {
        let srgb_channel_val = Float::with_val(DEFAULT_SRGB_PRECISION, rgb_channel_val) / SRGB_SINGLE_BYTE_CHANNEL_RANGE.end();
        SrgbChannel::with_val(srgb_channel_val)
    }

    /// Returns the channel value as-is.
    pub fn value(&self) -> &Float {
        &self.value
    }

    /// Returns the closest value from 0 to 255 based on the channel value. Note that precision may be lost.
    pub fn to_u8(&self) -> u8 {
        let rgb_channel_val_float = self.value().clone() * SRGB_SINGLE_BYTE_CHANNEL_RANGE.end();
        rgb_channel_val_float.to_f32().ceil() as u8
    }
}

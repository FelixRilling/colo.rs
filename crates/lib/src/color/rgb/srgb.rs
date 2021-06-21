use rug::Float;

/// Floating point precision used when creating floats internally.
/// Chosen arbitrarily, but the current value seems to work based on most exploration tests.
pub const DEFAULT_SRGB_PRECISION: u32 = 64;

pub(crate) const SRGB_CHANNEL_MIN: f32 = 0.0;
pub(crate) const SRGB_CHANNEL_MAX: f32 = 1.0;

pub(crate) const RGB_CHANNEL_MAX: u8 = u8::MAX;

pub(crate) fn srgb_max() -> Float {
    Float::with_val(DEFAULT_SRGB_PRECISION, SRGB_CHANNEL_MAX)
}

/// sRGB channel. Can hold value from 0 to 1.
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
        assert!(srgb_channel_val >= SRGB_CHANNEL_MIN && srgb_channel_val <= SRGB_CHANNEL_MAX);

        SrgbChannel { value: srgb_channel_val }
    }

    /// Creates a new channel based on the given RGB value in the range 0 to 255.
    pub fn from_u8(rgb_channel_val: u8) -> SrgbChannel {
        let srgb_channel_val = Float::with_val(DEFAULT_SRGB_PRECISION, rgb_channel_val) / RGB_CHANNEL_MAX;
        SrgbChannel::with_val(srgb_channel_val)
    }

    /// Returns the channel value as-is.
    pub fn value(&self) -> &Float {
        &self.value
    }

    /// Returns the closest RGB value to the channel value. Note that precision may be lost.
    pub fn to_u8(&self) -> u8 {
        let rgb_channel_val_float = self.value().clone() * RGB_CHANNEL_MAX;
        rgb_channel_val_float.to_f32().ceil() as u8
    }
}

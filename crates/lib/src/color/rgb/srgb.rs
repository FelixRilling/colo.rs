use rug::Float;

/// Floating point precision used when creating floats internally.
/// Chosen arbitrarily, but the current value seems to work based on most exploration tests.
pub const DEFAULT_SRGB_PRECISION: u32 = 64;

pub(crate) const SRGB_CHANNEL_MIN: f32 = 0.0;
pub(crate) const SRGB_CHANNEL_MAX: f32 = 1.0;

pub(crate) const RGB_CHANNEL_MAX: u8 = u8::MAX;

pub(crate) fn srgb_to_rgb(srgb_val: &Float) -> u8 {
    debug_assert!(srgb_val >= &SRGB_CHANNEL_MIN && srgb_val <= &SRGB_CHANNEL_MAX);

    let rgb_val_float = srgb_val.clone() * RGB_CHANNEL_MAX;
    rgb_val_float.to_f32().ceil() as u8
}

pub(crate) fn rgb_to_srgb(rgb_val: u8) -> Float {
    let rbg_val_float = Float::with_val(DEFAULT_SRGB_PRECISION, rgb_val);
    rbg_val_float / RGB_CHANNEL_MAX
}

pub(crate) fn srgb_max() -> Float {
    Float::with_val(DEFAULT_SRGB_PRECISION, SRGB_CHANNEL_MAX)
}

/// sRGB channel. Can hold value from 0 to 1.
#[derive(Debug, PartialEq)]
pub struct SrgbChannel {
    value: Float,
}

impl SrgbChannel {
    /// Creates a new channel with the given value.
    pub fn with_val(value: Float) -> SrgbChannel {
        SrgbChannel { value }
    }

    /// Creates a new channel based on the given RGB value.
    pub fn from_u8(value: u8) -> SrgbChannel {
        SrgbChannel::with_val(rgb_to_srgb(value))
    }

    /// Returns the channel value as-is.
    pub fn value(&self) -> &Float {
        &self.value
    }

    /// Returns the closest RGB value to the channel value. Note that precision may be lost.
    pub fn to_u8(&self) -> u8 {
        srgb_to_rgb(self.value())
    }
}

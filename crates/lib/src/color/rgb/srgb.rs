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

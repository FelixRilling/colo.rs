use rug::Float;
use rug::float::Round;

pub const SRGB_PRECISION: u32 = 64;

pub fn srgb_to_rgb(srgb_val: Float) -> u8 {
    let rgb_val_float = srgb_val * u8::MAX;
    rgb_val_float.to_f64_round(Round::Up).ceil() as u8
}

pub fn rgb_to_srgb(rgb_val: u8) -> Float {
    let (rbg_val_float, _) = Float::with_val_round(SRGB_PRECISION, rgb_val, Round::Up);
    rbg_val_float / u8::MAX
}

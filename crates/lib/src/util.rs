use palette::{Srgba, WithAlpha};

/// Checks if the color is fully opaque
pub fn is_opaque(srgba: &Srgba) -> bool {
    srgba.eq(&srgba.with_alpha(1.0))
}

/// Checks if all channels can be represented using the u8 (number from 0 to 255) format.
pub fn channels_fit_in_u8(srgba: &Srgba<f32>) -> bool {
    channel_fits_in_u8(srgba.red)
        && channel_fits_in_u8(srgba.green)
        && channel_fits_in_u8(srgba.blue)
        && channel_fits_in_u8(srgba.alpha)
}

fn channel_fits_in_u8(channel: f32) -> bool {
    is_integer(channel * 255.0)
}

fn is_integer(tmp: f32) -> bool {
    tmp == tmp.floor()
}

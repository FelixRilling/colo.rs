use palette::{Srgba, WithAlpha};

pub fn is_opaque(srgba: &Srgba) -> bool {
    srgba.eq(&srgba.with_alpha(1.0))
}

pub fn channels_fit_in_u8(srgba: &Srgba<f32>) -> bool {
    channel_fits_in_u8(srgba.red)
        && channel_fits_in_u8(srgba.green)
        && channel_fits_in_u8(srgba.blue)
        && channel_fits_in_u8(srgba.alpha)
}

pub fn channel_fits_in_u8(channel: f32) -> bool {
    let tmp = channel * 255.0;
    tmp == tmp.floor()
}

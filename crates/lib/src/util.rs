use palette::Srgba;

// /home/rilling/.cargo/registry/src/github.com-1ecc6299db9ec823/palette-0.6.0/src/alpha.rs:70
pub fn is_opaque(srgba: &Srgba) -> bool {
    srgba.alpha == 1.0
}
pub fn channels_fit_in_u8(srgba: &Srgba) -> bool {
    channel_fit_in_u8(srgba.red)
        && channel_fit_in_u8(srgba.green)
        && channel_fit_in_u8(srgba.blue)
        && channel_fit_in_u8(srgba.alpha)
}

fn channel_fit_in_u8(channel: f32) -> bool {
    let maxed = channel * 255.0;
    maxed.floor() == maxed
}

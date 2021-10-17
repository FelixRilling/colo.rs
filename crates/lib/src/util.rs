use palette::{Srgba, WithAlpha};

/// Checks if the color is fully opaque
pub fn is_opaque(srgba: &Srgba) -> bool {
    srgba.eq(&srgba.with_alpha(1.0))
}

/// Checks if all channels can be represented using the u8 (number from 0 to 255) format.
pub fn channels_fit_in_u8(srgba: &Srgba) -> bool {
    channel_fits_in_u8(srgba.red)
        && channel_fits_in_u8(srgba.green)
        && channel_fits_in_u8(srgba.blue)
        && channel_fits_in_u8(srgba.alpha)
}

fn channel_fits_in_u8(channel: f32) -> bool {
    is_integer(channel * 255.0)
}

fn is_integer(val: f32) -> bool {
    val.eq(&val.trunc())
}

#[cfg(test)]
mod tests {
    use palette::Srgba;

    use super::*;

    #[test]
    fn is_opaque_false_for_transparent() {
        let color: Srgba = Srgba::new(1.0, 1.0, 1.0, 0.5);

        assert!(!is_opaque(&color));
    }

    #[test]
    fn is_opaque_true_for_opaque() {
        // 0.2 -> 51 in u8
        let color: Srgba = Srgba::new(1.0, 1.0, 1.0, 1.0);

        assert!(is_opaque(&color));
    }

    #[test]
    fn fits_in_u8_false_if_too_precise() {
        let color: Srgba = Srgba::new(0.0001, 1.0, 1.0, 1.0);

        assert!(!channels_fit_in_u8(&color));
    }

    #[test]
    fn fits_in_u8_true_if_fitting() {
        // 0.2 -> 51 in u8
        let color: Srgba = Srgba::new(0.2, 1.0, 1.0, 1.0);

        assert!(channels_fit_in_u8(&color));
    }
}

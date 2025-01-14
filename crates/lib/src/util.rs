use palette::{Srgba, WithAlpha};

/// Checks if the color is fully opaque
// TODO: allow any `Alpha` type
pub fn is_opaque(srgba: &Srgba) -> bool {
	srgba.eq(&srgba.with_alpha(1.0))
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
}

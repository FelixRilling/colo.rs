use rug::Float;

use crate::color::component::{FLOAT_COMPONENT_VALUE_RANGE, FloatComponent, SINGLE_BYTE_COMPONENT_VALUE_RANGE, SingleByteComponent};

/// Floating point precision used when creating floats internally.
/// Chosen arbitrarily, but the current value seems to work based on most exploration tests.
pub const DEFAULT_RGB_PRECISION: u32 = 64;

pub(crate) fn value_max() -> Float {
    Float::with_val(DEFAULT_RGB_PRECISION, FLOAT_COMPONENT_VALUE_RANGE.end())
}

/// [RGB](https://en.wikipedia.org/wiki/RGB_color_model) channel.
#[derive(Debug, PartialEq)]
pub struct RgbChannel {
    value: Float,
}

impl FloatComponent for RgbChannel {
    fn from_value(component_value: Float) -> Self {
        assert!(FLOAT_COMPONENT_VALUE_RANGE.contains(&component_value));

        RgbChannel {
            value: component_value,
        }
    }

    fn value(&self) -> &Float {
        &self.value
    }
}


impl SingleByteComponent for RgbChannel {
    fn from_u8(component_value: u8) -> RgbChannel {
        let component_value_float = Float::with_val(DEFAULT_RGB_PRECISION, component_value)
            / SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();
        RgbChannel::from_value(component_value_float)
    }

    fn to_u8(&self) -> u8 {
        let single_byte_component_value_float =
            (self.value().clone() * SINGLE_BYTE_COMPONENT_VALUE_RANGE.end()).ceil();
        // Because constructor enforces that value must be >= 0 and <=1, this conversion should never fail.
        single_byte_component_value_float
            .to_integer()
            .expect("Could not convert channel val to integer.")
            .to_u8()
            .expect("Could not convert channel val to u8.")
    }

    fn fits_in_u8(&self) -> bool {
        let single_byte_component_value_float = self.value().clone() * SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();
        single_byte_component_value_float.is_integer()
    }
}

#[cfg(test)]
mod tests {
    use crate::color::component::SingleByteComponent;

    use super::*;

    #[test]
    fn with_val_creates_with_val() {
        let float = Float::with_val(64, 1);
        let channel = RgbChannel::from_value(float.clone());

        assert_eq!(*channel.value(), float);
    }

    #[test]
    fn from_u8_converts_to_float() {
        let val: u8 = 255;
        let channel = RgbChannel::from_u8(val);

        assert_eq!(*channel.value(), Float::with_val(DEFAULT_RGB_PRECISION, 1));
    }

    #[test]
    fn to_u8_converts_from_float() {
        let float = Float::with_val(64, 1);
        let channel = RgbChannel::from_value(float);

        assert_eq!(channel.to_u8(), 255);
    }

    #[test]
    fn fits_in_u8_false_if_too_precise() {
        let float = Float::with_val(64, 0.0000000001);
        let channel = RgbChannel::from_value(float);

        assert!(!channel.fits_in_u8());
    }

    #[test]
    fn fits_in_u8_false_if_fitting() {
        let float = Float::with_val(64, 1);
        let channel = RgbChannel::from_value(float);

        assert!(channel.fits_in_u8());
    }
}

use rug::Float;

use crate::component::{
    FLOAT_COMPONENT_VALUE_RANGE, FloatComponent, SINGLE_BYTE_COMPONENT_VALUE_RANGE,
    SingleByteComponent,
};
use crate::error::RangeError;

/// Floating point precision used when creating floats internally.
// Chosen arbitrarily, but the current value seems to work based on most exploration tests.
pub const DEFAULT_RGB_PRECISION: u32 = 64;

pub(crate) fn value_max() -> Float {
    Float::with_val(DEFAULT_RGB_PRECISION, FLOAT_COMPONENT_VALUE_RANGE.end())
}

/// a single [RGB](https://en.wikipedia.org/wiki/RGB_color_model) channel.
#[derive(Debug, PartialEq, Clone)]
pub struct RgbChannel {
    value: Float,
}

impl FloatComponent for RgbChannel {
    // TODO maybe make this try
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

impl From<Float> for RgbChannel {
    fn from(val: Float) -> Self {
        RgbChannel::from_value(val)
    }
}

impl SingleByteComponent for RgbChannel {
    fn from_u8(component_value: u8) -> RgbChannel {
        let component_value_float = Float::with_val(DEFAULT_RGB_PRECISION, component_value)
            / SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();
        RgbChannel::from_value(component_value_float)
    }

    fn fits_in_u8(&self) -> bool {
        let single_byte_component_value_float =
            self.value().clone() * SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();
        single_byte_component_value_float.is_integer()
    }

    fn to_u8(&self) -> Result<u8, RangeError> {
        if self.fits_in_u8() {
            Ok(self.to_u8_round())
        } else {
            Err(RangeError("Value does not fit into 1 byte."))
        }
    }

    fn to_u8_round(&self) -> u8 {
        let single_byte_component_value_float =
            self.value().clone() * SINGLE_BYTE_COMPONENT_VALUE_RANGE.end();

        single_byte_component_value_float
            .ceil() // According to CSS color spec, rounding towards infinity is used when value is not an integer
            .to_integer()
            .expect("Could not convert channel val to integer.")
            .to_u8()// Because constructor enforces that value must be >= 0 and <=1, this conversion should never fail.
            .expect("Could not convert channel val to u8.")
    }
}

impl From<u8> for RgbChannel {
    fn from(val: u8) -> Self {
        RgbChannel::from_u8(val)
    }
}

#[cfg(test)]
mod tests {
    use crate::component::SingleByteComponent;

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

    #[test]
    fn to_u8_round_converts_from_float() {
        let float = Float::with_val(64, 1);
        let channel = RgbChannel::from_value(float);

        assert_eq!(channel.to_u8_round(), 255);
    }

    #[test]
    fn to_u8_round_rounds() {
        let float = Float::with_val(64, 0.0001);
        let channel = RgbChannel::from_value(float);

        assert_eq!(channel.to_u8_round(), 1);
    }

    #[test]
    fn to_u8_converts_from_float() {
        let float = Float::with_val(64, 1);
        let channel = RgbChannel::from_value(float);

        assert_eq!(channel.to_u8().unwrap(), 255);
    }

    #[test]
    fn to_u8_round_errors_out_of_range() {
        let float = Float::with_val(64, 0.0001);
        let channel = RgbChannel::from_value(float);

        assert!(channel.to_u8().is_err());
    }
}

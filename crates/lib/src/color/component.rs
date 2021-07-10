use std::ops::RangeInclusive;

use rug::Float;

pub(crate) const FLOAT_COMPONENT_VALUE_RANGE: RangeInclusive<f64> = 0.0..=1.0;

/// Channel that uses percentages/floats between 0 and 1 for the component value.
///
/// In the case of RGB: <https://en.wikipedia.org/wiki/RGB_color_model#Numeric_representations>
pub trait FloatComponent {
    /// Creates a new channel with the given value. Value must be >= 0 and <= 1.
    ///
    /// # Panics
    /// If value is out of range.
    fn from_value(value: Float) -> Self;

    /// Returns the channel value as-is.
    fn value(&self) -> &Float;
}

pub(crate) const SINGLE_BYTE_COMPONENT_VALUE_RANGE: RangeInclusive<u8> = u8::MIN..=u8::MAX;

/// Software often uses an unsigned 8-bit number to represent a component value.
///
/// In the case of RGB: <https://en.wikipedia.org/wiki/RGB_color_model#Numeric_representations>
///
/// As stated on <https://www.w3.org/TR/css-color-4/#rgb-functions>:
/// "0 again represents the minimum value for the color channel, but 255 represents the maximum.
/// These values come from the fact that many graphics engines store the color channels internally as a single byte,
/// which can hold integers between 0 and 255."
///
/// Due to most color models only using this as a secondary representation, method naming is named to avoid confusion.
pub trait SingleByteComponent {
    /// Creates a new channel based on the given value in the range 0 to 255.
    fn from_u8(single_byte_value: u8) -> Self;

    /// Returns the closest value from 0 to 255 based on the channel value. Note that precision may be lost,
    /// e.g. if the full value is a float.
    /// To check if precision will be lost on conversion, use [`fits_in_u8`](#method.fits_in_u8).
    fn to_u8(&self) -> u8;

    /// Checks if this channels value can be fully represented in a range from 0 to 255.
    /// Due to the lack of precision in this range, not all values can be.
    fn fits_in_u8(&self) -> bool;
}


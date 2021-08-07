//! Traits for color channel components.

use std::ops::RangeInclusive;

use rug::Float;

use crate::error::RangeError;

pub(crate) const FLOAT_COMPONENT_VALUE_RANGE: RangeInclusive<f64> = 0.0..=1.0;

/// Component that uses percentages or floats between 0 and 1 for its value.
///
/// In the case of RGB: <https://en.wikipedia.org/wiki/RGB_color_model#Numeric_representations>.
pub trait FloatComponent: From<Float> {
    /// Creates a new channel with the given value. Value must be >= 0 and <= 1.
    ///
    /// # Panics
    /// If value is out of range.
    fn from_value(value: Float) -> Self;

    /// Returns the channel value as-is.
    fn value(&self) -> &Float;
}

pub(crate) const SINGLE_BYTE_COMPONENT_VALUE_RANGE: RangeInclusive<u8> = u8::MIN..=u8::MAX;

/// Component with value able to be presented in a single byte.
/// This is often used in software for historical purposes, or if the limited precision is good enough.
///
/// In the case of RGB: <https://en.wikipedia.org/wiki/RGB_color_model#Numeric_representations>.
/// As stated on <https://www.w3.org/TR/css-color-4/#rgb-functions>:
/// "0 again represents the minimum value for the color channel, but 255 represents the maximum.
/// These values come from the fact that many graphics engines store the color channels internally as a single byte,
/// which can hold integers between 0 and 255."
///
/// Meant to be used as secondary representation, for example for a color channel that is based on [`FloatComponent`].
pub trait SingleByteComponent: From<u8> {

    /// Creates a new channel based on the given value in the range 0 to 255.
    fn from_u8(single_byte_value: u8) -> Self;

    /// Checks if this channels value can be fully represented in a range from 0 to 255.
    /// Due to the lack of precision in this range, not all values can be.
    fn fits_in_u8(&self) -> bool;

    /// Returns the closest value from 0 to 255 based on the channel value.
    /// To check if conversion will fail, use [`fits_in_u8`](#method.fits_in_u8).
    fn to_u8(&self) -> Result<u8, RangeError>;

    /// Returns the closest value from 0 to 255 based on the channel value. Note that precision may be lost,
    /// if the value does not exactly fit into 8 bit.
    /// To check if precision will be lost on conversion, use [`fits_in_u8`](#method.fits_in_u8).
    fn to_u8_round(&self) -> u8;
}

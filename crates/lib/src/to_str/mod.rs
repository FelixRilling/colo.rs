pub use crate::to_str::hsl_function::to_hsl_function_str;
pub use crate::to_str::rgb_function::to_rgb_function_str;
pub use crate::to_str::rgb_hex::{LetterCase, ShorthandNotation, to_rgb_hex_str};

mod css_types;
mod rgb_hex;
mod rgb_function;
mod hsl_function;

/// If the alpha channel may be omitted if it is opaque.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OmitAlphaChannel {
	Never,
	IfOpaque,
}

/// Possible CSS types able to represent an RGB component value.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ChannelUnit {
	Number,
	Percentage,
}

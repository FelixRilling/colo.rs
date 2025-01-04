pub use crate::to_str::hsl_function::to_hsl_function_str;
pub use crate::to_str::hwb_function::to_hwb_function_str;
pub use crate::to_str::rgb_function::to_rgb_function_str;
pub use crate::to_str::rgb_hex::{to_rgb_hex_str, LetterCase, ShorthandNotation};

mod common;
mod css_types;
mod hsl_function;
mod hwb_function;
mod rgb_function;
mod rgb_hex;

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

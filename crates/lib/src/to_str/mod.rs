pub use crate::to_str::rgb_function::{ChannelUnit, to_rgb_function_str};
pub use crate::to_str::rgb_hex::{LetterCase, ShorthandNotation, to_rgb_hex_str};

mod rgb_hex;
mod rgb_function;

/// If the alpha channel may be omitted if it is opaque.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum OmitAlphaChannel {
	Never,
	IfOpaque,
}

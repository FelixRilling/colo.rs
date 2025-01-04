use palette::Srgba;

use crate::to_str::css_types::format_alpha_value;
use crate::to_str::{ChannelUnit, OmitAlphaChannel};
use crate::util::is_opaque;

// TODO: allow any `Alpha` type
/// Wrapper to handle common handling of alpha omission
pub(crate) fn format_alpha_value_conditionally(
	color: &Srgba,
	alpha_channel_unit: ChannelUnit,
	omit_alpha_channel: OmitAlphaChannel,
) -> Option<String> {
	if omit_alpha_channel == OmitAlphaChannel::IfOpaque && is_opaque(color) {
		None
	} else {
		Some(format_alpha_value(color.alpha, alpha_channel_unit))
	}
}

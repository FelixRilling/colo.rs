use palette::Srgba;

use crate::to_str::{ChannelUnit, OmitAlphaChannel};
use crate::to_str::css_types::format_alpha_value;
use crate::util::is_opaque;

pub(crate) fn format_alpha_value_conditionally(
	color: &Srgba,
	alpha_channel_unit: ChannelUnit,
	omit_alpha_channel: OmitAlphaChannel,
) -> Option<String> {
	if is_opaque(color) && omit_alpha_channel == OmitAlphaChannel::IfOpaque {
		None
	} else {
		Some(format_alpha_value(color.alpha, alpha_channel_unit))
	}
}

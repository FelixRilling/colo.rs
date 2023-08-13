use cssparser::{BasicParseError, BasicParseErrorKind, Color, Parser, ParserInput};
use palette::rgb::Rgba;

use crate::error::ParsingError;

impl From<BasicParseError<'_>> for ParsingError<'_> {
	fn from(err: BasicParseError) -> Self {
		ParsingError::InvalidSyntax(match err.kind {
			BasicParseErrorKind::UnexpectedToken(_) => "Unexpected token",
			_ => "Unknown error",
		})
	}
}

/// Parses CSS color string.
///
/// # Errors
/// - If color is keyword 'currentcolor'.
/// - All other errors: See `cssparser` `Color::parse`.
pub fn parse_color(seq: &str) -> Result<Rgba, ParsingError> {
	let mut input = ParserInput::new(seq);
	let color = Color::parse(&mut Parser::new(&mut input))?;
	match color {
		Color::CurrentColor => Err(ParsingError::UnsupportedValue(
			"currentcolor is not supported in this context",
		)),
		Color::RGBA(rgba) => Ok(Rgba::new(
			rgba.red_f32(),
			rgba.green_f32(),
			rgba.blue_f32(),
			rgba.alpha_f32(),
		)),
	}
}

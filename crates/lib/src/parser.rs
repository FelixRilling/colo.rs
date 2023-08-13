use cssparser::{BasicParseError, BasicParseErrorKind, Color, Parser, ParserInput};
use palette::rgb::{Rgb, Rgba};
use palette::WithAlpha;

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
	let mut parser = Parser::new(&mut input);
	let color = Color::parse(&mut parser)
		.map_err(|_| ParsingError::InvalidSyntax("invalid syntax"))?;

	match color {
		Color::CurrentColor => Err(ParsingError::UnsupportedValue(
			"currentcolor is not supported in this context",
		)),
		Color::Rgba(rgba) => Ok(Rgb::new(rgba.red, rgba.green, rgba.blue)
			.with_alpha(rgba.alpha)
			.into_format()),
		_ => Err(ParsingError::UnsupportedValue("format is not supported")),
	}
}

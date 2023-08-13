use cssparser::{BasicParseError, BasicParseErrorKind, Color, Parser, ParserInput};
use palette::{Hsl, Hwb, IntoColor, Lab, Lch, WithAlpha};
use palette::rgb::{Rgb, Rgba};

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
	let color =
		Color::parse(&mut parser).map_err(|_| ParsingError::InvalidSyntax("invalid syntax"))?;

	match color {
		Color::CurrentColor => Err(ParsingError::UnsupportedValue(
			"currentcolor is not supported in this context",
		)),
		Color::Rgba(rgba) => Ok(Rgb::new(rgba.red, rgba.green, rgba.blue)
			.with_alpha(rgba.alpha)
			.into_format()),

		Color::Hsl(hsl) => Ok(Hsl::new(
			hsl.hue.unwrap_or(0.0),
			hsl.saturation.unwrap_or(0.0),
			hsl.lightness.unwrap_or(0.0),
		)
			.with_alpha(hsl.alpha.unwrap_or(1.0))
			.into_color()),
		Color::Hwb(hwb) => Ok(Hwb::new(
			hwb.hue.unwrap_or(0.0),
			hwb.whiteness.unwrap_or(0.0),
			hwb.blackness.unwrap_or(0.0),
		).with_alpha(hwb.alpha.unwrap_or(1.0))
			.into_format()
			.into_color()),
		Color::Lab(lab) => Ok(Lab::new(
			lab.lightness.unwrap_or(0.0),
			lab.a.unwrap_or(0.0),
			lab.b.unwrap_or(0.0),
		)
			.with_alpha(lab.alpha.unwrap_or(1.0))
			.into_color()),
		Color::Lch(lch) => Ok(todo!()),
		Color::Oklab(_) => { todo!() }
		Color::Oklch(_) => { todo!() }
		Color::ColorFunction(_) => Err(ParsingError::UnsupportedValue("format is not supported")),
	}
}

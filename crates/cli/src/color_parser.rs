use anyhow::{anyhow, Error, Result};
use cssparser::{ParseError, ParseErrorKind, Parser, ParserInput};
use cssparser_color::Color;
use palette::rgb::{Rgb, Rgba};
use palette::{Hsl, Hwb, IntoColor, Lab, Lch, Oklab, Oklch, WithAlpha};

fn map_parse_error<'i>(err: ParseError<'i, ()>) -> Error {
	anyhow!(
		"{} at L{}:{}.",
		match err.kind {
			ParseErrorKind::Basic(kind) => kind.to_string(),
			ParseErrorKind::Custom(_) => "Unknown error".to_string(),
		},
		err.location.line,
		err.location.column
	)
}

/// Parses CSS color string.
///
/// # Errors
/// - If color is keyword 'currentcolor'.
/// - All other errors: See `cssparser` `Color::parse`.
pub fn parse_color(seq: &str) -> Result<Rgba> {
	let mut input = ParserInput::new(seq);
	let color = Color::parse(&mut Parser::new(&mut input)).map_err(map_parse_error)?;

	match color {
		Color::ColorFunction(_) => Err(anyhow!("Format is not supported.")),

		Color::CurrentColor => Err(anyhow!("currentcolor is not supported in this context.",)),

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
		)
		.with_alpha(hwb.alpha.unwrap_or(1.0))
		.into_color()),

		Color::Lab(lab) => Ok(Lab::new(
			lab.lightness.unwrap_or(0.0),
			lab.a.unwrap_or(0.0),
			lab.b.unwrap_or(0.0),
		)
		.with_alpha(lab.alpha.unwrap_or(1.0))
		.into_color()),

		Color::Lch(lch) => Ok(Lch::new(
			lch.lightness.unwrap_or(0.0),
			lch.chroma.unwrap_or(0.0),
			lch.hue.unwrap_or(0.0),
		)
		.with_alpha(lch.alpha.unwrap_or(1.0))
		.into_color()),

		Color::Oklab(oklab) => Ok(Oklab::new(
			oklab.lightness.unwrap_or(0.0),
			oklab.a.unwrap_or(0.0),
			oklab.b.unwrap_or(0.0),
		)
		.with_alpha(oklab.alpha.unwrap_or(1.0))
		.into_color()),

		Color::Oklch(oklch) => Ok(Oklch::new(
			oklch.lightness.unwrap_or(0.0),
			oklch.chroma.unwrap_or(0.0),
			oklch.hue.unwrap_or(0.0),
		)
		.with_alpha(oklch.alpha.unwrap_or(1.0))
		.into_color()),
	}
}

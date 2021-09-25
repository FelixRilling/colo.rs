use cssparser::{BasicParseError, BasicParseErrorKind, Color, Parser, ParserInput, RGBA, ToCss};
use palette::Srgba;

use crate::error::ParsingError;

impl From<BasicParseError<'_>> for ParsingError<'_> {
    fn from(err: BasicParseError) -> Self {
        ParsingError::InvalidSyntax(match err.kind {
            BasicParseErrorKind::UnexpectedToken(_) => "Unexpected token",
            _ => "Unknown error",
        })
    }
}

/// Parses CSS color string,
// Wraps cssparser and converts to common color struct.
pub fn parse_color(seq: &str) -> Result<Srgba, ParsingError> {
    let mut input = ParserInput::new(seq);
    let color = Color::parse(&mut Parser::new(&mut input))?;
    match color {
        Color::CurrentColor => Err(ParsingError::InvalidSyntax(
            "'currentcolor' is not supported in this context.",
        )),
        Color::RGBA(rgba) => Ok(Srgba::from_components((
            rgba.red_f32(),
            rgba.green_f32(),
            rgba.blue_f32(),
            rgba.alpha_f32(),
        ))),
    }
}


/// Creates CSS string representation for color.
pub fn color_as_string(color: &Srgba) -> String {
    let rgba = RGBA::from_floats(color.red, color.green, color.blue, color.alpha);
    rgba.to_css_string()
}

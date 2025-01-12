use std::io::Write;

use palette::rgb::Rgba;
use termcolor::{ColorChoice, StandardStream};

use color_utils::util::is_opaque;

use crate::color_printing::print_color;
use crate::options::{ColorFormat, Options};

pub fn print_details(color: &Rgba, options: &Options) -> std::io::Result<()> {
	let mut out = StandardStream::stdout(ColorChoice::Auto);

	write!(&mut out, "Details for color ")?;
	print_color(&mut out, color, options.format)?;
	writeln!(&mut out, ":")?;
	writeln!(&mut out, "-------")?;

	print_general_details(&mut out, color)?;

	print_format_details(&mut out, color)
}

fn print_general_details(out: &mut StandardStream, color: &Rgba) -> std::io::Result<()> {
	writeln!(out, "General: ")?;
	writeln!(out, "\tIs opaque: {}.", is_opaque(color))
	// TODO: output if color fits in 8 bit channel
}

fn print_format_details(out: &mut StandardStream, color: &Rgba) -> std::io::Result<()> {
	writeln!(out, "Formats: ")?;

	write!(out, "\tIn RGB hexadecimal notation: ")?;
	print_color(out, color, ColorFormat::RgbHex)?;
	// TODO: output if precision is lost in this form
	writeln!(out, ".")?;

	write!(out, "\tIn RGB function notation: ")?;
	print_color(out, color, ColorFormat::RgbFunction)?;
	writeln!(out, ".")?;

	write!(out, "\tIn HSL function notation: ")?;
	print_color(out, color, ColorFormat::HslFunction)?;
	writeln!(out, ".")?;

	write!(out, "\tIn HWB function notation: ")?;
	print_color(out, color, ColorFormat::HwbFunction)?;
	writeln!(out, ".")
}

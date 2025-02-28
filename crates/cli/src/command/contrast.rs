use core::fmt;
use std::collections::HashSet;
use std::fmt::Display;
use std::io::Write;

use palette::color_difference::Wcag21RelativeContrast;
use palette::rgb::{Rgb, Rgba};
use termcolor::{ColorChoice, StandardStream};

use crate::color_printing::print_color;
use crate::options::Options;

/// Contrast target values based on
/// <https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast>.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ContrastLevel {
	/// Enhanced contrast for text.
	Aaa,

	/// Enhanced contrast for large text.
	LargeAaa,

	/// Minimum contrast for text.
	Aa,

	/// Minimum contrast for large text.
	LargeAa,
}

impl Display for ContrastLevel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match &self {
			ContrastLevel::Aaa => "AAA",
			ContrastLevel::LargeAaa => "AAA (Large Text)",
			ContrastLevel::Aa => "AA",
			ContrastLevel::LargeAa => "AA (Large Text)",
		})
	}
}

fn contrast_ratio_levels_reached(color_1: &Rgb, color_2: &Rgb) -> HashSet<ContrastLevel> {
	let mut reached = HashSet::with_capacity(4);
	if color_1.has_min_contrast_large_text(*color_2) {
		reached.insert(ContrastLevel::LargeAa);
		if color_1.has_min_contrast_text(*color_2) {
			reached.insert(ContrastLevel::Aa);
			reached.insert(ContrastLevel::LargeAaa);
			if color_1.has_enhanced_contrast_text(*color_2) {
				reached.insert(ContrastLevel::Aaa);
			}
		}
	}
	reached
}

fn hash_set_as_sorted_vec<T: Ord>(hash_set: HashSet<T>) -> Vec<T> {
	let mut set_copy_vec = hash_set.into_iter().collect::<Vec<_>>();
	set_copy_vec.sort();
	set_copy_vec
}

pub fn print_contrast(color_1: &Rgba, color_2: &Rgba, options: &Options) -> std::io::Result<()> {
	let mut out = StandardStream::stdout(ColorChoice::Auto);

	print_contrast_ratio(&mut out, color_1, color_2, options)?;

	print_contrast_levels_reached(&mut out, color_1, color_2)
}

fn print_contrast_ratio(
	out: &mut StandardStream,
	color_1: &Rgba,
	color_2: &Rgba,
	options: &Options,
) -> std::io::Result<()> {
	write!(out, "WCAG 2.0 AA/AAA contrast ratio for ")?;
	print_color(out, color_1, options.format)?;
	write!(out, " to ")?;
	print_color(out, color_2, options.format)?;

	let contrast_ratio = color_1.relative_contrast(**color_2);
	writeln!(out, " is {:.2}.", contrast_ratio)
}

fn print_contrast_levels_reached(
	out: &mut StandardStream,
	color_1: &Rgba,
	color_2: &Rgba,
) -> std::io::Result<()> {
	let contrast_levels_reached = contrast_ratio_levels_reached(color_1, color_2);
	let contrast_levels_reached_str: String = if contrast_levels_reached.is_empty() {
		String::from("None")
	} else {
		hash_set_as_sorted_vec(contrast_levels_reached)
			.iter()
			.map(std::string::ToString::to_string)
			.collect::<Vec<String>>()
			.join(", ")
	};
	writeln!(
		out,
		"Contrast level(s) reached: {}.",
		contrast_levels_reached_str
	)
}

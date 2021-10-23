use core::fmt;
use std::collections::HashSet;
use std::fmt::Display;
use std::io::Write;

use palette::{RelativeContrast, Srgba};
use termcolor::{ColorChoice, StandardStream};

use color_utils_internal::floor_n_decimals;

use crate::color_printing::print_color;
use crate::options::Options;

/// Contrast target values based on
/// <https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast>.
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ContrastLevel {
	/// Enhanced contrast for text.
	AAA,

	/// Enhanced contrast for large text.
	LargeAAA,

	/// Minimum contrast for text.
	AA,

	/// Minimum contrast for large text.
	LargeAA,
}

impl Display for ContrastLevel {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(match &self {
			ContrastLevel::AAA => "AAA",
			ContrastLevel::LargeAAA => "AAA (Large Text)",
			ContrastLevel::AA => "AA",
			ContrastLevel::LargeAA => "AA (Large Text)",
		})
	}
}

fn contrast_ratio_levels_reached(color_1: &Srgba, color_2: &Srgba) -> HashSet<ContrastLevel> {
	let mut reached = HashSet::with_capacity(4);
	if color_1.has_min_contrast_large_text(color_2) {
		reached.insert(ContrastLevel::LargeAA);
		if color_1.has_min_contrast_text(color_2) {
			reached.insert(ContrastLevel::AA);
			reached.insert(ContrastLevel::LargeAAA);
			if color_1.has_enhanced_contrast_text(color_2) {
				reached.insert(ContrastLevel::AAA);
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

pub fn print_contrast(color_1: &Srgba, color_2: &Srgba, options: &Options) -> std::io::Result<()> {
	let mut out = StandardStream::stdout(ColorChoice::Auto);

	print_contrast_ratio(&mut out, color_1, color_2, options)?;

	print_contrast_levels_reached(&mut out, color_1, color_2)
}

fn print_contrast_ratio(
	out: &mut StandardStream,
	color_1: &Srgba,
	color_2: &Srgba,
	options: &Options,
) -> std::io::Result<()> {
	write!(out, "WCAG 2.0 AA/AAA contrast ratio for ")?;
	print_color(out, color_1, &options.format)?;
	write!(out, " to ")?;
	print_color(out, color_2, &options.format)?;

	let contrast_ratio = color_1.get_contrast_ratio(color_2);
	let contrast_ratio_str = if options.verbosity == 0 {
		// Usually only displaying the last 2 digits is enough.
		floor_n_decimals(contrast_ratio.into(), 2).to_string()
	} else {
		contrast_ratio.to_string()
	};
	writeln!(out, " is {}.", contrast_ratio_str)
}

fn print_contrast_levels_reached(
	out: &mut StandardStream,
	color_1: &Srgba,
	color_2: &Srgba,
) -> std::io::Result<()> {
	let contrast_levels_reached =
		contrast_ratio_levels_reached(&color_1.to_owned(), &color_2.to_owned());
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

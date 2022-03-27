extern crate palette;

use std::convert::TryInto;

use clap::{Arg, Command};
use log::LevelFilter;

use color_format::ColorFormat;
use options::Options;

mod color_format;
mod color_parsing;
mod color_printing;
mod command;
mod options;

const COLOR_ARG_HELP: &str =
	"CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'. Parsing can be \
	customized via the `format` arg.";

fn main() {
	let matches = Command::new("color-utils")
		.arg(
			Arg::new("v")
				.short('v')
				.multiple_occurrences(true)
				.takes_value(false)
				.help("Increases message verbosity."),
		)
		.arg(
			Arg::new("format")
				.long("format")
				.takes_value(true)
				.value_name("format-name")
				.possible_values(&[
					ColorFormat::Auto.to_string().as_str(),
					ColorFormat::RgbHex.to_string().as_str(),
					ColorFormat::RgbFunction.to_string().as_str(),
					ColorFormat::HslFunction.to_string().as_str(),
					ColorFormat::HwbFunction.to_string().as_str(),
				])
				.required(false)
				.default_value(ColorFormat::Auto.to_string().as_str())
				.help("Which color format to use for parsing and output"),
		)
		.subcommand(
			Command::new("details")
				.about("Prints details for a color.")
				.arg(
					Arg::new("color")
						.required(true)
						.takes_value(true)
						.help(COLOR_ARG_HELP),
				),
		)
		.subcommand(
			Command::new("contrast")
				.about("Calculates WCAG contrast of two colors.")
				.arg(
					Arg::new("color")
						.required(true)
						.takes_value(true)
						.help(COLOR_ARG_HELP),
				)
				.arg(
					Arg::new("other-color")
						.required(true)
						.takes_value(true)
						.help(COLOR_ARG_HELP),
				),
		)
		.get_matches();

	let verbosity = matches
		.occurrences_of("v")
		.try_into()
		.expect("Unexpected count of verbosity flags");
	env_logger::builder()
		.filter_level(match &verbosity {
			0 => LevelFilter::Error,
			1 => LevelFilter::Warn,
			2 => LevelFilter::Info,
			3 => LevelFilter::Debug,
			_ => LevelFilter::Trace,
		})
		.init();

	// Unwrapping should be safe as 'possible_values' only allows parseable values,
	// and we either have a value or use a default.
	let format = matches.value_of_t("format").unwrap();

	let options = Options { verbosity, format };

	match matches.subcommand() {
		Some(("details", matches)) => {
			let color_str = matches.value_of("color").unwrap();

			match color_parsing::parse_color(color_str) {
				Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
				Ok(color) => {
					command::print_details(&color, &options).expect("Could not print details.")
				}
			}
		}
		Some(("contrast", matches)) => {
			let color_str = matches.value_of("color").unwrap();
			let other_color_str = matches.value_of("other-color").unwrap();

			match color_parsing::parse_color(color_str) {
				Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
				Ok(color) => match color_parsing::parse_color(other_color_str) {
					Err(e_2) => eprintln!("Could not parse other color: {}.", e_2),
					Ok(other_color) => command::print_contrast(&color, &other_color, &options)
						.expect("Could not print contrast."),
				},
			}
		}
		_ => eprintln!("No subcommand provided. See --help."),
	}
}

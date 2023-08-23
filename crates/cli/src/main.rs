extern crate palette;

use clap::{Arg, ArgAction, Command, ValueEnum};
use clap::builder::{EnumValueParser, PossibleValue};
use log::LevelFilter;

use color_format::ColorFormat;
use options::Options;

mod color_format;
mod color_parsing;
mod color_printing;
mod command;
mod options;

const COLOR_ARG_HELP: &str =
	"CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'.";

const COLOR_FORMAT_AUTO: &str = "auto";

impl ValueEnum for ColorFormat {
	fn value_variants<'a>() -> &'a [Self] {
		&[
			ColorFormat::Auto,
			ColorFormat::RgbHex,
			ColorFormat::RgbFunction,
			ColorFormat::HslFunction,
			ColorFormat::HwbFunction,
		]
	}

	fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
		match self {
			ColorFormat::Auto => Some(PossibleValue::new(COLOR_FORMAT_AUTO)),
			ColorFormat::RgbHex => Some(PossibleValue::new("rgb-hex")),
			ColorFormat::RgbFunction => Some(PossibleValue::new("rgb-function")),
			ColorFormat::HslFunction => Some(PossibleValue::new("hsl-function")),
			ColorFormat::HwbFunction => Some(PossibleValue::new("hwb-function")),
		}
	}
}

fn main() {
	let matches = Command::new("color-utils")
		.arg(
			Arg::new("v")
				.short('v')
				.action(ArgAction::Count)
				.help("Increases message verbosity"),
		)
		.arg(
			Arg::new("format")
				.long("format")
				.value_name("format-name")
				.required(false)
				.num_args(1)
				.action(ArgAction::Set)
				.value_parser(EnumValueParser::<ColorFormat>::new())
				.default_value(COLOR_FORMAT_AUTO)
				.help("Which color format to use for output"),
		)
		.subcommand(
			Command::new("details")
				.about("Prints details for a color")
				.arg(
					Arg::new("color")
						.required(true)
						.num_args(1)
						.action(ArgAction::Set)
						.help(COLOR_ARG_HELP),
				),
		)
		.subcommand(
			Command::new("contrast")
				.about("Calculates WCAG contrast of two colors")
				.arg(
					Arg::new("color")
						.required(true)
						.num_args(1)
						.action(ArgAction::Set)
						.help(COLOR_ARG_HELP),
				)
				.arg(
					Arg::new("other-color")
						.required(true)
						.num_args(1)
						.action(ArgAction::Set)
						.help(COLOR_ARG_HELP),
				),
		)
		.get_matches();

	let verbosity = matches.get_count("v");
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
	let format: ColorFormat = *matches.get_one::<ColorFormat>("format").unwrap();

	let options = Options { verbosity, format };

	match matches.subcommand() {
		Some(("details", matches)) => {
			let color_str = matches.get_one::<String>("color").unwrap();

			match color_parsing::parse_color(color_str) {
				Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
				Ok(color) => {
					command::print_details(&color, &options).expect("Could not print details.")
				}
			}
		}
		Some(("contrast", matches)) => {
			let color_str = matches.get_one::<String>("color").unwrap();
			let other_color_str = matches.get_one::<String>("other-color").unwrap();

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

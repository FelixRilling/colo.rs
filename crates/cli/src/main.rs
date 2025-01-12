use clap::{Parser, Subcommand};
use log::LevelFilter;
use options::{ColorFormat, Options};

mod color_parsing;
mod color_printing;
mod command;
mod options;

const COLOR_ARG_HELP: &str = "CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'";

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
	#[arg(
		long,
		required = false,
		default_value = "auto",
		value_enum,
		help = "Which color format to use for output"
	)]
	format: ColorFormat,

	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	#[command(about = "Prints the details of a color")]
	Details {
		#[arg(required = true, help = COLOR_ARG_HELP)]
		color: String,
	},

	#[command(about = "Calculates the WCAG contrast of two colors")]
	Contrast {
		#[arg(required = true, help = COLOR_ARG_HELP)]
		color: String,

		#[arg(required = true, help = COLOR_ARG_HELP)]
		other_color: String,
	},
}

fn main() {
	env_logger::builder().filter_level(LevelFilter::Info).init();

	let args = Cli::parse();

	let options = Options {
		format: args.format,
	};

	match args.command {
		Commands::Details { color: color_str } => match color_parsing::parse_color(&color_str) {
			Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
			Ok(color) => {
				command::print_details(&color, &options).expect("Could not print details.")
			}
		},
		Commands::Contrast {
			color: color_str,
			other_color: other_color_str,
		} => match color_parsing::parse_color(&color_str) {
			Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
			Ok(color) => match color_parsing::parse_color(&other_color_str) {
				Err(e_2) => eprintln!("Could not parse other color: {}.", e_2),
				Ok(other_color) => command::print_contrast(&color, &other_color, &options)
					.expect("Could not print contrast."),
			},
		},
	}
}

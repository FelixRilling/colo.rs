use clap::{Parser, Subcommand};
use color_parser::parse_color;
use log::LevelFilter;
use options::{ColorFormat, Options};
use palette::Srgba;

mod color_parser;
mod color_printing;
mod command;
mod options;

const COLOR_ARG_HELP: &str = "CSS-like color value, e.g. '#00FF11' or 'rgb(255 128 0)'";

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
		#[arg(required = true, help = COLOR_ARG_HELP, value_parser = parse_color)]
		color: Srgba,
	},

	#[command(about = "Calculates the WCAG contrast of two colors")]
	Contrast {
		#[arg(required = true, help = COLOR_ARG_HELP, value_parser = parse_color)]
		color: Srgba,

		#[arg(required = true, help = COLOR_ARG_HELP, value_parser = parse_color)]
		other_color: Srgba,
	},
}

fn main() -> Result<(), std::io::Error> {
	env_logger::builder().filter_level(LevelFilter::Info).init();

	let args = Cli::parse();

	let options = Options {
		format: args.format,
	};

	match args.command {
		Commands::Details { color } => command::print_details(&color, &options),
		Commands::Contrast { color, other_color } => {
			command::print_contrast(&color, &other_color, &options)
		}
	}
}

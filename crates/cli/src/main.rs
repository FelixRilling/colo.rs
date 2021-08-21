use std::convert::TryInto;

use clap::{App, Arg, SubCommand};
use clap::value_t;
use log::LevelFilter;

use color_format::ColorFormat;
use options::Options;

mod color_format;
mod color_parsing;
mod color_printing;
mod command;
mod options;

const FORMAT_ARG_KEY: &str = "format";
const INPUT_FORMAT_ARG_KEY: &str = "input-format";
const OUTPUT_FORMAT_ARG_KEY: &str = "output-format";

fn main() {
    let default_format = ColorFormat::Auto.to_string();

    let color_arg_help = format!("CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'. Parsing can be customized via `{}` arg.", INPUT_FORMAT_ARG_KEY);

    let matches = App::new("color-utils")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .takes_value(false)
                .help("Increases message verbosity."),
        )
        .arg(
            Arg::with_name(FORMAT_ARG_KEY)
                .long(FORMAT_ARG_KEY)
                .takes_value(true)
                .value_name("format-name")
                .possible_values(&[
                    ColorFormat::Auto.to_string().as_str(),
                    ColorFormat::RgbHex.to_string().as_str(),
                    ColorFormat::RgbFunction.to_string().as_str(),
                ])
                .case_insensitive(true)
                .required(false)
                .default_value(default_format.as_str())
                .help(&format!(
                    "Shorthand to set both `{}` and `{}`.",
                    INPUT_FORMAT_ARG_KEY, OUTPUT_FORMAT_ARG_KEY
                )),
        )
        .arg(
            Arg::with_name(INPUT_FORMAT_ARG_KEY)
                .long(INPUT_FORMAT_ARG_KEY)
                .takes_value(true)
                .value_name("format-name")
                .possible_values(&[
                    ColorFormat::Auto.to_string().as_str(),
                    ColorFormat::RgbHex.to_string().as_str(),
                    ColorFormat::RgbFunction.to_string().as_str(),
                ])
                .case_insensitive(true)
                .required(false)
                .help("Which color format to use when parsing input colors."),
        )
        .arg(
            Arg::with_name(OUTPUT_FORMAT_ARG_KEY)
                .long(OUTPUT_FORMAT_ARG_KEY)
                .takes_value(true)
                .value_name("format-name")
                .possible_values(&[
                    ColorFormat::Auto.to_string().as_str(),
                    ColorFormat::RgbHex.to_string().as_str(),
                    ColorFormat::RgbFunction.to_string().as_str(),
                ])
                .case_insensitive(true)
                .required(false)
                .help("Which color format to use when outputting colors."),
        )
        .subcommand(
            SubCommand::with_name("details")
                .about("Prints details for a color.")
                .arg(
                    Arg::with_name("color")
                        .required(true)
                        .takes_value(true)
                        .help(&color_arg_help),
                ),
        )
        .subcommand(
            SubCommand::with_name("contrast")
                .about("Calculates WCAG contrast of two colors.")
                .arg(
                    Arg::with_name("color")
                        .required(true)
                        .takes_value(true)
                        .help(&color_arg_help),
                )
                .arg(
                    Arg::with_name("other-color")
                        .required(true)
                        .takes_value(true)
                        .help(&color_arg_help),
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

    // Unwrapping should be safe as 'possible_values' only allows parseable values
    // and we either have a value or use a default.
    let general_format = value_t!(matches, FORMAT_ARG_KEY, ColorFormat).unwrap();

    let input_format =
        value_t!(matches, INPUT_FORMAT_ARG_KEY, ColorFormat).unwrap_or(general_format);
    let output_format =
        value_t!(matches, OUTPUT_FORMAT_ARG_KEY, ColorFormat).unwrap_or(general_format);

    let options = Options {
        verbosity,
        input_format,
        output_format,
    };

    match matches.subcommand() {
        ("details", Some(matches)) => {
            let color_str = matches.value_of("color").unwrap();

            match color_parsing::parse_color(color_str, &options.input_format) {
                Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
                Ok(color) => {
                    command::print_details(&color, &options).expect("Could not print details.")
                }
            }
        }
        ("contrast", Some(matches)) => {
            let color_str = matches.value_of("color").unwrap();
            let other_color_str = matches.value_of("other-color").unwrap();

            match color_parsing::parse_color(color_str, &options.input_format) {
                Err(e_1) => eprintln!("Could not parse color: {}.", e_1),
                Ok(color) => {
                    match color_parsing::parse_color(other_color_str, &options.input_format) {
                        Err(e_2) => eprintln!("Could not parse other color: {}.", e_2),
                        Ok(other_color) => command::print_contrast(&color, &other_color, &options)
                            .expect("Could not print contrast."),
                    }
                }
            }
        }
        _ => eprintln!("No subcommand provided. See --help."),
    }
}

use clap::{App, Arg, SubCommand};
use log::{debug, error, info, LevelFilter};

use color_utils::color::rgb::Rgb;
use color_utils::error::ParsingError;

mod color_printing;
mod contrast;

const COLOR_ARG_HELP: &str = "CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'.";

fn parse_color(seq: &str) -> Result<Rgb, ParsingError> {
    // TODO: Allow specifying which format should be used instead of trying all.
    debug!("Attempting to parse '{}' as hex string color.", seq);
    match Rgb::from_hex_str(seq) {
        Ok(color) => {
            info!("Successfully parsed '{}' as hex string color: '{}'.", seq, &color);
            Ok(color)
        }
        Err(hex_err) => {
            info!("Could not parse '{}' as hex string color: {}.", seq, &hex_err);

            debug!("Attempting to parse '{}' as RGB function string color.", seq);
            match Rgb::from_rgb_function_str(seq) {
                Ok(color) => {
                    info!("Successfully parsed '{}' as RGB function string color: '{}'.", seq, &color);
                    Ok(color)
                }
                Err(rgb_function_err) => {
                    info!("Could not parse '{}' as RGB function string color: {}.", seq, &rgb_function_err);
                    Err(rgb_function_err)
                }
            }
        }
    }
}

fn main() {
    let matches = App::new("Colo.rs")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity.")
        )
        .subcommand(
            SubCommand::with_name("contrast")
                .about("Calculate WCAG contrast of two colors.")
                .arg(
                    Arg::with_name("color_1")
                        .required(true)
                        .help(COLOR_ARG_HELP)
                )
                .arg(
                    Arg::with_name("color_2")
                        .required(true)
                        .help(COLOR_ARG_HELP)
                )
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v") as usize;
    env_logger::builder().filter_level(match &verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        3 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    }).init();

    match matches.subcommand_matches("contrast") {
        Some(matches) => {
            let color_1_str = matches.value_of("color_1").unwrap();
            let color_2_str = matches.value_of("color_2").unwrap();

            match parse_color(color_1_str) {
                Err(_) => error!("Could not parse color 1."),
                Ok(color_1) => match parse_color(color_2_str) {
                    Err(_) => error!("Could not parse color 2."),
                    Ok(color_2) => contrast::print_contrast(&color_1, &color_2, verbosity).expect("Could not print contrast.")
                },
            }
        }
        None => error!("No subcommand provided. See --help.")
    }
}

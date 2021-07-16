use clap::{App, Arg, SubCommand};
use log::{debug, info, LevelFilter};

use color_utils::error::ParsingError;
use color_utils::rgb::Rgb;

mod color_printing;
mod contrast;

// TODO: Allow specifying which format should be used instead of trying all.

pub fn is_color(val: String) -> Result<(), String> {
    if Rgb::from_hex_str(&val).is_ok() || Rgb::from_rgb_function_str(&val).is_ok() {
        return Ok(());
    }
    Err(String::from(
        "Could not parse the value as a CSS-like color value.",
    ))
}

pub fn parse_color(seq: &str) -> Result<Rgb, ParsingError> {
    debug!("Attempting to parse '{}' as hex string color.", seq);
    match Rgb::from_hex_str(seq) {
        Ok(color) => {
            info!(
                "Successfully parsed '{}' as hex string color: '{}'.",
                seq, &color
            );
            Ok(color)
        }
        Err(hex_err) => {
            info!(
                "Could not parse '{}' as hex string color: {}.",
                seq, &hex_err
            );

            debug!(
                "Attempting to parse '{}' as RGB function string color.",
                seq
            );
            match Rgb::from_rgb_function_str(seq) {
                Ok(color) => {
                    info!(
                        "Successfully parsed '{}' as RGB function string color: '{}'.",
                        seq, &color
                    );
                    Ok(color)
                }
                Err(rgb_function_err) => {
                    info!(
                        "Could not parse '{}' as RGB function string color: {}.",
                        seq, &rgb_function_err
                    );
                    Err(rgb_function_err)
                }
            }
        }
    }
}

fn decorate_color_arg<'a>(arg: Arg<'a, 'a>) -> Arg<'a, 'a> {
    arg.required(true)
        .help("CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'.")
        .validator(is_color)
}

fn main() {
    let matches = App::new("Colo.rs")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Increase message verbosity."),
        )
        .subcommand(
            SubCommand::with_name("contrast")
                .about("Calculate WCAG contrast of two colors.")
                .arg(decorate_color_arg(Arg::with_name("color_1")))
                .arg(decorate_color_arg(Arg::with_name("color_2"))),
        )
        .get_matches();

    let verbosity = matches.occurrences_of("v") as usize;
    env_logger::builder()
        .filter_level(match &verbosity {
            0 => LevelFilter::Error,
            1 => LevelFilter::Warn,
            2 => LevelFilter::Info,
            3 => LevelFilter::Debug,
            _ => LevelFilter::Trace,
        })
        .init();

    match matches.subcommand_matches("contrast") {
        Some(matches) => {
            let color_1_str = matches.value_of("color_1").unwrap();
            let color_2_str = matches.value_of("color_2").unwrap();

            let color_1 = parse_color(color_1_str).unwrap();
            let color_2 = parse_color(color_2_str).unwrap();

            contrast::print_contrast(&color_1, &color_2, verbosity)
                .expect("Could not print contrast.");
        }
        None => eprintln!("No subcommand provided. See --help."),
    }
}

use std::collections::HashSet;
use std::fmt::Display;
use std::io::Write;
use std::ops::Deref;

use clap::{App, Arg, SubCommand};
use log::{debug, error, info, LevelFilter};
use rug::Float;
use termcolor::{ColorChoice, StandardStream};

use color_utils::color::rgb::Rgb;
use color_utils::contrast::{contrast_ratio_levels_reached, contrast_ratio_val};
use color_utils::error::ParsingError;

use crate::color_printing::print_color;

mod color_printing;

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
                    Ok(color_2) => print_contrast(&color_1, &color_2, verbosity).expect("Could not print contrast.")
                },
            }
        }
        None => error!("No subcommand provided. See --help.")
    }
}

fn floor_n_decimals(val: &Float, n: u32) -> Float {
    let factor = 10_i32.pow(n);
    let tmp = val.clone() * factor;
    tmp.floor() / factor
}

fn hash_set_as_sorted_vec<T: Ord>(hash_set: HashSet<T>) -> Vec<T> {
    let mut set_copy_vec = hash_set.into_iter().collect::<Vec<_>>();
    set_copy_vec.sort();
    set_copy_vec
}

fn print_contrast(color_1: &Rgb, color_2: &Rgb, verbosity: usize) -> std::io::Result<()> {
    let contrast_ratio_val = contrast_ratio_val(color_1, color_2);
    let contrast_levels_reached = contrast_ratio_levels_reached(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "WCAG 2.0 contrast ratio for ")?;
    print_color(&mut stdout, color_1)?;
    write!(&mut stdout, " to ")?;
    print_color(&mut stdout, color_2)?;

    let printable_contrast_ratio_val: Box<dyn Display> = if verbosity == 0 {
        // Usually only displaying the last 2 digits is enough.
        // Note that we cannot use the rounding provided by the formatter as contrast values may not be rounded up.
        Box::new(floor_n_decimals(&contrast_ratio_val, 2).to_f32())
    } else {
        Box::new(contrast_ratio_val)
    };
    writeln!(&mut stdout, " is {}.", printable_contrast_ratio_val.deref())?;

    let contrast_levels_reached_string: String = if contrast_levels_reached.is_empty() {
        String::from("None")
    } else {
        hash_set_as_sorted_vec(contrast_levels_reached)
            .iter()
            .map(|level| level.to_string())
            .collect::<Vec<String>>().join(", ")
    };
    writeln!(&mut stdout, "Contrast level(s) reached: {}.", contrast_levels_reached_string)
}

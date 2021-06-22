use std::collections::HashSet;
use std::io::Write;

use clap::{App, Arg, SubCommand};
use rug::Float;
use termcolor::{ColorChoice, StandardStream};

use color_utils::color::rgb::Rgb;
use color_utils::contrast::{contrast_ratio_levels_reached, contrast_ratio_val};
use color_utils::error::ParsingError;

use crate::color_printing::print_color;

mod color_printing;

const COLOR_ARG_HELP: &str = "CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'.";

struct Options {
    verbosity: u8,
}

fn parse_color(slice: &str) -> Result<Rgb, ParsingError> {
    // TODO: Allow specifying which format should be used instead of trying all.
    Rgb::from_hex_str(slice)
        .or_else(|_| Rgb::from_rgb_function_str(slice))
}

fn main() {
    let matches = App::new("Colo.rs")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity.")
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

    let options = Options { verbosity: matches.occurrences_of("v") as u8 };
    match matches.subcommand_matches("contrast") {
        Some(matches) => {
            let color_1_str = matches.value_of("color_1").unwrap();
            let color_2_str = matches.value_of("color_2").unwrap();
            match parse_color(color_1_str) {
                Err(e) => eprintln!("Color 1: {}", e),
                Ok(color_1) => match parse_color(color_2_str) {
                    Err(e) => eprintln!("Color 2: {}", e),
                    Ok(color_2) => print_contrast(&color_1, &color_2, &options).expect("Could not print contrast.")
                },
            }
        }
        None => eprintln!("No subcommand provided. See --help.")
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

fn print_contrast(color_1: &Rgb, color_2: &Rgb, options: &Options) -> std::io::Result<()> {
    let contrast_ratio_val = contrast_ratio_val(color_1, color_2);
    let contrast_levels_reached = contrast_ratio_levels_reached(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "WCAG 2.0 contrast ratio for ")?;
    print_color(&mut stdout, color_1)?;
    write!(&mut stdout, " to ")?;
    print_color(&mut stdout, color_2)?;

    match options.verbosity {
        verbosity if verbosity >= 1 => writeln!(&mut stdout, " is {}.", contrast_ratio_val)?,
        _ => {
            // Usually only displaying the last 2 digits is enough.
            // Note that we cannot use the rounding provided by the formatter as contrast values may not be rounded up.
            let floored = floor_n_decimals(&contrast_ratio_val, 2);
            writeln!(&mut stdout, " is {:.2}.", floored.to_f32())?
        }
    };

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

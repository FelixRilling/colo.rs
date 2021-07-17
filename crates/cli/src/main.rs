use clap::{App, Arg, ArgGroup, SubCommand};
use log::LevelFilter;

use color_format::ColorFormat;
use options::Options;

mod color_format;
mod color_printing;
mod contrast;
mod options;

fn decorate_color_arg<'a>(arg: Arg<'a, 'a>) -> Arg<'a, 'a> {
    arg.takes_value(true)
        .help(
            "CSS-like color value, e.g. #00FF11 or 'rgb(255 128 0)'. By default, all supported formats are tried. This can be changed via color-format arguments."
        )
}

fn main() {
    let matches = App::new("Colo.rs")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .takes_value(false)
                .help("Increase message verbosity."),
        )
        .arg(
            Arg::with_name("rgb-hex")
                .long("rgb-hex")
                .takes_value(false)
                .help("Use RGB hexadecimal format for input/output."),
        )
        .arg(
            Arg::with_name("rgb-function")
                .long("rgb-function")
                .takes_value(false)
                .help("Use RGB function format for input/output."),
        )
        .group(
            ArgGroup::with_name("color-format")
                .required(false)
                .arg("rgb-hex")
                .arg("rgb-function"),
        )
        .subcommand(
            SubCommand::with_name("contrast")
                .about("Calculate WCAG contrast of two colors.")
                .arg(decorate_color_arg(Arg::with_name("color-1").required(true)))
                .arg(decorate_color_arg(Arg::with_name("color-2").required(true))),
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

    let format = match matches {
        _ if matches.is_present("rgb-hex") => ColorFormat::RgbHex,
        _ if matches.is_present("rgb-function") => ColorFormat::RgbFunction,
        _ => ColorFormat::Auto,
    };

    let options = Options { verbosity, format };

    match matches.subcommand_matches("contrast") {
        Some(matches) => {
            let color_1_str = matches.value_of("color-1").unwrap();
            let color_2_str = matches.value_of("color-2").unwrap();

            match color_format::parse_color(color_1_str, &options.format) {
                Err(e_1) => eprintln!("Could not parse color 1: {}.", e_1),
                Ok(color_1) => match color_format::parse_color(color_2_str, &options.format) {
                    Err(e_2) => eprintln!("Could not parse color 2: {}.", e_2),
                    Ok(color_2) => contrast::print_contrast(&color_1, &color_2, &options)
                        .expect("Could not print contrast."),
                },
            }
        }
        None => eprintln!("No subcommand provided. See --help."),
    }
}

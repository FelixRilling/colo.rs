extern crate clap;

use std::io::Write;
use std::str::FromStr;

use clap::{App, Arg, SubCommand};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::color::RGB;
use crate::contrast::{get_best_contrast, wcag_contrast_ratio};

mod color;
mod contrast;

fn main() {
    let matches = App::new("Colo.rs")
        .subcommand(SubCommand::with_name("contrast")
            .arg(
                Arg::with_name("color_1")
                    .required(true)
            ).arg(
            Arg::with_name("color_2")
                .required(true)
        ))
        .get_matches();


    match matches.subcommand_matches("contrast") {
        Some(matches) => {
            let color_1 = RGB::from_str(matches.value_of("color_1").unwrap()).unwrap();
            let color_2 = RGB::from_str(matches.value_of("color_2").unwrap()).unwrap();
            print_contrast(&color_1, &color_2)
        }
        None => {
            panic!("TODO!")
        }
    }
}

const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
const WHITE: RGB = RGB { r: 255, g: 255, b: 255 };

fn rgb_as_term_color(color: &RGB) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

/// Prints colored color value to stream. Stream color is reset afterwards.
fn print_rgb(stdout: &mut StandardStream, color: &RGB) {
    let foreground_color_options = vec![&BLACK, &WHITE];
    let foreground_color = get_best_contrast(color, &foreground_color_options);
    stdout.set_color(ColorSpec::new()
        .set_bg(Some(rgb_as_term_color(color)))
        .set_fg(Some(rgb_as_term_color(foreground_color))));
    write!(stdout, "{}", color);
    stdout.set_color(&ColorSpec::default());
}

fn print_contrast(color_1: &RGB, color_2: &RGB) {
    let contrast = wcag_contrast_ratio(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    write!(&mut stdout, "WCAG 2.0 contrast ratio for ");
    print_rgb(&mut stdout, color_1);
    write!(&mut stdout, " to ");
    print_rgb(&mut stdout, color_2);
    writeln!(&mut stdout, " is {}. ", contrast);
}

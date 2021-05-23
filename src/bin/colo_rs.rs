use std::collections::HashSet;
use std::io::Write;
use std::iter::FromIterator;

use clap::{App, Arg, SubCommand};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use colo_rs::color::RGB;
use colo_rs::contrast::{contrast_ratio_levels_reached, contrast_ratio_val};

fn rgb_as_term_color(color: &RGB) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

/// Finds and returns the `color_options` value that has the best contrast to `initial_color`.
fn get_best_contrast<'a>
(initial_color: &'a RGB, color_options: &'a Vec<&RGB>) -> &'a RGB {
    let mut best_contrast_ratio: f32 = 0.0;
    // Default value only matters if all options have zero contrast, so they should be the same as initial_color anyways.
    let mut best_contrast_ratio_color: &RGB = initial_color;

    for color_option in color_options {
        let contrast_ratio = contrast_ratio_val(initial_color, color_option);
        if contrast_ratio > best_contrast_ratio {
            best_contrast_ratio = contrast_ratio;
            best_contrast_ratio_color = color_option;
        }
    }

    best_contrast_ratio_color
}


const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
const WHITE: RGB = RGB { r: 255, g: 255, b: 255 };

/// Prints colored color value to stream. Stream color is reset afterwards.
fn print_rgb(stdout: &mut StandardStream, color: &RGB) {
    let foreground_color_options = vec![&BLACK, &WHITE];
    let foreground_color = get_best_contrast(color, &foreground_color_options);

    stdout.set_color(ColorSpec::new()
        .set_bg(Some(rgb_as_term_color(color)))
        .set_fg(Some(rgb_as_term_color(foreground_color))))
        .expect("Could not set stdout color.");
    write!(stdout, "{}", color).expect("Could not write color to stdout.");
    stdout.set_color(&ColorSpec::default()).expect("Could not reset stdout color.");
}

fn set_as_ordered_vec<T: Ord>(hash_set: HashSet<T>) -> Vec<T> {
    let mut set_copy_vec: Vec<T> = Vec::from_iter(hash_set.into_iter());
    set_copy_vec.sort();
    set_copy_vec
}

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
            let color_1 = RGB::from_hex_str(matches.value_of("color_1").unwrap()).unwrap();
            let color_2 = RGB::from_hex_str(matches.value_of("color_2").unwrap()).unwrap();
            print_contrast(&color_1, &color_2)
        }
        None => {
            panic!("TODO!")
        }
    }
}

fn print_contrast(color_1: &RGB, color_2: &RGB) {
    let contrast_ratio_val = contrast_ratio_val(color_1, color_2);
    let contrast_levels_reached = contrast_ratio_levels_reached(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "WCAG 2.0 contrast ratio for ").unwrap();
    print_rgb(&mut stdout, color_1);
    write!(&mut stdout, " to ").unwrap();
    print_rgb(&mut stdout, color_2);
    writeln!(&mut stdout, " is {}.", contrast_ratio_val).unwrap();

    let contrast_levels_reached_string: String = if contrast_levels_reached.is_empty() {
        String::from("None")
    } else {
        set_as_ordered_vec(contrast_levels_reached)
            .iter()
            .map(|level| level.to_string())
            .collect::<Vec<String>>().join(", ")
    };
    writeln!(&mut stdout, "Contrast level(s) reached: {}.", contrast_levels_reached_string).unwrap();
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use colo_rs::color::RGB;

    use crate::{BLACK, get_best_contrast, WHITE};

    #[test]
    fn get_best_contrast_finds_result() {
        let bright_color = RGB::from_str("#ABCDEF").unwrap();
        let dark_color = RGB::from_str("#696969").unwrap();

        let options = vec![&BLACK, &WHITE];

        let bright_color_best_contrast_actual = get_best_contrast(&bright_color, &options);
        assert_eq!(bright_color_best_contrast_actual, &BLACK);

        let dark_color_best_contrast_actual = get_best_contrast(&dark_color, &options);
        assert_eq!(dark_color_best_contrast_actual, &WHITE);
    }
}

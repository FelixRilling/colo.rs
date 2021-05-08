use termcolor::{Color, StandardStream, ColorSpec, WriteColor, ColorChoice};
use crate::core::color::{RGB};
use crate::core::contrast::{contrast_ratio_val};
use std::io::Write;

fn rgb_as_term_color(color: &RGB) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}

const BLACK: RGB = RGB { r: 0, g: 0, b: 0 };
const WHITE: RGB = RGB { r: 255, g: 255, b: 255 };

/// Finds and returns the `color_options` value that has the best contrast to `initial_color`.
fn get_best_contrast<'a>
(initial_color: &'a RGB, color_options: &'a Vec<&RGB>) -> &'a RGB {
    let mut best_contrast_ratio: f32 = 0.0;
    let mut best_contrast_ratio_color: &RGB = initial_color; // Default value only matters if all options have zero contrast, so they should be the same as initial_color anyways.

    for color_option in color_options {
        let contrast_ratio = contrast_ratio_val(initial_color, color_option);
        if contrast_ratio > best_contrast_ratio {
            best_contrast_ratio = contrast_ratio;
            best_contrast_ratio_color = color_option;
        }
    }

    best_contrast_ratio_color
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

pub fn print_contrast(color_1: &RGB, color_2: &RGB) {
    let contrast = contrast_ratio_val(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);
    write!(&mut stdout, "WCAG 2.0 contrast ratio for ");
    print_rgb(&mut stdout, color_1);
    write!(&mut stdout, " to ");
    print_rgb(&mut stdout, color_2);
    writeln!(&mut stdout, " is {}. ", contrast);
}

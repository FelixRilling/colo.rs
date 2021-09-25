use std::collections::HashSet;
use std::io::Write;

use palette::{RelativeContrast, Srgba};
use rug::Float;
use termcolor::{ColorChoice, StandardStream};

use color_utils::contrast::aa_aaa::{contrast_ratio_levels_reached, contrast_ratio_val};
use color_utils_internal::float::float_to_string;

use crate::color_printing::print_color;
use crate::options::Options;

fn floor_n_decimals(val: f32, n: u32) -> f32 {
    let factor: f32 = 10_i16.pow(n).into();
    let tmp = val * factor;
    tmp.floor() / factor
}

fn hash_set_as_sorted_vec<T: Ord>(hash_set: HashSet<T>) -> Vec<T> {
    let mut set_copy_vec = hash_set.into_iter().collect::<Vec<_>>();
    set_copy_vec.sort();
    set_copy_vec
}

pub fn print_contrast(color_1: &Srgba, color_2: &Srgba, options: &Options) -> std::io::Result<()> {
    let mut out = StandardStream::stdout(ColorChoice::Auto);

    print_contrast_ratio(&mut out, color_1, color_2, options)?;

    print_contrast_levels_reached(&mut out, color_1, color_2)
}

fn print_contrast_ratio(
    out: &mut StandardStream,
    color_1: &Srgba,
    color_2: &Srgba,
    options: &Options,
) -> std::io::Result<()> {
    write!(out, "WCAG 2.0 AA/AAA contrast ratio for ")?;
    print_color(out, color_1, &options.format)?;
    write!(out, " to ")?;
    print_color(out, color_2, &options.format)?;

    let contrast_ratio = color_1.get_contrast_ratio(color_2);
    let contrast_ratio_str = if options.verbosity == 0 {
        // Usually only displaying the last 2 digits is enough.
        let floored_val = floor_n_decimals(contrast_ratio, 2);
        floored_val.to_string()
    } else {
        contrast_ratio.to_string()
    };
    writeln!(out, " is {}.", contrast_ratio_str)
}

fn print_contrast_levels_reached(
    out: &mut StandardStream,
    color_1: &Srgba,
    color_2: &Srgba,
) -> std::io::Result<()> {
    let contrast_levels_reached =
        contrast_ratio_levels_reached(&color_1.to_owned().into(), &color_2.to_owned().into());
    let contrast_levels_reached_str: String = if contrast_levels_reached.is_empty() {
        String::from("None")
    } else {
        hash_set_as_sorted_vec(contrast_levels_reached)
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>()
            .join(", ")
    };
    writeln!(
        out,
        "Contrast level(s) reached: {}.",
        contrast_levels_reached_str
    )
}

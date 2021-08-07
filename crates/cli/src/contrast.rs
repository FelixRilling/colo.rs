use std::collections::HashSet;
use std::io::Write;

use rug::Float;
use termcolor::{ColorChoice, StandardStream};

use color_utils::contrast::{contrast_ratio_levels_reached, contrast_ratio_val};
use color_utils::rgb::Rgb;
use color_utils_internal::float::float_to_string;

use crate::color_printing::print_color;
use crate::options::Options;

fn floor_n_decimals(val: Float, n: u32) -> Float {
    let factor = 10_i32.pow(n);
    let tmp = val * factor;
    tmp.floor() / factor
}

fn hash_set_as_sorted_vec<T: Ord>(hash_set: HashSet<T>) -> Vec<T> {
    let mut set_copy_vec = hash_set.into_iter().collect::<Vec<_>>();
    set_copy_vec.sort();
    set_copy_vec
}

pub fn print_contrast(color_1: &Rgb, color_2: &Rgb, options: &Options) -> std::io::Result<()> {
    let contrast_ratio_val = contrast_ratio_val(color_1, color_2);
    let contrast_levels_reached = contrast_ratio_levels_reached(color_1, color_2);

    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "WCAG 2.0 contrast ratio for ")?;
    print_color(&mut stdout, color_1, &options.format)?;
    write!(&mut stdout, " to ")?;
    print_color(&mut stdout, color_2, &options.format)?;

    let contrast_ratio_val_str = if options.verbosity == 0 {
        // Usually only displaying the last 2 digits is enough.
        let floored_val = floor_n_decimals(contrast_ratio_val, 2);
        float_to_string(&floored_val)
    } else {
        contrast_ratio_val.to_string()
    };
    writeln!(&mut stdout, " is {}.", contrast_ratio_val_str)?;

    let contrast_levels_reached_str: String = if contrast_levels_reached.is_empty() {
        String::from("None")
    } else {
        hash_set_as_sorted_vec(contrast_levels_reached)
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<String>>().join(", ")
    };
    writeln!(&mut stdout, "Contrast level(s) reached: {}.", contrast_levels_reached_str)
}

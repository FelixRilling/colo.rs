use std::io::Write;

use termcolor::{ColorChoice, StandardStream};

use color_utils::rgb::Rgb;

use crate::color_format::ColorFormat;
use crate::color_printing::print_color;
use crate::options::Options;

pub fn print_details(color: &Rgb, options: &Options) -> std::io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Auto);

    write!(&mut stdout, "Details for color ")?;
    print_color(&mut stdout, color, &options.format)?;
    writeln!(&mut stdout, ":")?;

    write!(&mut stdout, "In RGB hexadecimal notation: ")?;
    print_color(&mut stdout, color, &ColorFormat::RgbHex)?;
    writeln!(&mut stdout, ".")?;

    write!(&mut stdout, "In RGB function notation: ")?;
    print_color(&mut stdout, color, &ColorFormat::RgbFunction)?;
    writeln!(&mut stdout, ".")
}

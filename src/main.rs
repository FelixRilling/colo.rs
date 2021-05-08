extern crate clap;

use std::str::FromStr;

use clap::{App, Arg, SubCommand};

use crate::cli::print_contrast;
use crate::core::color::RGB;

mod core {
    pub(crate) mod color;
    pub(crate) mod contrast;
}

mod cli;

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

use crate::color_format::ColorFormat;

#[derive(Debug)]
pub struct Options {
    pub verbosity: u8,
    pub format: ColorFormat,
}

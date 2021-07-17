use crate::color_format::ColorFormat;

#[derive(Debug)]
pub struct Options {
    pub verbosity: usize,
    pub format: ColorFormat,
}

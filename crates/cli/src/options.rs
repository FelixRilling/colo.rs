#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Options {
	pub format: ColorFormat,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, clap::ValueEnum)]
pub enum ColorFormat {
	Auto,
	RgbHex,
	RgbFunction,
	HslFunction,
	HwbFunction,
}

use num_traits::Pow;

fn factor(n: u8) -> f64 {
	10_f64.pow(n)
}

pub fn floor_n_decimals(val: f64, n: u8) -> f64 {
	let factor = factor(n);
	(val * factor).floor() / factor
}

pub fn ceil_n_decimals(val: f64, n: u8) -> f64 {
	let factor = factor(n);
	(val * factor).ceil() / factor
}

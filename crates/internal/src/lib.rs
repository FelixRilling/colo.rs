use num_traits::Pow;

pub fn floor_n_decimals(val: f64, n: u8) -> f64 {
    let factor = 10f64.pow(n);
    (val * factor).floor() / factor
}

pub fn ceil_n_decimals(val: f64, n: u8) -> f64 {
    let factor = 10f64.pow(n);
    (val * factor).ceil() / factor
}

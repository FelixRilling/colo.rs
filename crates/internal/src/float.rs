use rug::Float;

/// Formats a `Float` as a string.
/// Unlike the built-in `Display` impl of `Float`, this version drops trailing decimal zeros.
#[inline]
pub fn float_to_string(f: &Float) -> String {
    // Using f64's formatting because:
    // - it drops trailing zeros.
    // - f64's floating point precision is usually good enough for output here.
    f.to_f64().to_string()
}

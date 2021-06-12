use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

use rug::Float;
use rug::ops::Pow;

use crate::color::rgb::RGB;
use crate::color::srgb::rgb_to_srgb;

/// Contrast target values based on
/// <https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast>.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ContrastLevel {
    /// Enhanced contrast for text.
    AAA,

    /// Enhanced contrast for large text.
    LargeAAA,

    /// Minimum contrast for text.
    AA,

    /// Minimum contrast for large text.
    LargeAA,
}

impl Display for ContrastLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match &self {
            ContrastLevel::AAA => "AAA",
            ContrastLevel::LargeAAA => "AAA (Large Text)",
            ContrastLevel::AA => "AA",
            ContrastLevel::LargeAA => "AA (Large Text)",
        })
    }
}


/// Checks which WCAG contrast ratio levels this combination of colors reach.
/// An empty set means no level is reached.
/// See <https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast> for details.
// https://webaim.org/resources/contrastchecker/
pub fn contrast_ratio_levels_reached(color_1: &RGB, color_2: &RGB) -> HashSet<ContrastLevel> {
    let ratio = contrast_ratio_val(color_1, color_2);

    let mut reached = HashSet::new();
    // TODO: Use match here or something.
    if ratio.ge(&3.0) {
        reached.insert(ContrastLevel::LargeAA);
        if ratio.ge(&4.5) {
            reached.insert(ContrastLevel::AA);
            reached.insert(ContrastLevel::LargeAAA);
            if ratio.ge(&7.0) {
                reached.insert(ContrastLevel::AAA);
            }
        }
    }
    reached
}

/// Calculates the WCAG color ratio of two colors.
/// The same color inputs will produce the same output regardless of position.
///
/// The result is the number on the left side of the WCAG color ratio display syntax.
/// E.g. an result of "4.5" would be written "4.5:1".
// https://www.w3.org/TR/WCAG20-TECHS/G18.html#G18-tests
pub fn contrast_ratio_val(color_1: &RGB, color_2: &RGB) -> Float {
    let color_1_luminance = relative_luminance(color_1);
    let color_2_luminance = relative_luminance(color_2);

    let lighter_color_luminance;
    let darker_color_luminance;
    if color_1_luminance > color_2_luminance {
        lighter_color_luminance = color_1_luminance;
        darker_color_luminance = color_2_luminance;
    } else {
        lighter_color_luminance = color_2_luminance;
        darker_color_luminance = color_1_luminance;
    }

    (lighter_color_luminance + 0.05) / (darker_color_luminance + 0.05)
}

fn relative_luminance(color: &RGB) -> Float {
    return 0.2126 * transform_color_value(color.red())
        + 0.7152 * transform_color_value(color.green())
        + 0.0722 * transform_color_value(color.blue());
}

fn transform_color_value(rgb_val: u8) -> Float {
    let rgbs_val = rgb_to_srgb(rgb_val);
    if rgbs_val <= 0.03928 {
        rgbs_val / 12.92
    } else {
        let tmp: Float = (rgbs_val + 0.055) / 1.055;
        tmp.pow(2.4)
    }
}

#[cfg(test)]
mod tests {
    use rug::float::Round;

    use crate::color::rgb::RGB;
    use crate::contrast::ContrastLevel;

    use super::*;

    #[test]
    fn contrast_ratio_levels_reached_same_color() {
        let black = RGB::from_hex_str("#000000").unwrap();

        let actual = contrast_ratio_levels_reached(&black, &black);
        assert!(actual.is_empty());
    }

    #[test]
    fn contrast_ratio_levels_reached_max_contrast() {
        let black = RGB::from_hex_str("#000000").unwrap();
        let white = RGB::from_hex_str("#FFFFFF").unwrap();

        let actual = contrast_ratio_levels_reached(&black, &white);
        assert!(actual.contains(&ContrastLevel::AAA));
        assert!(actual.contains(&ContrastLevel::LargeAAA));
        assert!(actual.contains(&ContrastLevel::AA));
        assert!(actual.contains(&ContrastLevel::LargeAA));
    }

    #[test]
    fn contrast_ratio_levels_reached_ignores_order() {
        let a = RGB::from_hex_str("#90B5AC").unwrap();
        let b = RGB::from_hex_str("#662270").unwrap();

        let actual_1 = contrast_ratio_levels_reached(&a, &b);
        let actual_2 = contrast_ratio_levels_reached(&b, &a);
        assert_eq!(actual_1, actual_2)
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=171717
    #[test]
    fn contrast_ratio_levels_reached_lowest() {
        let a = RGB::from_hex_str("#000000").unwrap();
        let b = RGB::from_hex_str("#171717").unwrap();

        let actual = contrast_ratio_levels_reached(&a, &b);
        assert!(actual.is_empty())
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=5C5C5C
    #[test]
    fn contrast_ratio_levels_reached_low() {
        let a = RGB::from_hex_str("#000000").unwrap();
        let b = RGB::from_hex_str("#5C5C5C").unwrap();

        let actual = contrast_ratio_levels_reached(&a, &b);
        assert_eq!(actual.len(), 1);
        assert!(actual.contains(&ContrastLevel::LargeAA))
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=757575
    #[test]
    fn contrast_ratio_levels_reached_average() {
        let a = RGB::from_hex_str("#000000").unwrap();
        let b = RGB::from_hex_str("#757575").unwrap();

        let actual = contrast_ratio_levels_reached(&a, &b);
        assert_eq!(actual.len(), 3);
        assert!(actual.contains(&ContrastLevel::LargeAA));
        assert!(actual.contains(&ContrastLevel::AA));
        assert!(actual.contains(&ContrastLevel::LargeAAA));
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=969696
    #[test]
    fn contrast_ratio_levels_reached_high() {
        let a = RGB::from_hex_str("#000000").unwrap();
        let b = RGB::from_hex_str("#969696").unwrap();

        let actual = contrast_ratio_levels_reached(&a, &b);
        assert_eq!(actual.len(), 4);
        assert!(actual.contains(&ContrastLevel::LargeAA));
        assert!(actual.contains(&ContrastLevel::AA));
        assert!(actual.contains(&ContrastLevel::LargeAAA));
        assert!(actual.contains(&ContrastLevel::AAA));
    }


    #[test]
    fn contrast_ratio_val_same_color() {
        let black = RGB::from_hex_str("#000000").unwrap();

        let expected: f64 = 1.0;
        let actual = contrast_ratio_val(&black, &black);
        assert_eq!(actual.to_f64_round(Round::Down), expected)
    }

    #[test]
    fn contrast_ratio_val_max_contrast() {
        let black = RGB::from_hex_str("#000000").unwrap();
        let white = RGB::from_hex_str("#FFFFFF").unwrap();

        let expected: f64 = 21.0;
        let actual = contrast_ratio_val(&black, &white);
        assert_eq!(actual.to_f64_round(Round::Down), expected)
    }

    #[test]
    fn contrast_ratio_val_ignores_order() {
        let a = RGB::from_hex_str("#90B5AC").unwrap();
        let b = RGB::from_hex_str("#662270").unwrap();

        let actual_1 = contrast_ratio_val(&a, &b);
        let actual_2 = contrast_ratio_val(&b, &a);
        assert_eq!(actual_1, actual_2)
    }
}

use std::collections::HashSet;

use crate::core::color::RGB;

/// Contrast target values based on
/// https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast.
#[derive(Hash, Eq, PartialEq, Debug)]
pub enum ContrastTarget {
    /// Enhanced contrast for text.
    AAA,
    /// Enhanced contrast for large text.
    LargeAAA,
    /// Minimum contrast for text.
    AA,
    /// Minimum contrast for large text.
    LargeAA,
}

/// Checks which WCAG contrast ratio targets this combination of colors reach.
/// An empty set means no target is reached.
/// See https://www.w3.org/TR/2008/REC-WCAG20-20081211/#visual-audio-contrast-contrast for details.
// https://webaim.org/resources/contrastchecker/
pub fn contrast_ratio_targets_reached(color_1: &RGB, color_2: &RGB) -> HashSet<ContrastTarget> {
    let ratio = contrast_ratio_val(color_1, color_2);

    let mut targets_reached = HashSet::new();
    // TODO: Use match here or something.
    if ratio.ge(&3.0) {
        targets_reached.insert(ContrastTarget::LargeAA);
        if ratio.ge(&4.5) {
            targets_reached.insert(ContrastTarget::AA);
            targets_reached.insert(ContrastTarget::LargeAAA);
            if ratio.ge(&7.0) {
                targets_reached.insert(ContrastTarget::AAA);
            }
        }
    }
    targets_reached
}

/// Calculates the WCAG color ratio of two colors.
/// The same color inputs will produce the same output regardless of position.
///
/// The result is the number on the left side of the WCAG color ratio display syntax.
/// E.g. an result of "4.5" would be written "4.5:1".
// https://www.w3.org/TR/WCAG20-TECHS/G18.html#G18-tests
pub fn contrast_ratio_val(color_1: &RGB, color_2: &RGB) -> f32 {
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

fn relative_luminance(color: &RGB) -> f32 {
    return 0.2126 * transform_color_value(color.r)
        + 0.7152 * transform_color_value(color.g)
        + 0.0722 * transform_color_value(color.b);
}

fn transform_color_value(rgb_val: u8) -> f32 {
    let adapted_val = f32::from(rgb_val) / 255.0;
    if adapted_val <= 0.03928 {
        adapted_val / 12.92
    } else {
        ((adapted_val + 0.055) / 1.055).powf(2.4)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use float_cmp::approx_eq;

    use crate::core::color::RGB;
    use crate::core::contrast::ContrastTarget;

    use super::{contrast_ratio_targets_reached, contrast_ratio_val};

    #[test]
    fn contrast_ratio_targets_reached_same_color() {
        let black = RGB::from_str("#000000").unwrap();

        let actual = contrast_ratio_targets_reached(&black, &black);
        assert!(actual.is_empty());
    }

    #[test]
    fn contrast_ratio_targets_reached_max_contrast() {
        let black = RGB::from_str("#000000").unwrap();
        let white = RGB::from_str("#FFFFFF").unwrap();

        let actual = contrast_ratio_targets_reached(&black, &white);
        assert!(actual.contains(&ContrastTarget::AAA));
        assert!(actual.contains(&ContrastTarget::LargeAAA));
        assert!(actual.contains(&ContrastTarget::AA));
        assert!(actual.contains(&ContrastTarget::LargeAA));
    }

    #[test]
    fn contrast_ratio_targets_reached_ignores_order() {
        let a = RGB::from_str("#90B5AC").unwrap();
        let b = RGB::from_str("#662270").unwrap();

        let actual_1 = contrast_ratio_targets_reached(&a, &b);
        let actual_2 = contrast_ratio_targets_reached(&b, &a);
        assert_eq!(actual_1, actual_2)
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=171717
    #[test]
    fn contrast_ratio_targets_reached_lowest() {
        let a = RGB::from_str("#000000").unwrap();
        let b = RGB::from_str("#171717").unwrap();

        let actual = contrast_ratio_targets_reached(&a, &b);
        assert!(actual.is_empty())
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=5C5C5C
    #[test]
    fn contrast_ratio_targets_reached_low() {
        let a = RGB::from_str("#000000").unwrap();
        let b = RGB::from_str("#5C5C5C").unwrap();

        let actual = contrast_ratio_targets_reached(&a, &b);
        assert_eq!(actual.len(), 1);
        assert!(actual.contains(&ContrastTarget::LargeAA))
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=757575
    #[test]
    fn contrast_ratio_targets_reached_average() {
        let a = RGB::from_str("#000000").unwrap();
        let b = RGB::from_str("#757575").unwrap();

        let actual = contrast_ratio_targets_reached(&a, &b);
        assert_eq!(actual.len(), 3);
        assert!(actual.contains(&ContrastTarget::LargeAA));
        assert!(actual.contains(&ContrastTarget::AA));
        assert!(actual.contains(&ContrastTarget::LargeAAA));
    }

    // https://webaim.org/resources/contrastchecker/?fcolor=000000&bcolor=969696
    #[test]
    fn contrast_ratio_targets_reached_high() {
        let a = RGB::from_str("#000000").unwrap();
        let b = RGB::from_str("#969696").unwrap();

        let actual = contrast_ratio_targets_reached(&a, &b);
        assert_eq!(actual.len(), 4);
        assert!(actual.contains(&ContrastTarget::LargeAA));
        assert!(actual.contains(&ContrastTarget::AA));
        assert!(actual.contains(&ContrastTarget::LargeAAA));
        assert!(actual.contains(&ContrastTarget::AAA));
    }


    #[test]
    fn contrast_ratio_val_same_color() {
        let black = RGB::from_str("#000000").unwrap();

        let expected: f32 = 1.0;
        let actual = contrast_ratio_val(&black, &black);
        assert!(approx_eq!(f32, actual, expected, ulps = 2));
    }

    #[test]
    fn contrast_ratio_val_max_contrast() {
        let black = RGB::from_str("#000000").unwrap();
        let white = RGB::from_str("#FFFFFF").unwrap();

        let expected: f32 = 21.0;
        let actual = contrast_ratio_val(&black, &white);
        assert!(approx_eq!(f32, actual, expected, ulps = 2));
    }

    #[test]
    fn contrast_ratio_val_ignores_order() {
        let a = RGB::from_str("#90B5AC").unwrap();
        let b = RGB::from_str("#662270").unwrap();

        let actual_1 = contrast_ratio_val(&a, &b);
        let actual_2 = contrast_ratio_val(&b, &a);
        assert_eq!(actual_1, actual_2)
    }
}

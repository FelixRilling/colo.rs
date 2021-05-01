use crate::color::RGB;

/// Calculates the WCAG color ratio of two colors
// https://css-tricks.com/understanding-web-accessibility-color-contrast-guidelines-and-ratios/
// https://www.w3.org/TR/WCAG20-TECHS/G18.html#G18-tests
// https://github.com/tmcw/wcag-contrast/blob/master/index.js
pub fn wcag_contrast_ratio(color_1: &RGB, color_2: &RGB) -> f32 {
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

use std::fmt;

pub mod angle;
pub mod ratio;
use angle::Angle;
use ratio::Ratio;

/// A trait that can be used for converting between different color models
/// and performing various transformations on them.
pub trait Color {
    /// Converts `self` to its CSS string format.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, ratio::Ratio as Ratio};
    ///
    /// let salmon = RGB::new(250, 128, 114);
    /// let opaque_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    /// assert_eq!(opaque_salmon.to_css(), "rgba(250, 128, 114, 0.50)");
    /// ```
    fn to_css(self) -> String;

    /// Converts `self` into its RGB representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, ratio::Ratio as Ratio};
    ///
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(opaque_tomato.to_rgb(), RGB::new(255, 99, 71));
    /// ```
    fn to_rgb(self) -> RGB;

    /// Converts `self` into its RGBA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, ratio::Ratio as Ratio};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    ///
    /// assert_eq!(tomato.to_rgba(), RGBA::new(255, 99, 71, 255));
    /// ```
    fn to_rgba(self) -> RGBA;

    /// Converts `self` into its HSL representation.
    /// When converting from a color model that supports an alpha channel
    /// (e.g. RGBA), the alpha value will not be preserved.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSL, angle::Angle as Angle, ratio::Ratio as Ratio};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(tomato.to_hsl(), HSL::new(9, 100, 64));
    /// assert_eq!(opaque_tomato.to_hsl(), HSL::new(9, 100, 64));
    /// ```
    fn to_hsl(self) -> HSL;

    /// Converts `self` into its HSLA representation.
    /// When converting from a color model that does not supports an alpha channel
    /// (e.g. RGB), it will be treated as fully opaque.
    ///
    /// # Examples
    /// ```
    /// use css_colors::{Color, RGB, RGBA, HSLA, angle::Angle as Angle, ratio::Ratio as Ratio};
    ///
    /// let tomato = RGB::new(255, 99, 71);
    /// let opaque_tomato = RGBA::new(255, 99, 71, 128);
    ///
    /// assert_eq!(tomato.to_hsla(), HSLA::new(9, 100, 64, 255));
    /// assert_eq!(opaque_tomato.to_hsla(), HSLA::new(9, 100, 64, 128));
    /// ```
    fn to_hsla(self) -> HSLA;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
///
/// Valid values for r, g, and b must be a u8 between `0-255`, represented as a `Ratio`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgb-color).
pub struct RGB {
    // red
    pub r: Ratio,

    // green
    pub g: Ratio,

    // blue
    pub b: Ratio,
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgb({}, {}, {})",
            self.r.as_u8(),
            self.g.as_u8(),
            self.b.as_u8()
        )
    }
}

impl RGB {
    /// Transforms numerical values into an RGB struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::RGB, css_colors::ratio::Ratio as Ratio};
    ///
    /// let salmon = RGB::new(250, 128, 114);
    ///
    /// assert_eq!(salmon, RGB { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114) });
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB {
            r: Ratio::from_u8(r),
            g: Ratio::from_u8(g),
            b: Ratio::from_u8(b),
        }
    }
}

impl Color for RGB {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self
    }

    fn to_rgba(self) -> RGBA {
        RGBA::new(self.r.as_u8(), self.g.as_u8(), self.b.as_u8(), 255)
    }

    /// The algorithm for converting from rgb to hsl format, which determines
    /// the equivalent luminosity, saturation, and hue.
    fn to_hsl(self) -> HSL {
        let RGB { r, g, b } = self;

        // If r, g, and b are the same, the color is a shade of grey (between
        // black and white), with no hue or saturation. In that situation, there
        // is no saturation or hue, and we can use any value to determine luminosity.
        if r == g && g == b {
            return HSL {
                h: Angle::new(0),             // h
                s: Ratio::from_percentage(0), // s
                l: r,                         // l
            };
        }

        // Otherwise, to determine luminosity, we conver the RGB values into a
        // percentage value, find the max and the min of those values, sum them
        // together, and divide by 2.
        let r = self.r.as_f32();
        let g = self.g.as_f32();
        let b = self.b.as_f32();

        // let max = vec![r, g, b].iter().max().to_f32()
        let max = if r > g && r > b {
            r
        } else if g > b {
            g
        } else {
            b
        };

        let min = if r < g && r < b {
            r
        } else if g < b {
            g
        } else {
            b
        };

        let luminosity = (max + min) / 2.0;

        // To find the saturation, we look at the max and min values.
        // If the max and the min are the same, there is no saturation to the color.
        // Otherwise, we calculate the saturation based on if the luminosity is
        // greater than or less than 0.5.
        let saturation = if max == min {
            0.0
        } else if luminosity < 0.5 {
            (max - min) / (max + min)
        } else {
            (max - min) / (2.0 - max - min)
        };

        // To calculate the hue, we look at which value (r, g, or b) is the max.
        // Based on that, we subtract the difference between the other two values,
        // adding 2.0 or 4.0 to account for the degrees on the color wheel, and
        // then dividing that by the difference between the max and the min values.
        // Finally, we multiply the hue value by 60 to convert it to degrees on
        // the color wheel, accounting for negative hues as well.
        let mut hue = if max == r {
            (g - b) / (max - min)
        } else if max == g {
            2.0 + (b - r) / (max - min)
        } else {
            4.0 + (r - g) / (max - min)
        };

        hue *= 60.0;

        // TODO: handle when hue is negative (add 360 to make it positive).
        assert!(hue >= 0.0, "oops, forgot to handle negative");

        HSL::new(
            hue.round() as u16,                 // h
            (saturation * 100.0).round() as u8, // s
            (luminosity * 100.0).round() as u8, // l
        )
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();

        HSLA::new(h.degrees(), s.as_percentage(), l.as_percentage(), 255)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications.
///
/// Valid values for r, g, and b must be a u8 between `0-255`, represented as a `Ratio`.
/// Alpha (a) values must fall between `0-255`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgba-color).
pub struct RGBA {
    // red
    pub r: Ratio,

    // green
    pub g: Ratio,

    // blue
    pub b: Ratio,

    // alpha
    pub a: u8,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.02})",
            self.r.as_u8(),
            self.g.as_u8(),
            self.b.as_u8(),
            self.a as f32 / 255.0
        )
    }
}

impl RGBA {
    /// Transforms numerical values into an RGBA struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::RGBA, css_colors::ratio::Ratio as Ratio};
    ///
    /// let light_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(light_salmon, RGBA { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114), a: 128 });
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {
            r: Ratio::from_u8(r),
            g: Ratio::from_u8(g),
            b: Ratio::from_u8(b),
            a,
        }
    }
}

impl Color for RGBA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        RGB::new(self.r.as_u8(), self.g.as_u8(), self.b.as_u8())
    }

    fn to_rgba(self) -> RGBA {
        self
    }

    fn to_hsl(self) -> HSL {
        self.to_rgb().to_hsl()
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();
        HSLA::new(h.degrees(), s.as_percentage(), l.as_percentage(), self.a)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// The hue is a degree on the color wheel; 0 (or 360) is red, 120 is green, 240 is blue.
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsl-color).
pub struct HSL {
    // hue
    pub h: Angle,

    // saturation
    pub s: Ratio,

    // luminosity
    pub l: Ratio,
}

impl fmt::Display for HSL {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "hsl({}, {}, {})", self.h.degrees(), self.s, self.l)
    }
}

impl HSL {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSL, css_colors::angle::Angle as Angle, css_colors::ratio::Ratio as Ratio};
    ///
    /// let salmon = HSL::new(6, 93, 71);
    ///
    /// assert_eq!(salmon, HSL { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71) });
    /// ```
    pub fn new(h: u16, s: u8, l: u8) -> HSL {
        HSL {
            h: Angle::new(h),
            s: Ratio::from_percentage(s),
            l: Ratio::from_percentage(l),
        }
    }

    fn to_rgb_value(value: f32, temp_1: f32, temp_2: f32) -> f32 {
        let converted: f32;

        // Check whether temporary variable * 6 is larger than one.
        if value * 6.0 > 1.0 {
            // If it is larger than 1, check it's product with 2.
            if value * 2.0 > 1.0 {
                // If it is large than 1, check it's product with 3.
                if value * 3.0 > 2.0 {
                    converted = temp_2;
                } else {
                    converted = temp_2 + ((temp_1 - temp_2) * (0.666 - value) * 6.0);
                }
            } else {
                converted = temp_1;
            }
        } else {
            converted = (temp_2 + (temp_1 - temp_2)) * value * 6.0;
        }

        converted
    }
}

impl Color for HSL {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        let h = self.h;
        let s = (Ratio::as_f32(self.s) * 100.0).round() / 100.0;
        let l = (Ratio::as_f32(self.l) * 100.0).round() / 100.0;

        // If there is no saturation, the color is a shade of grey.
        // We can convert the luminosity and set r, g, and b to that value.
        if s == 0.0 {
            return RGB::new(
                Ratio::from_f32(l).as_u8(),
                Ratio::from_f32(l).as_u8(),
                Ratio::from_f32(l).as_u8(),
            );
        }

        // If the color is not a grey, then we need to create a temporary variable to continue with the algorithm.
        let temp_1;

        // If the luminosity is less than 50%, we add 1.0 to the saturation and multiply by the luminosity.
        // Otherwise, we add the luminosity and saturation, and subtract the product of luminosity and saturation from it.
        if l < 50.0 {
            temp_1 = l * (1.0 + s);
        } else {
            temp_1 = (l + s) - (l * s);
        }

        // Another temporary variable.
        let temp_2 = (2.0 * l) - temp_1;

        // Convert the hue by dividing the angle by 360.
        let hue = h.degrees() as f32 / 360.0;

        let mut temporary_r = hue + 0.333;
        let mut temporary_g = hue;
        let mut temporary_b = hue - 0.333;

        // TODO: handle what happens if these temporary rgb's are bigger than 1 or less than 0.
        // can i just use 'as_percentage' here??
        if temporary_r > 1.0 {
            temporary_r -= 1.0;
        }

        if temporary_g > 1.0 {
            temporary_g -= 1.0;
        }

        if temporary_b > 1.0 {
            temporary_b -= 1.0;
        }

        let red = HSL::to_rgb_value(temporary_r, temp_1, temp_2);
        let green = HSL::to_rgb_value(temporary_g, temp_1, temp_2);
        let blue = HSL::to_rgb_value(temporary_b, temp_1, temp_2);

        RGB {
            r: Ratio::from_f32(red),
            g: Ratio::from_f32(green),
            b: Ratio::from_f32(blue),
        }
    }

    fn to_rgba(self) -> RGBA {
        let RGB { r, g, b } = self.to_rgb();

        RGBA::new(r.as_u8(), g.as_u8(), b.as_u8(), 255)
    }

    fn to_hsl(self) -> HSL {
        self
    }

    fn to_hsla(self) -> HSLA {
        HSLA::new(
            self.h.degrees(),
            self.s.as_percentage(),
            self.l.as_percentage(),
            255,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much hue, saturation, and luminosity should be added to create a color.
/// Also handles alpha specifications.
///
/// A valid value for `h` must range between `0-360`.
/// The saturation ranges between `0-100`, where `0` is completely desaturated, and `100` is full saturation.
/// The luminosity ranges between `0-100`, where `0` is no light (black), and `100` is full light (white).
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#hsla-color).
pub struct HSLA {
    // hue
    pub h: Angle,

    // saturation
    pub s: Ratio,

    // luminosity
    pub l: Ratio,

    // alpha
    pub a: u8,
}

impl fmt::Display for HSLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hsla({}, {}, {}, {:.02})",
            self.h,
            self.s,
            self.l,
            self.a as f32 / 255.0
        )
    }
}

impl HSLA {
    /// Transforms numerical values into a HSL struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::HSLA, css_colors::angle::Angle as Angle, css_colors::ratio::Ratio as Ratio};
    /// let light_salmon = HSLA::new(6, 93, 71, 128);
    ///
    /// assert_eq!(light_salmon, HSLA { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71), a: 128 });
    /// ```
    pub fn new(h: u16, s: u8, l: u8, a: u8) -> HSLA {
        HSLA {
            h: Angle::new(h),
            s: Ratio::from_percentage(s),
            l: Ratio::from_percentage(l),
            a,
        }
    }
}

impl Color for HSLA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self.to_hsl().to_rgb()
    }

    fn to_rgba(self) -> RGBA {
        self.to_hsl().to_rgba()
    }

    fn to_hsl(self) -> HSL {
        HSL::new(
            self.h.degrees(),
            self.s.as_percentage(),
            self.l.as_percentage(),
        )
    }

    fn to_hsla(self) -> HSLA {
        self
    }
}

#[cfg(test)]
mod css_color_tests {
    use {Angle, Color, Ratio, HSL, HSLA, RGB, RGBA};

    #[test]
    fn can_create_color_structs() {
        assert_eq!(
            RGB::new(5, 10, 15),
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
            }
        );
        assert_eq!(
            RGBA::new(5, 10, 15, 255),
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: 255
            }
        );
        assert_eq!(
            HSL::new(6, 93, 71),
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71)
            }
        );
        assert_eq!(
            HSLA::new(6, 93, 71, 255),
            HSLA {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
                a: 255
            }
        );
    }

    #[test]
    fn can_convert_to_rgb_notations() {
        let rgb_color = RGB::new(24, 98, 119);
        let rgba_color = RGBA::new(24, 98, 119, 255);
        let hsl_color = HSL::new(193, 67, 28);
        let hsla_color = HSLA::new(193, 67, 28, 255);

        // HSL to RGB
        assert_eq!(hsl_color.to_rgb(), rgb_color);
        assert_eq!(hsla_color.to_rgb(), rgb_color);

        // RGBA to RGB
        assert_eq!(rgba_color.to_rgb(), rgb_color);

        // TODO: add tests for more conversions.
        // let another_rgb_color = RGB::new(10, 15, 25);
        // let another_hsl_color = HSL::new(73, 57, 54);
        // assert_eq!(hsla_color.to_rgb(), rgb_color);
    }

    #[test]
    fn can_convert_to_rgba_notations() {
        // TODO: add tests for more conversions.
        // let rgb_color = RGB::new(5, 10, 15);
        // let rgba_color = RGBA::new(5, 10, 15, 255);
        // let hsl_color = HSL::new(6, 93, 71);
        // let hsla_color = HSLA::new(6, 93, 71, 255);

        let rgb_color = RGB::new(24, 98, 119);
        let rgba_color = RGBA::new(24, 98, 119, 255);
        let hsl_color = HSL::new(193, 67, 28);
        let hsla_color = HSLA::new(193, 67, 28, 255);

        // HSL to RGBA (currently unimplemented!)
        assert_eq!(hsl_color.to_rgba(), rgba_color);
        assert_eq!(hsla_color.to_rgba(), rgba_color);

        // RGB to RGBA
        assert_eq!(rgb_color.to_rgba(), rgba_color);
    }

    #[test]
    fn can_convert_to_hsl_notations() {
        let rgb_color = RGB::new(172, 95, 82);
        let rgba_color = RGBA::new(172, 95, 82, 255);
        let hsl_color = HSL::new(9, 35, 50);
        let hsla_color = HSLA::new(9, 35, 50, 255);

        // RGB to HSL
        assert_eq!(rgb_color.to_hsl().to_string(), hsl_color.to_string());
        assert_eq!(rgba_color.to_hsl().to_string(), hsl_color.to_string());

        // HSLA to HSL
        assert_eq!(hsla_color.to_hsl().to_string(), hsl_color.to_string());
    }

    #[test]
    fn can_convert_to_hsla_notations() {
        let rgb_color = RGB::new(172, 95, 82);
        let rgba_color = RGBA::new(172, 95, 82, 255);
        let hsl_color = HSL::new(9, 35, 50);
        let hsla_color = HSLA::new(9, 35, 50, 255);

        // RGB to HSLA
        assert_eq!(rgb_color.to_hsla().to_string(), hsla_color.to_string());
        assert_eq!(rgba_color.to_hsla().to_string(), hsla_color.to_string());

        // HSL to HSLA
        assert_eq!(hsl_color.to_hsla().to_string(), hsla_color.to_string());
    }

    #[test]
    fn can_convert_between_grey_colors() {
        let grey_rgb_color = RGB::new(230, 230, 230);
        let grey_rgba_color = RGBA::new(230, 230, 230, 255);
        let grey_hsl_color = HSL::new(0, 0, 90);
        let grey_hsla_color = HSLA::new(0, 0, 90, 255);

        // TO GREY HSL & HSLA
        assert_eq!(grey_rgb_color.to_hsl(), grey_hsl_color);
        assert_eq!(grey_rgb_color.to_hsla(), grey_hsla_color);

        // TO GREY RGB & RGBA
        assert_eq!(grey_hsl_color.to_rgb(), grey_rgb_color);
        assert_eq!(grey_hsl_color.to_rgba(), grey_rgba_color);
    }

    #[test]
    fn can_clone() {
        let rgb_color = RGB::new(5, 10, 15);
        let rgba_color = RGBA::new(5, 10, 15, 255);
        let hsl_color = HSL::new(6, 93, 71);
        let hsla_color = HSLA::new(6, 93, 71, 255);

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
        assert_eq!(hsl_color, hsl_color.clone());
        assert_eq!(hsla_color, hsla_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = RGB::new(172, 95, 82);
        let rgba_color = RGBA::new(172, 95, 82, 255);
        let hsl_color = HSL::new(9, 35, 50);
        let hsla_color = HSLA::new(9, 35, 50, 255);

        let copied_rgb_color = rgb_color;
        let copied_rgba_color = rgba_color;
        let copied_hsl_color = hsl_color;
        let copied_hsla_color = hsla_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
        assert_eq!(hsl_color, copied_hsl_color);
        assert_eq!(hsla_color, copied_hsla_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!("{:?}", RGB::new(5, 10, 15));
        let rgba_value = format!("{:?}", RGBA::new(5, 10, 15, 255));
        let hsl_value = format!("{:?}", HSL::new(6, 93, 71));
        let hsla_value = format!("{:?}", HSLA::new(6, 93, 71, 255));

        assert_eq!(rgb_value, "RGB { r: Ratio(5), g: Ratio(10), b: Ratio(15) }");
        assert_eq!(
            rgba_value,
            "RGBA { r: Ratio(5), g: Ratio(10), b: Ratio(15), a: 255 }"
        );
        assert_eq!(
            hsl_value,
            "HSL { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181) }"
        );
        assert_eq!(
            hsla_value,
            "HSLA { h: Angle { degrees: 6 }, s: Ratio(237), l: Ratio(181), a: 255 }"
        );
    }

    #[test]
    fn can_convert_to_css() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 255);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 255);

        assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1.00)");
        assert_eq!(hsl.to_css(), "hsl(6, 93%, 71%)");
        assert_eq!(hsla.to_css(), "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!("{}", RGB::new(5, 10, 255));
        let printed_rgba = format!("{}", RGBA::new(5, 10, 255, 255));
        let printed_hsl = format!("{}", HSL::new(6, 93, 71));
        let printed_hsla = format!("{}", HSLA::new(6, 93, 71, 255));

        assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        assert_eq!(printed_rgba, "rgba(5, 10, 255, 1.00)");
        assert_eq!(printed_hsl, "hsl(6, 93%, 71%)");
        assert_eq!(printed_hsla, "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 190);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 255);

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));
        assert_eq!("hsl(6, 93%, 71%)".to_owned(), format!("{}", hsl));
        assert_eq!("hsla(6, 93%, 71%, 1.00)".to_owned(), format!("{}", hsla));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = RGB::new(5, 10, 255);
        let rgba = RGBA::new(5, 10, 255, 128);
        let hsl = HSL::new(6, 93, 71);
        let hsla = HSLA::new(6, 93, 71, 128);

        assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        assert_eq!(String::from("rgba(5, 10, 255, 0.50)"), rgba.to_string());
        assert_eq!(String::from("hsl(6, 93%, 71%)"), hsl.to_string());
        assert_eq!(String::from("hsla(6, 93%, 71%, 0.50)"), hsla.to_string());
    }
}

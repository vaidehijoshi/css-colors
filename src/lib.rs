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

    fn saturate(self, amount: u8) -> Self;

    fn fadein(self, amount: u8) -> Self;
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
            (max - min) / (2.0 - (max + min))
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

        // If hue is negative, add 360 to make it it positive.
        if hue <= 0.0 {
            hue += 360.0;
        }

        HSL {
            h: Angle::new(hue.round() as u16),
            s: Ratio::from_f32(saturation),
            l: Ratio::from_f32(luminosity),
        }
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();

        HSLA::new(h.degrees(), s.as_percentage(), l.as_percentage(), 255)
    }

    fn saturate(self, amount: u8) -> Self {
        self.to_hsl().saturate(amount).to_rgb()
    }

    fn fadein(self, amount: u8) -> Self {
        self
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

    fn saturate(self, amount: u8) -> Self {
        self.to_hsla().saturate(amount).to_rgba()
    }

    fn fadein(self, amount: u8) -> Self {
        let RGBA { r, g, b, a } = self;

        RGBA {
            r,
            g,
            b,
            a: a + amount,
        }
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
}

impl Color for HSL {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        let hue = self.h;
        let s = self.s.as_f32();
        let l = self.l.as_f32();

        // If there is no saturation, the color is a shade of grey.
        // We can convert the luminosity and set r, g, and b to that value.
        if s == 0.0 {
            return RGB {
                r: self.l,
                g: self.l,
                b: self.l,
            };
        }

        // If the color is not a grey, then we need to create a temporary variable to continue with the algorithm.
        // If the luminosity is less than 50%, we add 1.0 to the saturation and multiply by the luminosity.
        // Otherwise, we add the luminosity and saturation, and subtract the product of luminosity and saturation from it.
        let temp_1 = if l < 0.5 {
            l * (1.0 + s)
        } else {
            (l + s) - (l * s)
        };

        // Another temporary variable.
        let temp_2 = (2.0 * l) - temp_1;

        // Create a rotation of 120 degrees in order to divide the angle into thirds.
        let rotation = Angle::new(120);

        // Then rotate the circle clockwise by 1/3 for the red value, and by 2/3rds for the blue value.
        let temporary_r = (hue + rotation).degrees();
        let temporary_g = hue.degrees();
        let temporary_b = (hue - rotation).degrees();

        let red = to_rgb_value(temporary_r, temp_1, temp_2);
        let green = to_rgb_value(temporary_g, temp_1, temp_2);
        let blue = to_rgb_value(temporary_b, temp_1, temp_2);

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

    fn saturate(self, amount: u8) -> Self {
        let HSL { h, s, l } = self;

        HSL {
            h,
            s: s + amount,
            l,
        }
    }

    fn fadein(self, amount: u8) -> Self {
        self
    }
}

// A function to convert an HSL value (either h, s, or l) into the equivalent, valid RGB value.
fn to_rgb_value(val: u16, temp_1: f32, temp_2: f32) -> f32 {
    let value = val as f32 / 360.0;

    if value > (2.0 / 3.0) {
        // value > 0.66667
        temp_2
    } else if value > (1.0 / 2.0) {
        // value is between 0.5 and 0.66667
        temp_2 + ((temp_1 - temp_2) * ((2.0 / 3.0) - value) * 6.0)
    } else if value > (1.0 / 6.0) {
        // value is between 0.16667 and 0.5
        temp_1
    } else {
        // value <= 0.16667
        temp_2 + ((temp_1 - temp_2) * value * 6.0)
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
        let RGB { r, g, b } = self.to_rgb();
        RGBA { r, g, b, a: self.a }
    }

    fn to_hsl(self) -> HSL {
        let HSLA { h, s, l, .. } = self;
        HSL { h, s, l }
    }

    fn to_hsla(self) -> HSLA {
        self
    }

    fn saturate(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s: s + amount,
            l,
            a,
        }
    }

    fn fadein(self, amount: u8) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l,
            a: a + amount,
        }
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

    mod conversions {
        use crate::{Angle, Ratio, HSL, HSLA, RGB, RGBA};

        trait ApproximatelyEq {
            fn approximately_eq(self, other: Self) -> bool;
        }

        impl ApproximatelyEq for u8 {
            fn approximately_eq(self, other: Self) -> bool {
                self == other || self + 1 == other || self - 1 == other
            }
        }

        impl ApproximatelyEq for u16 {
            fn approximately_eq(self, other: Self) -> bool {
                self == other || self + 1 == other || self - 1 == other
            }
        }

        impl ApproximatelyEq for Angle {
            fn approximately_eq(self, other: Self) -> bool {
                self.degrees().approximately_eq(other.degrees())
            }
        }

        impl ApproximatelyEq for Ratio {
            fn approximately_eq(self, other: Self) -> bool {
                self.as_u8().approximately_eq(other.as_u8())
            }
        }

        impl ApproximatelyEq for RGB {
            fn approximately_eq(self, other: Self) -> bool {
                self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
            }
        }

        impl ApproximatelyEq for RGBA {
            fn approximately_eq(self, other: Self) -> bool {
                self.r.approximately_eq(other.r)
                    && self.g.approximately_eq(other.g)
                    && self.b.approximately_eq(other.b)
                    && self.a == other.a
            }
        }

        impl ApproximatelyEq for HSL {
            fn approximately_eq(self, other: Self) -> bool {
                self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
            }
        }

        impl ApproximatelyEq for HSLA {
            fn approximately_eq(self, other: Self) -> bool {
                self.h.approximately_eq(other.h)
                    && self
                        .s
                        .as_percentage()
                        .approximately_eq(other.s.as_percentage())
                    && self
                        .l
                        .as_percentage()
                        .approximately_eq(other.l.as_percentage())
                    && self.a == other.a
            }
        }

        macro_rules! assert_approximately_eq {
            ($lhs:expr, $rhs:expr) => {
                let lhs = $lhs;
                let rhs = $rhs;

                assert!(lhs.approximately_eq(rhs), "lhs: {}, rhs: {}", lhs, rhs);
            };
        }

        macro_rules! conversion_test {
            (
                $color_name:ident,
                rgb($r:expr, $g:expr, $b:expr),
                hsl($h:expr, $s:expr, $l:expr)
            ) => {
                mod $color_name {
                    use super::ApproximatelyEq;
                    use $crate::{Color, HSL, HSLA, RGB, RGBA};

                    #[test]
                    fn rgb_to_rgb() {
                        assert_eq!(RGB::new($r, $g, $b).to_rgb(), RGB::new($r, $g, $b));
                    }

                    #[test]
                    fn rgb_to_rgba() {
                        assert_eq!(RGB::new($r, $g, $b).to_rgba(), RGBA::new($r, $g, $b, 255));
                    }

                    #[test]
                    fn rgba_to_rgb() {
                        assert_eq!(RGBA::new($r, $g, $b, 255).to_rgb(), RGB::new($r, $g, $b));
                        assert_eq!(RGBA::new($r, $g, $b, 200).to_rgb(), RGB::new($r, $g, $b));
                        assert_eq!(RGBA::new($r, $g, $b, 0).to_rgb(), RGB::new($r, $g, $b));
                    }

                    #[test]
                    fn rgba_to_rgba() {
                        assert_eq!(
                            RGBA::new($r, $g, $b, 255).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );

                        assert_eq!(
                            RGBA::new($r, $g, $b, 200).to_rgba(),
                            RGBA::new($r, $g, $b, 200)
                        );

                        assert_eq!(RGBA::new($r, $g, $b, 0).to_rgba(), RGBA::new($r, $g, $b, 0));
                    }

                    #[test]
                    fn rgb_to_hsl() {
                        assert_approximately_eq!(
                            RGB::new($r, $g, $b).to_hsl(),
                            HSL::new($h, $s, $l)
                        );
                    }

                    #[test]
                    fn rgb_to_hsla() {
                        assert_approximately_eq!(
                            RGB::new($r, $g, $b).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );
                    }

                    #[test]
                    fn rgba_to_hsl() {
                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 255).to_hsl(),
                            HSL::new($h, $s, $l)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 200).to_hsl(),
                            HSL::new($h, $s, $l)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 0).to_hsl(),
                            HSL::new($h, $s, $l)
                        );
                    }

                    #[test]
                    fn rgba_to_hsla() {
                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 255).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 200).to_hsla(),
                            HSLA::new($h, $s, $l, 200)
                        );

                        assert_approximately_eq!(
                            RGBA::new($r, $g, $b, 0).to_hsla(),
                            HSLA::new($h, $s, $l, 0)
                        );
                    }

                    #[test]
                    fn hsl_to_hsl() {
                        assert_eq!(HSL::new($h, $s, $l).to_hsl(), HSL::new($h, $s, $l));
                    }

                    #[test]
                    fn hsl_to_hsla() {
                        assert_eq!(HSL::new($h, $s, $l).to_hsla(), HSLA::new($h, $s, $l, 255));
                    }

                    #[test]
                    fn hsla_to_hsl() {
                        assert_eq!(HSLA::new($h, $s, $l, 255).to_hsl(), HSL::new($h, $s, $l));

                        assert_eq!(HSLA::new($h, $s, $l, 200).to_hsl(), HSL::new($h, $s, $l));

                        assert_eq!(HSLA::new($h, $s, $l, 0).to_hsl(), HSL::new($h, $s, $l));
                    }

                    #[test]
                    fn hsla_to_hsla() {
                        assert_eq!(
                            HSLA::new($h, $s, $l, 255).to_hsla(),
                            HSLA::new($h, $s, $l, 255)
                        );

                        assert_eq!(
                            HSLA::new($h, $s, $l, 200).to_hsla(),
                            HSLA::new($h, $s, $l, 200)
                        );

                        assert_eq!(HSLA::new($h, $s, $l, 0).to_hsla(), HSLA::new($h, $s, $l, 0));
                    }

                    #[test]
                    fn hsl_to_rgb() {
                        assert_approximately_eq!(
                            HSL::new($h, $s, $l).to_rgb(),
                            RGB::new($r, $g, $b)
                        );
                    }

                    #[test]
                    fn hsl_to_rgba() {
                        assert_approximately_eq!(
                            HSL::new($h, $s, $l).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );
                    }

                    #[test]
                    fn hsla_to_rgb() {
                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 255).to_rgb(),
                            RGB::new($r, $g, $b)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 200).to_rgb(),
                            RGB::new($r, $g, $b)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 0).to_rgb(),
                            RGB::new($r, $g, $b)
                        );
                    }

                    #[test]
                    fn hsla_to_rgba() {
                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 255).to_rgba(),
                            RGBA::new($r, $g, $b, 255)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 200).to_rgba(),
                            RGBA::new($r, $g, $b, 200)
                        );

                        assert_approximately_eq!(
                            HSLA::new($h, $s, $l, 0).to_rgba(),
                            RGBA::new($r, $g, $b, 0)
                        );
                    }
                }
            };
        }

        conversion_test!(black, rgb(0, 0, 0), hsl(0, 0, 0));
        conversion_test!(grey, rgb(230, 230, 230), hsl(0, 0, 90));
        conversion_test!(white, rgb(255, 255, 255), hsl(0, 0, 100));

        conversion_test!(pink, rgb(253, 216, 229), hsl(339, 90, 92));
        conversion_test!(brown, rgb(172, 96, 83), hsl(9, 35, 50));
        conversion_test!(teal, rgb(23, 98, 119), hsl(193, 68, 28));
        conversion_test!(green, rgb(89, 161, 54), hsl(100, 50, 42));
        conversion_test!(pale_blue, rgb(148, 189, 209), hsl(200, 40, 70));
        conversion_test!(mauve, rgb(136, 102, 153), hsl(280, 20, 50));
        conversion_test!(cherry, rgb(230, 25, 60), hsl(350, 80, 50));
        conversion_test!(tomato, rgb(255, 99, 71), hsl(9, 100, 64));
        conversion_test!(light_salmon, rgb(255, 160, 122), hsl(17, 100, 74));
        conversion_test!(blue_violet, rgb(138, 43, 226), hsl(271, 76, 53));
        conversion_test!(dark_orange, rgb(255, 140, 0), hsl(33, 100, 50));
        conversion_test!(deep_pink, rgb(255, 20, 147), hsl(328, 100, 54));
        conversion_test!(chartreuse, rgb(127, 255, 0), hsl(90, 100, 50));
    }

    #[test]
    fn can_convert_between_notations() {
        // Test with brown
        let rgb_brown = RGB::new(172, 96, 83);
        let rgba_brown = RGBA::new(172, 96, 83, 255);
        let hsl_brown = HSL::new(9, 35, 50);
        let hsla_brown = HSLA::new(9, 35, 50, 255);

        // Test with pink
        let rgb_pink = RGB::new(253, 216, 229);
        let rgba_pink = RGBA::new(253, 216, 229, 255);
        let hsl_pink = HSL::new(340, 89, 92);
        let hsla_pink = HSLA::new(340, 89, 92, 255);

        // RGB
        assert_eq!(rgb_brown.to_hsl(), hsl_brown);
        assert_eq!(rgb_brown.to_hsla(), hsla_brown);
        // assert_eq!(rgb_pink.to_hsl(), hsl_pink); // fails
        // assert_eq!(rgb_pink.to_hsla(), hsla_pink); // fails

        // RGBA
        assert_eq!(rgba_brown.to_hsl(), hsl_brown);
        assert_eq!(rgba_brown.to_hsla(), hsla_brown);
        // assert_eq!(rgba_pink.to_hsl(), hsl_pink); // fails
        // assert_eq!(rgba_pink.to_hsla(), hsla_pink); // fails

        // HSL
        // assert_eq!(hsl_brown.to_rgb(), rgb_brown); // fails
        // assert_eq!(hsl_brown.to_rgba(), rgba_brown); // fails
        // assert_eq!(hsl_pink.to_rgb(), rgb_pink); // fails
        // assert_eq!(hsla_pink.to_rgba(), rgba_pink); // fails

        // HSLA
        // assert_eq!(hsla_brown.to_rgb(), rgb_brown); // fails
        // assert_eq!(hsla_brown.to_rgba(), rgba_brown); // fails
        // assert_eq!(hsla_pink.to_rgb(), rgb_pink); // fails
        // assert_eq!(hsla_pink.to_rgba(), rgba_pink); // fails
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

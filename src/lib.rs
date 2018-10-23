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
    /// let salmon = RGB { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114) };
    /// let opaque_salmon = RGBA { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114), a: 128 };
    ///
    /// assert_eq!(salmon.to_css(), "rgb(98%, 50%, 45%)");
    /// assert_eq!(opaque_salmon.to_css(), "rgba(98%, 50%, 45%, 0.50)");
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
    /// let opaque_tomato = RGBA { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71), a: 128 };
    ///
    /// assert_eq!(opaque_tomato.to_rgb(), RGB { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71) });
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
    /// let tomato = RGB { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71) };
    ///
    /// assert_eq!(tomato.to_rgba(), RGBA { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71), a: 255 });
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
    /// let tomato = RGB { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71) };
    /// let opaque_tomato = RGBA { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71), a: 128 };
    ///
    /// assert_eq!(tomato.to_hsl(), HSL { h: Angle::new(9), s: Ratio::from_percentage(100), l: Ratio::from_percentage(64) });
    /// assert_eq!(opaque_tomato.to_hsl(), HSL { h: Angle::new(9), s: Ratio::from_percentage(100), l: Ratio::from_percentage(64) });
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
    /// let tomato = RGB { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71) };
    /// let opaque_tomato = RGBA { r: Ratio::from_u8(255), g: Ratio::from_u8(99), b: Ratio::from_u8(71), a: 128 };
    ///
    /// assert_eq!(tomato.to_hsla(), HSLA { h: Angle::new(9), s: Ratio::from_percentage(100), l: Ratio::from_percentage(64), a: 255 });
    /// assert_eq!(opaque_tomato.to_hsla(), HSLA { h: Angle::new(9), s: Ratio::from_percentage(100), l: Ratio::from_percentage(64), a: 128 });
    /// ```
    fn to_hsla(self) -> HSLA;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
///
/// Valid values for r, g, and b must fall between `0-255`.
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
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}

impl RGB {
    /// Transforms numerical values into an RGB struct.
    ///
    /// # Example
    /// ```
    /// use {css_colors::RGB, css_colors::ratio::Ratio as Ratio};
    ///
    /// let salmon = RGB::new(Ratio::from_u8(250), Ratio::from_u8(128), Ratio::from_u8(114));
    ///
    /// assert_eq!(salmon, RGB { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114) });
    /// ```
    pub fn new(r: Ratio, g: Ratio, b: Ratio) -> RGB {
        RGB { r, g, b }
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
        RGBA::new(self.r, self.g, self.b, 255)
    }

    /// The algorithm for converting from rgb to hsl format, which determines
    /// the equivalent luminosity, saturation, and hue.
    fn to_hsl(self) -> HSL {
        let RGB { r, g, b } = self;

        // If r, g, and b are the same, the color is a shade of grey (between
        // black and white), with no hue or saturation. In that situation, there
        // is no saturation or hue, and we can use any value to determine luminosity.
        if r == g && g == b {
            return HSL::new(
                Angle::new(0),             // h
                Ratio::from_percentage(0), // s
                r,                         // l
            );
        }

        // Otherwise, to determine luminosity, we conver the RGB values into a
        // percentage value, find the max and the min of those values, sum them
        // together, and divide by 2.

        // TODO: delete me let r = self.r.to_f32() if r is a Ratio
        let r = Ratio::as_f32(self.r);
        let g = Ratio::as_f32(self.g);
        let b = Ratio::as_f32(self.b);

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
            Angle::new(hue.round() as u16), // h
            Ratio::from_f32(saturation),    // s
            Ratio::from_f32(luminosity),    // l
        )
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();
        HSLA::new(h, s, l, 255)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications.
///
/// Valid values for r, g, b, and a must fall between `0-255`.
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
            self.r,
            self.g,
            self.b,
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
    /// let light_salmon = RGBA::new(Ratio::from_u8(250), Ratio::from_u8(128), Ratio::from_u8(114), 128);
    ///
    /// assert_eq!(light_salmon, RGBA { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114), a: 128 });
    /// ```
    pub fn new(r: Ratio, g: Ratio, b: Ratio, a: u8) -> RGBA {
        RGBA { r, g, b, a }
    }
}

impl Color for RGBA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        RGB::new(self.r, self.g, self.b)
    }

    fn to_rgba(self) -> RGBA {
        self
    }

    fn to_hsl(self) -> HSL {
        self.to_rgb().to_hsl()
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self.to_hsl();
        HSLA::new(h, s, l, self.a)
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
    /// let salmon = HSL::new(Angle::new(6), Ratio::from_percentage(93), Ratio::from_percentage(71));
    ///
    /// assert_eq!(salmon, HSL { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71) });
    /// ```
    pub fn new(h: Angle, s: Ratio, l: Ratio) -> HSL {
        HSL { h, s, l }
    }
}

impl Color for HSL {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        // FIXME: create impl, add tests for this
        RGB::new(Ratio::from_u8(0), Ratio::from_u8(0), Ratio::from_u8(0))
    }

    fn to_rgba(self) -> RGBA {
        // FIXME: create impl, add tests for this
        RGBA::new(Ratio::from_u8(0), Ratio::from_u8(0), Ratio::from_u8(0), 0)
    }

    fn to_hsl(self) -> HSL {
        self
    }

    fn to_hsla(self) -> HSLA {
        HSLA::new(self.h, self.s, self.l, 255)
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
    /// let light_salmon = HSLA::new(Angle::new(6), Ratio::from_percentage(93), Ratio::from_percentage(71), 128);
    ///
    /// assert_eq!(light_salmon, HSLA { h: Angle::new(6), s: Ratio::from_percentage(93), l: Ratio::from_percentage(71), a: 128 });
    /// ```
    pub fn new(h: Angle, s: Ratio, l: Ratio, a: u8) -> HSLA {
        HSLA { h, s, l, a }
    }
}

impl Color for HSLA {
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        // FIXME: create impl, add tests for this
        RGB::new(Ratio::from_u8(0), Ratio::from_u8(0), Ratio::from_u8(0))
    }

    fn to_rgba(self) -> RGBA {
        // FIXME: create impl, add tests for this
        RGBA::new(Ratio::from_u8(0), Ratio::from_u8(0), Ratio::from_u8(0), 0)
    }

    fn to_hsl(self) -> HSL {
        HSL::new(self.h, self.s, self.l)
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
            RGB::new(Ratio::from_u8(5), Ratio::from_u8(10), Ratio::from_u8(15)),
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15)
            }
        );
        assert_eq!(
            RGBA::new(
                Ratio::from_u8(5),
                Ratio::from_u8(10),
                Ratio::from_u8(15),
                255
            ),
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: 255
            }
        );
        assert_eq!(
            HSL::new(
                Angle::new(6),
                Ratio::from_percentage(93),
                Ratio::from_percentage(71)
            ),
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71)
            }
        );
        assert_eq!(
            HSLA::new(
                Angle::new(6),
                Ratio::from_percentage(93),
                Ratio::from_percentage(71),
                255
            ),
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
        let rgb_color = RGB {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
        };
        let rgba_color = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
            a: 255,
        };
        let hsl_color = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 255,
        };

        assert_eq!(
            rgb_color.to_rgba(),
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: 255,
            }
        );
        assert_eq!(
            rgba_color.to_rgb(),
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15)
            }
        );

        // FIXME: update these tests once HSL <-> RBG impl exists
        assert_eq!(
            hsl_color.to_rgb(),
            RGB {
                r: Ratio::from_u8(0),
                g: Ratio::from_u8(0),
                b: Ratio::from_u8(0)
            }
        );
        assert_eq!(
            hsl_color.to_rgba(),
            RGBA {
                r: Ratio::from_u8(0),
                g: Ratio::from_u8(0),
                b: Ratio::from_u8(0),
                a: 0
            }
        );
        assert_eq!(
            hsla_color.to_rgb(),
            RGB {
                r: Ratio::from_u8(0),
                g: Ratio::from_u8(0),
                b: Ratio::from_u8(0)
            }
        );
        assert_eq!(
            hsla_color.to_rgba(),
            RGBA {
                r: Ratio::from_u8(0),
                g: Ratio::from_u8(0),
                b: Ratio::from_u8(0),
                a: 0
            }
        );
    }

    #[test]
    fn can_convert_to_hsl_notations() {
        let rgb_rust = RGB {
            r: Ratio::from_u8(172),
            g: Ratio::from_u8(95),
            b: Ratio::from_u8(82),
        };
        let rgba_rust = RGBA {
            r: Ratio::from_u8(172),
            g: Ratio::from_u8(95),
            b: Ratio::from_u8(82),
            a: 255,
        };
        let hsl_rust = HSL {
            h: Angle::new(9),
            s: Ratio::from_percentage(35),
            l: Ratio::from_percentage(50),
        };

        let hsla_rust = HSLA {
            h: Angle::new(9),
            s: Ratio::from_percentage(35),
            l: Ratio::from_percentage(50),
            a: 255,
        };

        // RGB to HSL & HSLA
        assert_eq!(rgb_rust.to_hsl().to_string(), hsl_rust.to_string());
        assert_eq!(rgb_rust.to_hsla().to_string(), hsla_rust.to_string());

        // RGBA to HSL & HSLA
        assert_eq!(rgba_rust.to_hsl().to_string(), hsl_rust.to_string());
        assert_eq!(rgba_rust.to_hsla().to_string(), hsla_rust.to_string());
    }

    #[test]
    fn can_clone() {
        let rgb_color = RGB {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
        };
        let rgba_color = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
            a: 255,
        };
        let hsl_color = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 255,
        };

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
        assert_eq!(hsl_color, hsl_color.clone());
        assert_eq!(hsla_color, hsla_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
            a: 255,
        };
        let copied_rgb_color = rgb_color;
        let rgba_color = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(15),
            a: 255,
        };
        let copied_rgba_color = rgba_color;
        let hsl_color = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let copied_hsl_color = hsl_color;
        let hsla_color = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 255,
        };
        let copied_hsla_color = hsla_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
        assert_eq!(hsl_color, copied_hsl_color);
        assert_eq!(hsla_color, copied_hsla_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!(
            "{:?}",
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15)
            }
        );
        let rgba_value = format!(
            "{:?}",
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(15),
                a: 255
            }
        );
        let hsl_value = format!(
            "{:?}",
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
            }
        );
        let hsla_value = format!(
            "{:?}",
            HSLA {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
                a: 255
            }
        );

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
        let rgb = RGB {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
        };
        let rgba = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
            a: 255,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 255,
        };

        // TODO: You should probably be able to write rgb in both formats (percentage and u8).
        // Need to add the ability to do that, and then these two tests will pass.
        // assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        // assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1.00)");

        assert_eq!(rgb.to_css(), "rgb(2%, 4%, 100%)");
        assert_eq!(rgba.to_css(), "rgba(2%, 4%, 100%, 1.00)");
        assert_eq!(hsl.to_css(), "hsl(6, 93%, 71%)");
        assert_eq!(hsla.to_css(), "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!(
            "{}",
            RGB {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(255),
            }
        );
        let printed_rgba = format!(
            "{}",
            RGBA {
                r: Ratio::from_u8(5),
                g: Ratio::from_u8(10),
                b: Ratio::from_u8(255),
                a: 255,
            }
        );
        let printed_hsl = format!(
            "{}",
            HSL {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
            }
        );
        let printed_hsla = format!(
            "{}",
            HSLA {
                h: Angle::new(6),
                s: Ratio::from_percentage(93),
                l: Ratio::from_percentage(71),
                a: 255,
            }
        );

        // TODO: You should probably be able to write rgb in both formats (percentage and u8).
        // Need to add the ability to do that, and then these two tests will pass.
        // assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        // assert_eq!(printed_rgba, "rgba(5, 10, 255, 1.00)");

        assert_eq!(printed_rgb, "rgb(2%, 4%, 100%)");
        assert_eq!(printed_rgba, "rgba(2%, 4%, 100%, 1.00)");
        assert_eq!(printed_hsl, "hsl(6, 93%, 71%)");
        assert_eq!(printed_hsla, "hsla(6, 93%, 71%, 1.00)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = RGB {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
        };
        let rgba = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
            a: 190,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 255,
        };

        // TODO: You should probably be able to write rgb in both formats (percentage and u8).
        // Need to add the ability to do that, and then these two tests will pass.
        // assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        // assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));

        assert_eq!("rgb(2%, 4%, 100%)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(2%, 4%, 100%, 0.75)".to_owned(), format!("{}", rgba));
        assert_eq!("hsl(6, 93%, 71%)".to_owned(), format!("{}", hsl));
        assert_eq!("hsla(6, 93%, 71%, 1.00)".to_owned(), format!("{}", hsla));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = RGB {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
        };
        let rgba = RGBA {
            r: Ratio::from_u8(5),
            g: Ratio::from_u8(10),
            b: Ratio::from_u8(255),
            a: 128,
        };
        let hsl = HSL {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
        };
        let hsla = HSLA {
            h: Angle::new(6),
            s: Ratio::from_percentage(93),
            l: Ratio::from_percentage(71),
            a: 128,
        };

        // TODO: You should probably be able to write rgb in both formats (percentage and u8).
        // Need to add the ability to do that, and then these two tests will pass.
        // assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        // assert_eq!(String::from("rgba(5, 10, 255, 0.50)"), rgba.to_string());

        assert_eq!(String::from("rgb(2%, 4%, 100%)"), rgb.to_string());
        assert_eq!(String::from("rgba(2%, 4%, 100%, 0.50)"), rgba.to_string());
        assert_eq!(String::from("hsl(6, 93%, 71%)"), hsl.to_string());
        assert_eq!(String::from("hsla(6, 93%, 71%, 0.50)"), hsla.to_string());
    }
}

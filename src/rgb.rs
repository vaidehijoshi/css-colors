use super::{deg, percent, Angle, Color, Ratio, HSL, HSLA};
use std::fmt;

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
    /// use css_colors::{RGB, Ratio};
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
    type Alpha = RGBA;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self
    }

    fn to_rgba(self) -> RGBA {
        let RGB { r, g, b } = self;

        RGBA {
            r,
            g,
            b,
            a: percent(100),
        }
    }

    /// The algorithm for converting from rgb to hsl format, which determines
    /// the equivalent luminosity, saturation, and hue.
    fn to_hsl(self) -> HSL {
        self.to_rgba().to_hsl()
    }

    fn to_hsla(self) -> HSLA {
        self.to_rgba().to_hsla()
    }

    fn saturate(self, amount: Ratio) -> Self {
        self.to_rgba().saturate(amount).to_rgb()
    }

    fn desaturate(self, amount: Ratio) -> Self {
        self.to_rgba().desaturate(amount).to_rgb()
    }

    fn lighten(self, amount: Ratio) -> Self {
        self.to_rgba().lighten(amount).to_rgb()
    }

    fn darken(self, amount: Ratio) -> Self {
        self.to_rgba().darken(amount).to_rgb()
    }

    fn fadein(self, _amount: Ratio) -> Self {
        self
    }

    fn fadeout(self, _amount: Ratio) -> Self {
        self
    }

    fn fade(self, amount: Ratio) -> RGBA {
        self.to_rgba().fade(amount)
    }

    fn spin(self, amount: Angle) -> Self {
        self.to_rgba().spin(amount).to_rgb()
    }

    fn mix<T: Color>(self, other: T, weight: Ratio) -> RGBA {
        self.to_rgba().mix(other, weight)
    }

    fn tint(self, weight: Ratio) -> Self {
        self.to_rgba().tint(weight).to_rgb()
    }

    fn shade(self, weight: Ratio) -> Self {
        self.to_rgba().shade(weight).to_rgb()
    }

    fn greyscale(self) -> Self {
        self.to_rgba().greyscale().to_rgb()
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
    pub a: Ratio,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.02})",
            self.r.as_u8(),
            self.g.as_u8(),
            self.b.as_u8(),
            self.a.as_f32()
        )
    }
}

impl RGBA {
    /// Transforms numerical values into an RGBA struct.
    ///
    /// # Example
    /// ```
    /// use css_colors::{RGBA, Ratio};
    ///
    /// let light_salmon = RGBA::new(250, 128, 114, 128);
    ///
    /// assert_eq!(light_salmon, RGBA { r: Ratio::from_u8(250), g: Ratio::from_u8(128), b: Ratio::from_u8(114), a: Ratio::from_u8(128) });
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBA {
        RGBA {
            r: Ratio::from_u8(r),
            g: Ratio::from_u8(g),
            b: Ratio::from_u8(b),
            a: Ratio::from_u8(a),
        }
    }
}

impl Color for RGBA {
    type Alpha = Self;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        let RGBA { r, g, b, .. } = self;
        RGB { r, g, b }
    }

    fn to_rgba(self) -> RGBA {
        self
    }

    fn to_hsl(self) -> HSL {
        self.to_hsla().to_hsl()
    }

    fn to_hsla(self) -> HSLA {
        let RGBA { r, g, b, a } = self;

        // If r, g, and b are the same, the color is a shade of grey (between
        // black and white), with no hue or saturation. In that situation, there
        // is no saturation or hue, and we can use any value to determine luminosity.
        if r == g && g == b {
            return HSLA {
                h: deg(0),
                s: percent(0),
                l: r,
                a,
            };
        }

        // Otherwise, to determine luminosity, we conver the RGB values into a
        // percentage value, find the max and the min of those values, sum them
        // together, and divide by 2.
        let r = self.r.as_f32();
        let g = self.g.as_f32();
        let b = self.b.as_f32();

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
        // adding 120 or 240 deg to account for the degrees on the color wheel, and
        // then dividing that by the difference between the max and the min values.
        // Finally, we multiply the hue value by 60 to convert it to degrees on
        // the color wheel, accounting for negative hues as well.
        let hue = if max == r {
            60.0 * (g - b) / (max - min)
        } else if max == g {
            120.0 + 60.0 * (b - r) / (max - min)
        } else {
            240.0 + 60.0 * (r - g) / (max - min)
        };

        HSLA {
            h: deg(hue.round() as i32),
            s: Ratio::from_f32(saturation),
            l: Ratio::from_f32(luminosity),
            a,
        }
    }

    fn saturate(self, amount: Ratio) -> Self {
        self.to_hsla().saturate(amount).to_rgba()
    }

    fn desaturate(self, amount: Ratio) -> Self {
        self.to_hsla().desaturate(amount).to_rgba()
    }

    fn lighten(self, amount: Ratio) -> Self {
        self.to_hsla().lighten(amount).to_rgba()
    }

    fn darken(self, amount: Ratio) -> Self {
        self.to_hsla().darken(amount).to_rgba()
    }

    fn fadein(self, amount: Ratio) -> Self {
        self.fade(self.a + amount)
    }

    fn fadeout(self, amount: Ratio) -> Self {
        self.fade(self.a - amount)
    }

    fn fade(self, amount: Ratio) -> Self {
        let RGBA { r, g, b, .. } = self;
        RGBA { r, g, b, a: amount }
    }

    fn spin(self, amount: Angle) -> Self {
        self.to_hsla().spin(amount).to_rgba()
    }

    // This algorithm takes into account both the user-provided weight (w) and
    // the difference between the alpha values of the two colors (a) to determine
    // the weighted average of the two colors.
    // Taken from Sass's implementation (http://sass-lang.com/documentation/Sass/Script/Functions.html#mix-instance_method)
    fn mix<T: Color>(self, other: T, weight: Ratio) -> Self {
        let RGBA {
            r: r_lhs,
            g: g_lhs,
            b: b_lhs,
            a: a_lhs,
        } = self;

        let RGBA {
            r: r_rhs,
            g: g_rhs,
            b: b_rhs,
            a: a_rhs,
        } = other.to_rgba();

        // Convert weight into a decimal, and then scale it so that it falls between a range of [-1, 1].
        let w = (weight.as_f32() * 2.0) - 1.0;

        // Find the difference between the left and right side's alphas (somewhere between [-1, 1]).
        let a = a_lhs.as_f32() - a_rhs.as_f32();

        // Find the combined rgb_weight, taking into account the user's passed-in weight and alpha (range of [-1, 1]).
        let rgb_weight = if w * a == -1.0 {
            w
        } else {
            (w + a) / (1.0 + w * a)
        };

        // Find the combined rgb weight, scaling it to fall in a range bewtween [0, 1].
        let rgb_weight = (rgb_weight + 1.0) / 2.0;

        // Convert left and right side's weights into Ratios.
        let rgb_weight_lhs = Ratio::from_f32(rgb_weight);
        let rgb_weight_rhs = Ratio::from_f32(1.0) - rgb_weight_lhs;

        let alpha_weight_lhs = weight;
        let alpha_weight_rhs = Ratio::from_f32(1.0) - alpha_weight_lhs;

        RGBA {
            r: (r_lhs * rgb_weight_lhs) + (r_rhs * rgb_weight_rhs),
            g: (g_lhs * rgb_weight_lhs) + (g_rhs * rgb_weight_rhs),
            b: (b_lhs * rgb_weight_lhs) + (b_rhs * rgb_weight_rhs),
            a: (a_lhs * alpha_weight_lhs) + (a_rhs * alpha_weight_rhs),
        }
    }

    fn tint(self, weight: Ratio) -> Self {
        self.mix(RGB::new(255, 255, 255), weight)
    }

    fn shade(self, weight: Ratio) -> Self {
        self.mix(RGB::new(0, 0, 0), weight)
    }

    fn greyscale(self) -> Self {
        self.to_hsla().greyscale().to_rgba()
    }
}

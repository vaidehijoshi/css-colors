use super::{deg, percent, Angle, Color, Ratio, RGB, RGBA};
use std::fmt;

/// Constructs a HSL Color from numerical values, similar to the
/// [`hsl` function](css-hsl) in CSS.
///
/// The hue component is expressed in degrees. Values outside of
/// the 0-359° range will be normalized accordingly. The saturation
/// and lightness components are expressed in percentages. Values
/// outside of the 0-100% range will cause a panic.
///
/// # Example
/// ```
/// use css_colors::{Color, hsl};
///
/// let salmon = hsl(6, 93, 71);
///
/// assert_eq!(salmon.to_css(), "hsl(6, 93%, 71%)");
/// ```
///
/// [css-hsl]: https://www.w3.org/TR/css-color-3/#hsl-color
pub fn hsl(h: i32, s: u8, l: u8) -> HSL {
    HSL {
        h: deg(h),
        s: percent(s),
        l: percent(l),
    }
}

/// Constructs a HSLA Color from numerical values, similar to the
/// [`hsla` function](css-hsla) in CSS.
///
/// The hue component is expressed in degrees. Values outside of
/// the 0-359° range will be normalized accordingly. The saturation
/// and lightness components are expressed in percentages. Values
/// outside of the 0-100% range will cause a panic. The alpha value
/// is expressed as a float. Values outside of the 0.0-1.0 range will
/// cause a panic.
///
/// # Example
/// ```
/// use css_colors::{Color, hsla};
///
/// let salmon = hsla(6, 93, 71, 0.50);
///
/// assert_eq!(salmon.to_css(), "hsla(6, 93%, 71%, 0.50)");
/// ```
///
/// [css-hsla]: https://www.w3.org/TR/css-color-3/#hsla-color
pub fn hsla(h: i32, s: u8, l: u8, a: f32) -> HSLA {
    HSLA {
        h: deg(h),
        s: percent(s),
        l: percent(l),
        a: Ratio::from_f32(a),
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

impl Color for HSL {
    type Alpha = HSLA;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self.to_hsla().to_rgb()
    }

    fn to_rgba(self) -> RGBA {
        self.to_hsla().to_rgba()
    }

    fn to_hsl(self) -> HSL {
        self
    }

    fn to_hsla(self) -> HSLA {
        let HSL { h, s, l } = self;

        HSLA {
            h,
            s,
            l,
            a: percent(100),
        }
    }

    fn saturate(self, amount: Ratio) -> Self {
        self.to_hsla().saturate(amount).to_hsl()
    }

    fn desaturate(self, amount: Ratio) -> Self {
        self.to_hsla().desaturate(amount).to_hsl()
    }

    fn lighten(self, amount: Ratio) -> Self {
        self.to_hsla().lighten(amount).to_hsl()
    }

    fn darken(self, amount: Ratio) -> Self {
        self.to_hsla().darken(amount).to_hsl()
    }

    fn fadein(self, _amount: Ratio) -> Self {
        self
    }

    fn fadeout(self, _amount: Ratio) -> Self {
        self
    }

    fn fade(self, amount: Ratio) -> Self::Alpha {
        self.to_hsla().fade(amount)
    }

    fn spin(self, amount: Angle) -> Self {
        self.to_hsla().spin(amount).to_hsl()
    }

    fn mix<T: Color>(self, other: T, weight: Ratio) -> Self::Alpha {
        self.to_hsla().mix(other, weight)
    }

    fn tint(self, weight: Ratio) -> Self {
        self.to_hsla().tint(weight).to_hsl()
    }

    fn shade(self, weight: Ratio) -> Self {
        self.to_hsla().shade(weight).to_hsl()
    }

    fn greyscale(self) -> Self {
        self.to_hsla().greyscale().to_hsl()
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
    pub a: Ratio,
}

impl fmt::Display for HSLA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "hsla({}, {}, {}, {:.02})",
            self.h.degrees(),
            self.s,
            self.l,
            self.a.as_f32()
        )
    }
}

impl Color for HSLA {
    type Alpha = Self;

    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self.to_rgba().to_rgb()
    }

    fn to_rgba(self) -> RGBA {
        let HSLA { h, s, l, a } = self;

        // If there is no saturation, the color is a shade of grey.
        // We can convert the luminosity and set r, g, and b to that value.
        if s == percent(0) {
            return RGBA {
                r: l,
                g: l,
                b: l,
                a,
            };
        }

        let s = s.as_f32();
        let l = l.as_f32();

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
        let temporary_r = (h + rotation).degrees();
        let temporary_g = h.degrees();
        let temporary_b = (h - rotation).degrees();

        let red = to_rgb_value(temporary_r, temp_1, temp_2);
        let green = to_rgb_value(temporary_g, temp_1, temp_2);
        let blue = to_rgb_value(temporary_b, temp_1, temp_2);

        RGBA {
            r: Ratio::from_f32(red),
            g: Ratio::from_f32(green),
            b: Ratio::from_f32(blue),
            a,
        }
    }

    fn to_hsl(self) -> HSL {
        let HSLA { h, s, l, .. } = self;
        HSL { h, s, l }
    }

    fn to_hsla(self) -> HSLA {
        self
    }

    fn saturate(self, amount: Ratio) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s: s + amount,
            l,
            a,
        }
    }

    fn desaturate(self, amount: Ratio) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s: s - amount,
            l,
            a,
        }
    }

    fn lighten(self, amount: Ratio) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l: l + amount,
            a,
        }
    }

    fn darken(self, amount: Ratio) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h,
            s,
            l: l - amount,
            a,
        }
    }

    fn fadein(self, amount: Ratio) -> Self {
        self.fade(self.a + amount)
    }

    fn fadeout(self, amount: Ratio) -> Self {
        self.fade(self.a - amount)
    }

    fn fade(self, amount: Ratio) -> Self::Alpha {
        let HSLA { h, s, l, .. } = self;
        HSLA { h, s, l, a: amount }
    }

    fn spin(self, amount: Angle) -> Self {
        let HSLA { h, s, l, a } = self;

        HSLA {
            h: h + amount,
            s,
            l,
            a,
        }
    }

    fn mix<T: Color>(self, other: T, weight: Ratio) -> Self::Alpha {
        self.to_rgba().mix(other, weight).to_hsla()
    }

    fn tint(self, weight: Ratio) -> Self {
        self.to_rgba().tint(weight).to_hsla()
    }

    fn shade(self, weight: Ratio) -> Self {
        self.to_rgba().shade(weight).to_hsla()
    }

    fn greyscale(self) -> Self {
        let HSLA { h, l, a, .. } = self;

        HSLA {
            h,
            s: percent(0),
            l,
            a,
        }
    }
}

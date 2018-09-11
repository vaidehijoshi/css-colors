use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications, with valid percentages ranging between 0-1.
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgba-color).
///
pub struct RGBA {
    // red between 0-255
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,

    // alpha
    pub a: f32,
}

/// Converts a set of RGBA values into valid CSS.
///
/// # Examples
///
/// ```
/// let salmon = css_colors::RGBA { r: 250, g: 128, b: 114, a: 1.0 };
///
/// assert_eq!("rgb(250, 128, 114, 1)", salmon.to_css());
/// ```
impl RGBA {
    pub fn to_css(&self) -> String {
        format!("rgb({red}, {green}, {blue}, {alpha})", red=self.r, green=self.g, blue=self.b, alpha=self.a)
    }
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}


#[cfg(test)]
mod rgba_tests {
    use ::RGBA;

    #[test]
    fn can_clone() {
        let color = RGBA { r: 5, g: 10, b: 15, a: 1.0 };

        assert_eq!(color, color.clone());
    }

    #[test]
    fn can_copy() {
        let color = RGBA { r: 5, g: 10, b: 15, a: 1.0 };
        let copied_color = color;

        assert_eq!(color, copied_color);
    }

    #[test]
    fn can_debug() {
        let color = RGBA { r: 5, g: 10, b: 15, a: 1.0 };
        let value = format!("{:?}", color);

        assert_eq!(value, "RGBA { r: 5, g: 10, b: 15, a: 1.0 }");
    }

    #[test]
    fn can_convert_to_css() {
        let color = RGBA { r: 5, g: 10, b: 255, a: 1.0 };

        assert_eq!(color.to_css(), "rgb(5, 10, 255, 1)");
    }

    #[test]
    fn can_print_in_css() {
        let color = RGBA { r: 5, g: 10, b: 255, a: 1.0 };
        let printed = format!("{}", color);

        assert_eq!(printed, "rgb(5, 10, 255, 1)");
    }

    #[test]
    fn can_be_displayed() {
        let color = RGBA { r: 5, g: 10, b: 255, a: 0.75 };

        assert_eq!("rgb(5, 10, 255, 0.75)".to_owned(), format!("{}", color));
    }

    #[test]
    fn can_be_stringified() {
        let color = RGBA { r: 5, g: 10, b: 255, a: 0.5 };
        let color_string = String::from("rgb(5, 10, 255, 0.5)");

        assert_eq!(color_string, color.to_string());
    }
}

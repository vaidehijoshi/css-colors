use std::fmt;

/// TODO...
pub trait Color {
    // TODO...
    fn to_css(self) -> String;

    // TODO...
    fn to_rgb(self) -> RGB;

    // TODO...
    fn to_rgba(self) -> RGBA;
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
///
/// Valid values for r, g, and b must fall between `0-255`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgb-color).
pub struct RGB {
    // red
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
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
    /// use css_colors::RGB;
    ///
    /// let salmon = RGB::new(250, 128, 114);
    ///
    /// assert_eq!(salmon, RGB { r: 250, g: 128, b: 114 });
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r, g, b }
    }
}

impl Color for RGB {
    /// Converts a set of RGBA values into valid CSS.
    ///
    /// # Example
    /// ```
    /// use css_colors::{Color, RGB};
    ///
    /// let salmon = RGB { r: 250, g: 128, b: 114 };
    ///
    /// assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
    /// ```
    fn to_css(self) -> String {
        self.to_string()
    }

    fn to_rgb(self) -> RGB {
        self
    }

    /// Converts a set of numerical RGB values into an RGBA struct.
    /// Defaults to an alpha value of 1.0.
    ///
    /// # Example
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGB { r: 255, g: 99, b: 71 };
    ///
    /// assert_eq!(tomato.to_rgba(), RGBA { r: 255, g: 99, b: 71, a: 1.0 });
    /// ```
    fn to_rgba(self) -> RGBA {
        RGBA::new(self.r, self.g, self.b, 1.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// A struct to represent how much red, green, and blue should be added to create a color.
/// Also handles alpha specifications.
///
/// Valid values for r, g, and b must fall between `0-255`.
/// Valid values for alpha specifications must be a percentage ranging between `0.0 - 1.0`.
///
/// For more, see the [CSS Color Spec](https://www.w3.org/TR/2018/REC-css-color-3-20180619/#rgba-color).
pub struct RGBA {
    // red
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,

    // alpha
    pub a: f32,
}

impl fmt::Display for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl RGBA {
    /// Transforms numerical values into an RGBA struct.
    ///
    /// # Example
    /// ```
    /// use css_colors::RGBA;
    ///
    /// let light_salmon = RGBA::new(250, 128, 114, 0.5);
    ///
    /// assert_eq!(light_salmon, RGBA { r: 250, g: 128, b: 114, a: 0.5 });
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> RGBA {
        RGBA { r, g, b, a }
    }
}

impl Color for RGBA {
    /// Converts a set of RGBA values into valid CSS.
    ///
    /// # Example
    /// ```
    /// use css_colors::{Color, RGBA};
    ///
    /// let salmon = RGBA { r: 250, g: 128, b: 114, a: 1.0 };
    ///
    /// assert_eq!(salmon.to_css(), "rgba(250, 128, 114, 1)");
    /// ```
    fn to_css(self) -> String {
        self.to_string()
    }

    /// Converts a set of numerical RGBA values into an RGB struct.
    /// Ignores any alpha value supplemented.
    ///
    /// # Example
    /// ```
    /// use css_colors::{Color, RGB, RGBA};
    ///
    /// let tomato = RGBA { r: 255, g: 99, b: 71, a: 1.0 };
    ///
    /// assert_eq!(tomato.to_rgb(), RGB { r: 255, g: 99, b: 71 });
    /// ```
    fn to_rgb(self) -> RGB {
        RGB::new(self.r, self.g, self.b)
    }

    fn to_rgba(self) -> RGBA {
        self
    }
}

#[cfg(test)]
mod css_color_tests {
    use {Color, RGB, RGBA};

    #[test]
    fn can_create_color_structs() {
        assert_eq!(RGB::new(5, 10, 15), RGB { r: 5, g: 10, b: 15 });
        assert_eq!(
            RGBA::new(5, 10, 15, 1.0),
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 1.0
            }
        );
    }

    #[test]
    fn can_convert_between_rgb_notations() {
        let rgb_color = RGB { r: 5, g: 10, b: 15 };
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 1.0,
        };

        assert_eq!(
            rgb_color.to_rgba(),
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 1.0
            }
        );
        assert_eq!(rgba_color.to_rgb(), RGB { r: 5, g: 10, b: 15 });
    }

    #[test]
    fn can_clone() {
        let rgb_color = RGB { r: 5, g: 10, b: 15 };
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 1.0,
        };

        assert_eq!(rgb_color, rgb_color.clone());
        assert_eq!(rgba_color, rgba_color.clone());
    }

    #[test]
    fn can_copy() {
        let rgb_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 1.0,
        };
        let copied_rgb_color = rgb_color;
        let rgba_color = RGBA {
            r: 5,
            g: 10,
            b: 15,
            a: 1.0,
        };
        let copied_rgba_color = rgba_color;

        assert_eq!(rgb_color, copied_rgb_color);
        assert_eq!(rgba_color, copied_rgba_color);
    }

    #[test]
    fn can_debug() {
        let rgb_value = format!("{:?}", RGB { r: 5, g: 10, b: 15 });
        let rgba_value = format!(
            "{:?}",
            RGBA {
                r: 5,
                g: 10,
                b: 15,
                a: 1.0
            }
        );

        assert_eq!(rgb_value, "RGB { r: 5, g: 10, b: 15 }");
        assert_eq!(rgba_value, "RGBA { r: 5, g: 10, b: 15, a: 1.0 }");
    }

    #[test]
    fn can_convert_to_css() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 1.0,
        };

        assert_eq!(rgb.to_css(), "rgb(5, 10, 255)");
        assert_eq!(rgba.to_css(), "rgba(5, 10, 255, 1)");
    }

    #[test]
    fn can_print_in_css() {
        let printed_rgb = format!(
            "{}",
            RGB {
                r: 5,
                g: 10,
                b: 255
            }
        );
        let printed_rgba = format!(
            "{}",
            RGBA {
                r: 5,
                g: 10,
                b: 255,
                a: 1.0
            }
        );

        assert_eq!(printed_rgb, "rgb(5, 10, 255)");
        assert_eq!(printed_rgba, "rgba(5, 10, 255, 1)");
    }

    #[test]
    fn can_be_displayed() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 0.75,
        };

        assert_eq!("rgb(5, 10, 255)".to_owned(), format!("{}", rgb));
        assert_eq!("rgba(5, 10, 255, 0.75)".to_owned(), format!("{}", rgba));
    }

    #[test]
    fn can_be_stringified() {
        let rgb = RGB {
            r: 5,
            g: 10,
            b: 255,
        };
        let rgba = RGBA {
            r: 5,
            g: 10,
            b: 255,
            a: 0.5,
        };

        assert_eq!(String::from("rgb(5, 10, 255)"), rgb.to_string());
        assert_eq!(String::from("rgba(5, 10, 255, 0.5)"), rgba.to_string());
    }
}

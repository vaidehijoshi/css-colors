use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB {
    // red between 0-255
    pub r: u8,

    // green
    pub g: u8,

    // blue
    pub b: u8,
}

impl RGB {
    pub fn to_css(&self) -> String {
        format!("rgb({red}, {green}, {blue})", red=self.r, green=self.g, blue=self.b)
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.r, self.g, self.b)
    }
}


#[cfg(test)]
mod rgb_tests {
    use ::RGB;

    #[test]
    fn can_clone() {
        let color = RGB { r: 5, g: 10, b: 15 };

        assert_eq!(color, color.clone());
    }

    #[test]
    fn can_copy() {
        let color = RGB { r: 5, g: 10, b: 15 };
        let copied_color = color;

        assert_eq!(color, copied_color);
    }

    #[test]
    fn can_debug() {
        let color = RGB { r: 5, g: 10, b: 15 };
        let value = format!("{:?}", color);

        assert_eq!(value, "RGB { r: 5, g: 10, b: 15 }");
    }

    #[test]
    fn can_convert_to_css() {
        let color = RGB { r: 5, g: 10, b: 255 };

        assert_eq!(color.to_css(), "rgb(5, 10, 255)");
    }

    #[test]
    fn can_print_in_css() {
        let color = RGB { r: 5, g: 10, b: 255 };
        let printed = format!("{}", color);

        assert_eq!(printed, "rgb(5, 10, 255)");
    }
}

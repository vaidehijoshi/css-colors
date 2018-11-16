use std::fmt;
use std::ops;

/// Construct an ratio from percentages. Values outside of the 0-100% range
/// will cause a panic.
///
/// # Example
/// ```
/// use css_colors::{percent};
///
/// assert_eq!(percent(0).to_string(), "0%");
/// assert_eq!(percent(25).to_string(), "25%");
/// assert_eq!(percent(100).to_string(), "100%");
/// ```
pub fn percent(percentage: u8) -> Ratio {
    Ratio::from_percentage(percentage)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A struct that represents a ratio and determines the legal value(s) for a given type.
/// Clamps any values that fall beyond the valid legal range for the type.
/// Used to convert a type into a valid percentage representation.
pub struct Ratio(u8);

impl Ratio {
    pub fn from_percentage(percentage: u8) -> Self {
        assert!(percentage <= 100, "Invalid value for percentage");

        Ratio::from_f32(percentage as f32 / 100.0)
    }

    pub fn from_u8(value: u8) -> Self {
        Ratio(value)
    }

    pub fn from_f32(float: f32) -> Self {
        assert!(float >= 0.0, "Invalid ratio for type f32");
        assert!(float <= 1.0, "Invalid ratio for type f32");

        Ratio((float * 255.0).round() as u8)
    }

    pub fn as_percentage(self) -> u8 {
        (self.0 as f32 / 255.0 * 100.0).round() as u8
    }

    pub fn as_u8(self) -> u8 {
        self.0
    }

    pub fn as_f32(self) -> f32 {
        self.0 as f32 / 255.0
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}%", self.as_percentage())
    }
}

impl ops::Add for Ratio {
    type Output = Ratio;

    fn add(self, other: Ratio) -> Ratio {
        clamp_ratio(self.as_f32() + other.as_f32())
    }
}

impl ops::Sub for Ratio {
    type Output = Ratio;

    fn sub(self, other: Ratio) -> Ratio {
        clamp_ratio(self.as_f32() - other.as_f32())
    }
}

impl ops::Mul for Ratio {
    type Output = Ratio;

    fn mul(self, other: Ratio) -> Ratio {
        clamp_ratio(self.as_f32() * other.as_f32())
    }
}

impl ops::Div for Ratio {
    type Output = Ratio;

    fn div(self, other: Ratio) -> Ratio {
        clamp_ratio(self.as_f32() / other.as_f32())
    }
}

// A function to clamp the value of a Ratio to fall between [0.0 - 1.0].
fn clamp_ratio(value: f32) -> Ratio {
    if value > 1.0 {
        Ratio::from_f32(1.0)
    } else if value >= 0.0 && value <= 1.0 {
        Ratio::from_f32(value)
    } else {
        Ratio::from_f32(0.0)
    }
}

#[cfg(test)]
mod tests {
    use Ratio;

    #[test]
    #[should_panic]
    fn handles_invalid_percentage() {
        Ratio::from_percentage(101);
    }

    #[test]
    #[should_panic]
    fn handles_invalid_f32() {
        Ratio::from_f32(1.01);
    }

    #[test]
    fn can_clamp_percentage() {
        assert_eq!(
            Ratio::from_percentage(50) + Ratio::from_percentage(55),
            Ratio::from_percentage(100)
        );
        assert_eq!(
            Ratio::from_percentage(50) - Ratio::from_percentage(55),
            Ratio::from_percentage(0)
        );
        assert_eq!(
            Ratio::from_percentage(55) / Ratio::from_percentage(50),
            Ratio::from_percentage(100)
        );
    }

    #[test]
    fn can_clamp_f32() {
        assert_eq!(
            Ratio::from_f32(0.75) + Ratio::from_f32(0.75),
            Ratio::from_f32(1.0)
        );
        assert_eq!(
            Ratio::from_f32(0.25) - Ratio::from_f32(0.75),
            Ratio::from_f32(0.0)
        );
        assert_eq!(
            Ratio::from_f32(0.75) / Ratio::from_f32(0.25),
            Ratio::from_f32(1.0)
        );
    }

    #[test]
    fn adds_percentage() {
        let a = Ratio::from_percentage(55);
        let b = Ratio::from_percentage(45);
        let c = Ratio::from_percentage(10);

        assert_eq!(a + b, Ratio::from_percentage(100));
        assert_eq!(a + c, Ratio::from_percentage(65));
    }

    #[test]
    fn subtracts_percentage() {
        let a = Ratio::from_percentage(45);
        let b = Ratio::from_percentage(10);
        let c = Ratio::from_percentage(1);

        assert_eq!(a - b, Ratio::from_percentage(35));
        assert_eq!(b - c, Ratio::from_percentage(9));
    }

    #[test]
    fn multiplies_percentage() {
        let a = Ratio::from_percentage(100);
        let b = Ratio::from_percentage(50);
        let c = Ratio::from_percentage(20);

        assert_eq!(a * a, Ratio::from_percentage(100));
        assert_eq!(b * b, Ratio::from_percentage(25));
        assert_eq!(c * c, Ratio::from_percentage(4));

        assert_eq!(a * b, Ratio::from_percentage(50));
        assert_eq!(b * a, Ratio::from_percentage(50));

        assert_eq!(a * c, Ratio::from_percentage(20));
        assert_eq!(c * a, Ratio::from_percentage(20));

        assert_eq!(b * c, Ratio::from_percentage(10));
        assert_eq!(c * b, Ratio::from_percentage(10));
    }

    #[test]
    fn divides_percentage() {
        let a = Ratio::from_percentage(100);
        let b = Ratio::from_percentage(50);
        let c = Ratio::from_percentage(20);

        assert_eq!(a / a, Ratio::from_percentage(100));
        assert_eq!(b / b, Ratio::from_percentage(100));
        assert_eq!(c / c, Ratio::from_percentage(100));

        assert_eq!(b / a, Ratio::from_percentage(50));
        assert_eq!(c / a, Ratio::from_percentage(20));
        assert_eq!(c / b, Ratio::from_percentage(40));
    }

    #[test]
    fn adds_f32() {
        let a = Ratio::from_f32(0.55);
        let b = Ratio::from_f32(0.45);
        let c = Ratio::from_f32(0.10);

        assert_eq!(a + b, Ratio::from_f32(1.0));
        assert_eq!(c + c, Ratio::from_u8(52));
        assert_eq!(b + c, Ratio::from_u8(141));
    }

    #[test]
    fn subtracts_f32() {
        let a = Ratio::from_f32(0.55);
        let b = Ratio::from_f32(0.45);
        let c = Ratio::from_f32(0.10);

        assert_eq!(b - c, Ratio::from_f32(0.35));
        assert_eq!(a - b, Ratio::from_u8(25));
        assert_eq!(a - c, Ratio::from_u8(114));
    }

    #[test]
    fn multiplies_f32() {
        let a = Ratio::from_f32(0.5);
        let b = Ratio::from_f32(0.25);

        assert_eq!(a * b, Ratio::from_f32(0.125));
        assert_eq!(a * a, Ratio::from_f32(0.25));
        assert_eq!(b * b, Ratio::from_f32(0.0625));
    }

    #[test]
    fn divides_f32() {
        let a = Ratio::from_f32(0.25);
        let b = Ratio::from_f32(0.50);
        let c = Ratio::from_f32(1.00);

        assert_eq!(a / b, Ratio::from_f32(0.5));
        assert_eq!(a / c, Ratio::from_f32(0.25));
        assert_eq!(b / c, Ratio::from_f32(0.5));
    }
}

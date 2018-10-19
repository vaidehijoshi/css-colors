use std::fmt;
use std::ops;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
/// A struct that represents a ratio and determines the legal value(s) for a given type.
/// Used to convert a type into a valid percentage representation.
pub struct Ratio(u8);

impl Ratio {
    pub fn from_percentage(percentage: u8) -> Self {
        assert!(percentage <= 100, "Invalid value for percentage");

        Ratio((percentage as f32 / 100.0 * 255.0).round() as u8)
    }

    pub fn from_u8(value: u8) -> Self {
        Ratio(value)
    }

    pub fn from_f32(float: f32) -> Self {
        assert!(float >= 0.0, "Invalid ratio for type f32");
        assert!(float <= 1.0, "Invalid ratio for type f32");

        Ratio((float / 1.0 * 255.0).round() as u8)
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
    type Output = Option<Ratio>;

    fn add(self, other: Ratio) -> Option<Ratio> {
        self.0.checked_add(other.0).map(|total| Ratio(total))
    }
}

impl ops::Sub for Ratio {
    type Output = Option<Ratio>;

    fn sub(self, other: Ratio) -> Option<Ratio> {
        self.0.checked_sub(other.0).map(|total| Ratio(total))
    }
}

impl ops::Mul for Ratio {
    type Output = Option<Ratio>;

    fn mul(self, other: Ratio) -> Option<Ratio> {
        self.0.checked_mul(other.0).map(|total| Ratio(total))
    }
}

impl ops::Div for Ratio {
    type Output = Option<Ratio>;

    fn div(self, other: Ratio) -> Option<Ratio> {
        self.0.checked_div(other.0).map(|total| Ratio(total))
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
    fn handles_overflow_percentage() {
        assert_eq!(
            Ratio::from_percentage(50) + Ratio::from_percentage(55),
            None
        );
        assert_eq!(
            Ratio::from_percentage(50) - Ratio::from_percentage(55),
            None
        );
        assert_eq!(
            Ratio::from_percentage(50) * Ratio::from_percentage(55),
            None
        );
    }

    #[test]
    fn handles_overflow_f32() {
        assert_eq!(Ratio::from_f32(0.75) + Ratio::from_f32(0.75), None);
        assert_eq!(Ratio::from_f32(0.25) - Ratio::from_f32(0.75), None);
        assert_eq!(Ratio::from_f32(0.07) * Ratio::from_f32(0.06), None);
    }

    #[test]
    fn computes_valid_percentage() {
        let a = Ratio::from_percentage(55);
        let b = Ratio::from_percentage(45);
        let c = Ratio::from_percentage(10);
        // let d = Ratio::from_percentage(5);

        assert_eq!((a + b).unwrap(), Ratio::from_percentage(100));
        assert_eq!((b - c).unwrap(), Ratio::from_percentage(35));
        // FIXME: need to figure out why this dosen't work as expected.
        // assert_eq!((a / d).unwrap(), Ratio::from_percentage(11));
    }

    #[test]
    fn computes_valid_f32() {
        let a = Ratio::from_f32(0.55);
        let b = Ratio::from_f32(0.45);
        let c = Ratio::from_f32(0.10);

        assert_eq!((a + b).unwrap(), Ratio::from_f32(1.0));
        assert_eq!((b - c).unwrap(), Ratio::from_f32(0.35));
    }
}

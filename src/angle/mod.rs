use std::fmt;
use std::ops;

pub fn degrees(mut degrees: i16) -> Angle {
    while degrees < 0 {
        degrees += 360;
    }

    while degrees >= 360 {
        degrees -= 360;
    }

    Angle::new(degrees as u16)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
/// A struct that represents the number of degrees in a circle.
/// Legal values range from `0-359`. Anything else is unused.
pub struct Angle {
    degrees: u16,
}

impl Angle {
    pub fn new(degrees: u16) -> Self {
        assert!(degrees < 360, "invalid angle");

        Angle { degrees }
    }

    pub fn degrees(self) -> u16 {
        self.degrees
    }
}

impl fmt::Display for Angle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.degrees)
    }
}

impl ops::Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        Angle {
            degrees: (360 - self.degrees) % 360,
        }
    }
}

impl ops::Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        let temp: u32 = self.degrees as u32 + other.degrees as u32;
        let degrees: u16 = (temp % 360) as u16;

        Angle { degrees }
    }
}

impl ops::Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        self + (-other)
    }
}

impl ops::Mul for Angle {
    type Output = Angle;

    fn mul(self, other: Angle) -> Angle {
        let temp: u32 = self.degrees as u32 * other.degrees as u32;
        let degrees: u16 = (temp % 360) as u16;

        Angle { degrees }
    }
}

impl ops::Div for Angle {
    type Output = Angle;

    fn div(self, other: Angle) -> Angle {
        if other.degrees == 0 {
            panic!("Cannot divide by zero-valued `Angle`!");
        }

        let temp: u32 = self.degrees as u32 / other.degrees as u32;
        let degrees: u16 = (temp % 360) as u16;

        Angle { degrees }
    }
}

#[cfg(test)]
mod tests {
    use Angle;

    #[test]
    fn can_have_degrees() {
        assert_eq!(Angle::new(30).degrees(), 30);
        assert_eq!(Angle::new(47).degrees(), 47);
    }

    #[test]
    fn can_display_angles() {
        assert_eq!("30", format!("{}", Angle::new(30)));
        assert_eq!("30", Angle::new(30).to_string());
    }

    #[test]
    fn can_eq_angles() {
        assert_eq!(Angle::new(30), Angle::new(30));
        assert_ne!(Angle::new(30), Angle::new(47));
    }

    #[test]
    fn can_ord_angles() {
        assert_eq!(Angle::new(30) < Angle::new(47), true);
        assert_eq!(Angle::new(47) < Angle::new(30), false);
        assert_eq!(Angle::new(30) < Angle::new(30), false);

        assert_eq!(Angle::new(30) <= Angle::new(47), true);
        assert_eq!(Angle::new(47) <= Angle::new(30), false);
        assert_eq!(Angle::new(30) <= Angle::new(30), true);

        assert_eq!(Angle::new(30) > Angle::new(47), false);
        assert_eq!(Angle::new(47) > Angle::new(30), true);
        assert_eq!(Angle::new(30) > Angle::new(30), false);

        assert_eq!(Angle::new(30) >= Angle::new(47), false);
        assert_eq!(Angle::new(47) >= Angle::new(30), true);
        assert_eq!(Angle::new(30) >= Angle::new(30), true);
    }

    #[test]
    fn can_add_angles() {
        assert_eq!(Angle::new(30) + Angle::new(47), Angle::new(77));
        assert_eq!(Angle::new(47) + Angle::new(30), Angle::new(77));
        assert_eq!(Angle::new(359) + Angle::new(1), Angle::new(0));
        assert_eq!(
            Angle::new(359) + Angle::new(359) + Angle::new(359),
            Angle::new(357)
        );
    }

    #[test]
    fn can_sub_angles() {
        assert_eq!(Angle::new(30) - Angle::new(47), Angle::new(343));
        assert_eq!(Angle::new(47) - Angle::new(30), Angle::new(17));
        assert_eq!(Angle::new(0) - Angle::new(1), Angle::new(359));
        assert_eq!(
            Angle::new(0) - Angle::new(359) - Angle::new(359) - Angle::new(359),
            Angle::new(3)
        );
    }

    #[test]
    fn test_mul_angles() {
        assert_eq!(Angle::new(30) * Angle::new(0), Angle::new(0));
        assert_eq!(Angle::new(30) * Angle::new(1), Angle::new(30));
        assert_eq!(Angle::new(30) * Angle::new(2), Angle::new(60));

        assert_eq!(Angle::new(30) * Angle::new(12), Angle::new(0));
        assert_eq!(Angle::new(30) * Angle::new(13), Angle::new(30));
        assert_eq!(Angle::new(30) * Angle::new(100), Angle::new(120));

        assert_eq!(Angle::new(47) * Angle::new(0), Angle::new(0));
        assert_eq!(Angle::new(47) * Angle::new(1), Angle::new(47));
        assert_eq!(Angle::new(47) * Angle::new(2), Angle::new(94));

        assert_eq!(Angle::new(47) * Angle::new(8), Angle::new(16));
        assert_eq!(Angle::new(47) * Angle::new(100), Angle::new(20));
    }

    #[test]
    fn test_divide_angles() {
        assert_eq!(Angle::new(30) / Angle::new(1), Angle::new(30));
        assert_eq!(Angle::new(30) / Angle::new(2), Angle::new(15));

        assert_eq!(Angle::new(180) / Angle::new(12), Angle::new(15));
        assert_eq!(Angle::new(180) / Angle::new(2), Angle::new(90));
        assert_eq!(Angle::new(180) / Angle::new(5), Angle::new(36));

        assert_eq!(Angle::new(47) / Angle::new(2), Angle::new(23));
    }
}

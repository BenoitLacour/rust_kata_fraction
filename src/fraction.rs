use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Formatter};

#[derive(PartialEq, Debug)]
pub struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    pub fn new(numerator: u32, denominator: u32) -> Fraction {
        Fraction { numerator, denominator }
    }
    pub fn add_fraction(&self, fraction: Fraction) -> Fraction {
        let denominator = fraction.denominator * self.denominator;
        let numerator = fraction.numerator * self.denominator + self.numerator * fraction.denominator;
        Self::simplify(numerator, denominator)
    }

    pub fn subtract(&self, fraction: Fraction) -> Fraction {
        let denominator = fraction.denominator * self.denominator;
        let numerator = fraction.numerator * self.denominator - self.numerator * fraction.denominator;
        Self::simplify(numerator, denominator)
    }

    pub fn multiply(&self, fraction: Fraction) -> Fraction {
        Self::simplify(fraction.numerator * self.numerator, fraction.denominator * self.denominator)
    }

    pub fn divide(&self, fraction: Fraction) -> Fraction {
        Self::simplify(self.numerator * fraction.denominator, self.denominator * fraction.numerator)
    }

    fn simplify(numerator: u32, denominator: u32) -> Fraction {
        let gcd = Self::pgcd(numerator, denominator);
        Fraction { numerator: numerator / gcd, denominator: denominator / gcd }
    }
    fn pgcd(a: u32, b: u32) -> u32 {
        if b == 0 {
            return a;
        }
        Self::pgcd(b, a % b)
    }
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "{};{}",
               self.numerator,
               self.denominator)
    }
}

impl TryFrom<&str> for Fraction {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fracts: Vec<&str> = value.split(&[';']).collect();
        if fracts.len() != 2 {
            return Err("Wrong input");
        }

        let numerator = match fracts[0].parse::<u32>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => return Err("Failed to parse numerator")
        };

        let denominator = match fracts[1].parse::<u32>() {
            Ok(parsed_num) => parsed_num,
            Err(_) => return Err("Failed to parse denominator")
        };

        if denominator == 0{
            return Err("Invalid denominator");
        }

        Ok(Fraction::new(numerator, denominator))
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;
    use std::convert::TryFrom;

    use crate::Fraction;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn should_get_a_fraction_when_adding_an_other_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 3 };
        let result = fraction.add_fraction(Fraction { numerator: 3, denominator: 5 });
        assert_eq!(type_of(result), "fraction_kata::fraction::Fraction")
    }

    #[test]
    fn should_add_a_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 3 };
        let result = fraction.add_fraction(Fraction { numerator: 3, denominator: 5 });
        assert_eq!(result, Fraction { numerator: 19, denominator: 15 })
    }

    #[test]
    fn should_reduce_a_fraction_when_added() {
        let fraction = Fraction { numerator: 1, denominator: 4 };
        let result = fraction.add_fraction(Fraction { numerator: 1, denominator: 4 });
        assert_eq!(result, Fraction { numerator: 1, denominator: 2 })
    }

    #[test]
    fn should_subtract_a_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 4 };
        let result = fraction.subtract(Fraction { numerator: 2, denominator: 4 });
        assert_eq!(result, Fraction { numerator: 0, denominator: 1 });
    }

    #[test]
    fn should_multiply_a_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 4 };
        let result = fraction.multiply(Fraction { numerator: 3, denominator: 4 });
        assert_eq!(result, Fraction { numerator: 3, denominator: 8 });
    }

    #[test]
    fn should_divide_a_fraction() {
        let fraction = Fraction { numerator: 8, denominator: 4 };
        let result = fraction.divide(Fraction { numerator: 3, denominator: 1 });
        assert_eq!(result, Fraction { numerator: 2, denominator: 3 });
    }

    #[test]
    fn should_get_a_valid_fraction() {
        let input = "1;2";
        let result = Fraction::try_from(input);
        assert_eq!(result.ok(), Some(Fraction::new(1, 2)));
    }

    #[test]
    fn should_get_an_error_when_denominator_is_zero() {
        let input = "1;0";
        let result = Fraction::try_from(input);
        assert_eq!(result.err().unwrap(), "Invalid denominator");
    }
}
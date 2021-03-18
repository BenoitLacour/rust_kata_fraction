use gcd::Gcd;

fn main() {}

#[derive(PartialEq, Debug)]
struct Fraction {
    numerator: u32,
    denominator: u32,
}

impl Fraction {
    fn add_fraction(&self, fraction: Fraction) -> Fraction {
        let denominator = fraction.denominator * self.denominator;
        let numerator = fraction.numerator * self.denominator + self.numerator * fraction.denominator;
        let gcd = numerator.gcd(denominator);
        Fraction {numerator : numerator/gcd, denominator : denominator/gcd }
    }
}

#[cfg(test)]
mod tests {
    use std::any::type_name;

    use crate::Fraction;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn should_get_a_fraction_when_adding_an_other_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 3 };
        let result = fraction.add_fraction(Fraction { numerator: 3, denominator: 5 });
        assert_eq!(type_of(result), "fraction_kata::Fraction")
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
}
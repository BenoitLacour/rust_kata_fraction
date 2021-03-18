fn main() {}

#[derive(PartialEq, Debug)]
struct Fraction {
    numerator: i32,
    denominator: i32,
}

impl Fraction {
    fn add_fraction(&self, fraction: Fraction) -> Fraction {
        fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::Fraction;
    use std::any::type_name;

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[test]
    fn should_get_a_fraction_when_adding_an_other_fraction() {
        let fraction = Fraction { numerator: 2, denominator: 3 };
        let result = fraction.add_fraction(Fraction { numerator: 3, denominator: 5 });
        assert_eq!(type_of(result), "fraction_kata::Fraction")
    }
}
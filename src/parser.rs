use std::convert::TryFrom;
use std::fmt;
use std::fmt::{Debug, Formatter};
use crate::fraction::Fraction;

#[derive(Debug, PartialEq)]
pub struct ParsedFractions {
    pub first: Fraction,
    pub second: Fraction,
    pub operation: Operation,
}

impl TryFrom<&str> for ParsedFractions {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let fractions: Vec<&str> = value.split(&['x', '+', '-', '*']).collect();
        if fractions.len() != 2 {
            return Err("Wrong input");
        }

        let sign_position = value.find(&['x', '+', '-', '*']);
        let sign_result = match sign_position {
            None => return Err("Wrong input"),
            Some(pos) => {
                let sign_str = value.chars().nth(pos).unwrap();
                Operation::try_from(&sign_str)
            }
        };

        let sign = match sign_result {
            Ok(res) => res,
            Err(err) => return Err(err)
        };

        let first_fraction = match Fraction::try_from(fractions[0]) {
            Ok(res) => res,
            Err(err) => return Err(err)
        };

        let second_fraction = match Fraction::try_from(fractions[1]) {
            Ok(res) => res,
            Err(err) => return Err(err)
        };

        Ok(ParsedFractions {
            first: first_fraction,
            second: second_fraction,
            operation: sign,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Multiply,
    Add,
    Divide,
    Subtract,
}

impl TryFrom<&char> for Operation {
    type Error = &'static str;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'x' => { Ok(Operation::Multiply) }
            '/' => { Ok(Operation::Divide) }
            '+' => { Ok(Operation::Add) }
            '-' => { Ok(Operation::Subtract) }
            _ => { Err("Unknown sign") }
        }
    }
}

impl fmt::Display for ParsedFractions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,
               "first:{};second:{};operation:{:?}",
               self.first,
               self.second,
               self.operation)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use crate::fraction::Fraction;
    use crate::parser::{Operation, ParsedFractions};

    #[test]
    fn should_get_a_multiply() {
        let input = 'x';
        let result = Operation::try_from(&input);
        assert_eq!(result.ok(), Some(Operation::Multiply))
    }

    #[test]
    fn should_get_a_add() {
        let input = '+';
        let result = Operation::try_from(&input);
        assert_eq!(result.ok(), Some(Operation::Add))
    }

    #[test]
    fn should_get_a_divide() {
        let input = '/';
        let result = Operation::try_from(&input);
        assert_eq!(result.ok(), Some(Operation::Divide))
    }

    #[test]
    fn should_get_a_subtract() {
        let input = '-';
        let result = Operation::try_from(&input);
        assert_eq!(result.ok(), Some(Operation::Subtract))
    }

    #[test]
    fn should_get_an_error() {
        let input = '1';
        let result = Operation::try_from(&input);
        assert_eq!(result.err(), Some("Unknown sign"))
    }

    #[test]
    fn should_get_a_result_with_fractions_and_operation() {
        let input = "1;3x1;2";
        let result = ParsedFractions::try_from(input);
        assert_eq!(result.ok(), Some(ParsedFractions {
            first: Fraction::new(1, 3),
            second: Fraction::new(1, 2),
            operation: Operation::Multiply,
        }));
    }
}
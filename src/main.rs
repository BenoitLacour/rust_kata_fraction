use std::convert::TryFrom;
use crate::fraction::Fraction;

use std::env;
use crate::parser::{Operation, ParsedFractions};

mod fraction;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let user_input: &str = &args[1];
        let parsed = ParsedFractions::try_from(user_input);

        if let Ok(inputs) = parsed {
            println!("input:{}", inputs);

            let operation_result = match inputs.operation {
                Operation::Multiply => inputs.first.multiply(inputs.second),
                Operation::Add => inputs.first.add_fraction(inputs.second),
                Operation::Divide => inputs.first.divide(inputs.second),
                Operation::Subtract => inputs.first.subtract(inputs.second),
            };

            println!("result:{}", operation_result);
        }

        return;
    }
    println!(
        "{}", Fraction::new(1, 2)
            .add_fraction(Fraction::new(2, 2)));
}

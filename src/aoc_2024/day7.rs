use crate::aoc_2024::day7::Operator::{ADD, CONCAT, MULTIPLY};
use std::fmt::Debug;
use std::str::FromStr;
use strum::EnumCount;
use strum_macros::EnumCount;

pub fn day7(input: &String) -> String {
    let ops = parse_operations(input);
    ops.iter()
        .filter(|op| op.try_all_combinations())
        .map(Operation::get_result)
        .sum::<usize>()
        .to_string()
}

pub fn day7_2(_input: &String) -> String {
    String::from("NYI")
}

fn parse_operations(input: &String) -> Vec<Operation> {
    input
        .split("\n")
        .map(Operation::from_str)
        .filter_map(Result::ok)
        .collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, EnumCount)]
enum Operator {
    ADD,
    MULTIPLY,
    CONCAT,
}

impl From<usize> for Operator {
    fn from(value: usize) -> Self {
        match value {
            0 => ADD,
            1 => MULTIPLY,
            2 => CONCAT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Operation {
    result: usize,
    values: Vec<usize>,
}

impl Operation {
    fn get_result(&self) -> usize {
        self.result
    }

    /// apply operators last first
    fn compute(&self, operators: &Vec<Operator>) -> usize {
        let mut ops = operators.to_owned();
        ops.reverse(); // We are popping from last
        self.values
            .iter()
            .map(usize::to_owned)
            .reduce(|acc, value| {
                let operator = ops.pop().unwrap();
                match operator {
                    MULTIPLY => acc * value,
                    ADD => acc + value,
                    CONCAT => {
                        let mut acc_as_string = acc.to_string();
                        acc_as_string.push_str(value.to_string().as_str());
                        usize::from_str(acc_as_string.as_str()).unwrap()
                    }
                }
            })
            .unwrap()
    }

    fn get_value_in_base(base: usize, number: usize, position: usize) -> usize {
        let divisor_to_round = base.pow(position as u32);
        let value_rounded_to = number / divisor_to_round;
        let value_at_modulo = value_rounded_to % base;
        value_at_modulo
    }

    fn try_all_combinations(&self) -> bool {
        let op_size = self.values.len() - 1;

        let possible_combinations = Operator::COUNT.pow(op_size as u32);
        // println!("Searching for {:?} with {} possible combinations", self, possible_combinations);

        let first_matching: Option<Vec<Operator>> = (0..possible_combinations)
            .map(|combination_number| {
                (0..op_size)
                    .map(|position| {
                        Self::get_value_in_base(Operator::COUNT, combination_number, position)
                    })
                    .map(Operator::from)
                    .collect::<Vec<Operator>>()
            })
            // .inspect(|possibility| println!("Possibility found {:?}", possibility))
            .filter(|combination| self.compute(&combination) == self.result)
            // .inspect(|combination| {println!("Combination OK: {}", combination)})
            .next();

        let is_some_matching = first_matching.is_some();
        if is_some_matching {
            println!(
                "Combinations found for {:?}: {:?}",
                self,
                first_matching.unwrap()
            );
        } else {
            println!("No combination found for {:?}", self);
        }
        is_some_matching
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split(":").collect::<Vec<&str>>();
        if split.len() != 2 {
            log::error!("Could not parse input: {:?}", input);
            return Err(());
        }
        let result = split[0].parse::<usize>().unwrap();
        let values: Vec<usize> = split[1]
            .split(' ')
            .map(|x| x.parse::<usize>())
            .filter_map(Result::ok)
            .collect();
        Ok(Operation { result, values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &'static str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    // 3312271365652
    #[test]
    fn test_day7_2() {
        let input = String::from(PUZZLE_INPUT);

        let result = day7(&input);

        assert_eq!(String::from("11387"), result);
    }

    #[test]
    fn test_day7_all_multiply_all_add() {
        let input = String::from(
            "120: 2 3 4 5
15: 1 2 3 4 5\
",
        );

        let result = day7(&input);

        assert_eq!(String::from("135"), result);
    }

    #[test]
    fn test_digit_extractor() {
        assert_eq!(1, Operation::get_value_in_base(2, 0b00010, 1));
        assert_eq!(1, Operation::get_value_in_base(2, 0b00001, 0));
        assert_eq!(1, Operation::get_value_in_base(2, 0b10100, 2));
        assert_eq!(1, Operation::get_value_in_base(2, 0b11000, 3));
        assert_eq!(1, Operation::get_value_in_base(2, 0b10000, 4));
        assert_eq!(0, Operation::get_value_in_base(2, 0b10000, 5));
    }

    #[test]
    fn test_digit_extractor_2() {
        assert_eq!(1, (0b10 / 2) % 2);
        assert_eq!(1, (0b100 / 4) % 2);
        assert_eq!(1, (0b1000 / 8) % 2);
        assert_eq!(1, (0b10000 / 16) % 2);
        assert_eq!(1, (0b10010 / 2) % 2);
    }
}

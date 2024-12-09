use crate::aoc_2024::day7::Operator::{ADD, MULTIPLY};
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
}

impl From<usize> for Operator {
    fn from(value: usize) -> Self {
        match value {
            0 => ADD,
            1 => MULTIPLY,
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
                }
            })
            .unwrap()
    }

    fn create_operators(&self) -> Vec<Operator> {
        (0..self.values.len()).map(|_| ADD).collect()
    }

    fn try_all_combinations(&self) -> bool {
        let op_size = self.values.len() - 1;

        let possible_combinations = Operator::COUNT.pow(op_size as u32);
        // println!("Searching for {:?} with {} possible combinations", self, possible_combinations);

        let first_matching: Option<Vec<Operator>> = (0..possible_combinations)
            .map(|combination_number| {
                (0..op_size)
                    .map(|position| {
                        let mut divisor_to_round = position * Operator::COUNT;
                        if divisor_to_round == 0 {
                            divisor_to_round = 1;
                        }
                        let value_rounded_to = combination_number / divisor_to_round;
                        let value_at_modulo = value_rounded_to % Operator::COUNT;
                        value_at_modulo
                    })
                    .map(Operator::from)
                    .collect::<Vec<Operator>>()
            })
            // .inspect(|possibility| println!("Possibility found {:?}", possibility))
            // .inspect(|combination| {println!("Combination OK: {}", combination)})
            .filter(|combination| self.compute(&combination) == self.result)
            .next();

        let is_some_matching = first_matching.is_some();
        if is_some_matching {
            println!("Combinations found for {:?}: {:?}", self, first_matching.unwrap());
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

    #[test]
    fn test_day7() {
        let input = String::from(PUZZLE_INPUT);

        let result = day7(&input);

        assert_eq!(String::from("3749"), result);
    }

    #[test]
    fn test_day7_all_multiply() {
        let input = String::from("120: 2 3 4 5
15: 1 2 3 4 5\
");

        let result = day7(&input);

        assert_eq!(String::from("135"), result);
    }

    #[test]
    fn test_day7_2() {
        let input = String::from(PUZZLE_INPUT);
        let result = day7_2(&input);

        assert_eq!(String::from("NYI"), result);
    }
}

#[cfg(not(test))]
use log::info;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
use crate::aoc_2025::day6::Operation::{Add, Divide, Multiply, Subtract};

pub fn day6(input: &String) -> String {
    parse_input(input).iter()
        .map(|(values, operation)| {
            let res = operation.execute_list(values);
            info!("Executing operation {operation:?} of {values:?} = {res}");
            res
        })
        .sum::<usize>()
        .to_string()
}

pub fn day6_2(input: &String) -> String {
    parse_input(input)
        .iter()
        .map(|(values, operation)| {
            let res = operation.execute_list(values);
            info!("Executing operation {operation:?} of {values:?} = {res}");
            res
        })
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operation {

    fn get_neutral(&self) -> usize {
        match self {
            Add => 0,
            Subtract => 0,
            Multiply => 1,
            Divide => 1
        }
    }

    pub fn execute(&self, val1: &usize, val2: &usize) -> usize {
        match self {
            Add => val1 + val2,
            Subtract => panic!("Not implemented"),
            Multiply => val1 * val2,
            Divide => panic!("Not implemented"),
        }
    }

    pub fn execute_list(&self, values: &Vec<usize>) -> usize {
        values.iter().fold(self.get_neutral(), |v1, v2| self.execute(&v1, v2))
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim() {
            "+" => Ok(Add),
            "-" => Ok(Subtract),
            "*" => Ok(Multiply),
            "/" => Ok(Divide),
            _ => Err(())
        }
    }
}

fn parse_input(input: &str) -> Vec<(Vec<usize>, Operation)> {
    let lines: Vec<Vec<&str>> = input.split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ").map(str::trim).filter(|elem| !elem.is_empty()).collect()
        })
        .collect();

    let number_of_operations = lines.first().unwrap().len();

    let mut operations: Vec<(Vec<usize>, Operation)> = Vec::with_capacity(number_of_operations);
    let (operator_line, values_lines) = lines.split_last().unwrap();

    for op_id in 0..number_of_operations {
        let operation_values = values_lines.iter()
            .map(|value_line| value_line.get(op_id).unwrap())
            .map(|x1| x1.parse::<usize>().unwrap())
            .collect();

        let operation = operator_line.get(op_id).map(|t| {
            Operation::from_str(*t).unwrap()
        }).unwrap();

        operations.push((operation_values, operation))
    }
    operations
}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";

    #[test]
    fn test_day6() {
        let input = String::from(TEST_INPUT);

        let result = day6(&input);

        assert_eq!(String::from("4277556"), result);
    }

    #[test]
    fn test_day6_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day6_2(&input);

        assert_eq!(String::from("14"), result);
    }
}

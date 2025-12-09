#[cfg(not(test))]
use log::info;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
use crate::aoc_2025::day6::Operator::{Add, Divide, Multiply, Subtract};

pub fn day6(input: &String) -> String {
    parse_input(input).iter()
        .map(|operation| operation.compute())
        .sum::<usize>()
        .to_string()
}

pub fn day6_2(input: &String) -> String {
    parse_input_2(input).iter()
        .map(|operation| operation.compute())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Operation {
    values: Vec<usize>,
    operator: Operator
}

impl Operation {
    pub fn compute(&self) -> usize {
        let res = self.operator.execute_list(&self.values);
        info!("Executing operation {self:?}= {res}");
        res
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operator {

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

impl FromStr for Operator {
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

fn parse_input(input: &str) -> Vec<Operation> {
    let split = parse_columns(input);
    split.iter()
        .map(|x| to_operation(x))
        .collect()
}

fn to_operation(col: &Vec<&str>) -> Operation {
    let (operator_str, values_str) = col.split_last().unwrap();
    let values: Vec<usize> = values_str.iter().map(|x| x.trim().parse().unwrap()).collect();
    let operator = Operator::from_str(operator_str).unwrap();
    Operation {values, operator }
}

fn parse_input_2(input: &str) -> Vec<Operation> {
    let split = parse_columns(input);
    split.iter()
        .map(|x| to_operation_2(x))
        .collect()
}

fn to_operation_2(col: &Vec<&str>) -> Operation {
    let (operator_str, values_str) = col.split_last().unwrap();

    let col_len = values_str.first().unwrap().len();
    let mut values: Vec<usize> = Vec::new();

    for i in (0..col_len).rev() {
        let value: String = values_str.iter().map(|x| x.get(i..=i).unwrap()).collect();
        let result = value.trim().parse();
        if result.is_ok() {
            values.push(result.unwrap())
        }
    }

    let operator = Operator::from_str(operator_str).unwrap();

    Operation {values, operator }
}

fn parse_columns(input: &str) -> Vec<Vec<&str>> {
    let split: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let nb_of_columns = split.first().unwrap().len();

    let mut columns: Vec<Vec<&str>> = Vec::new();
    let mut last_index = 0;
    for i in 0..nb_of_columns {
        let should_new_col = split.iter().all(|l| l.get(i..=i).unwrap().eq(" "));

        if should_new_col {
            let new_col = split.iter().map(|l| l.get(last_index..i).unwrap()).collect();
            info!("New column parsed: {new_col:?}");
            columns.push(new_col);
            last_index = i;
        }
    }
    // push last column
    let new_col = split.iter().map(|l| l.get(last_index..).unwrap()).collect();
    info!("New column parsed: {new_col:?}");
    columns.push(new_col);
    columns
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str =
"\n
\n123 328  51 64 \
\n 45 64  387 23 \
\n  6 98  215 314\
\n*   +   *   +  \
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

        assert_eq!(String::from("3263827"), result);
    }
}

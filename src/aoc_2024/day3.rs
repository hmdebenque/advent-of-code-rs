use clap::builder::TypedValueParser;
use regex::{Captures, Regex};
use std::fmt::Debug;

pub fn day3(input: &String) -> String {
    let mut parser = OperationsParser::new();
    for char in input.chars() {
        parser.push(char);
    }
    parser.sum_results().to_string()
}

trait Operation: Debug {
    fn compute(&self) -> usize;
}

#[derive(Debug)]
struct Multiplication {
    left: usize,
    right: usize,
}

impl Operation for Multiplication {
    fn compute(&self) -> usize {
        self.left * self.right
    }
}

#[derive(Debug)]
enum OperationType {
    DO,
    DONT,
    MULTIPLY(usize, usize),
}

struct OperationBuilder {
    build_regexp: Regex,
    match_regexp: Regex,
    chars: String,
    parser: fn(Captures) -> OperationType,
}

impl OperationBuilder {
    fn clear(&mut self) {
        self.chars.clear();
    }

    fn matches(&self, input: &str) -> bool {
        self.build_regexp.is_match(input)
    }

    fn parse_operation(&self, input: &String) -> Option<Box<OperationType>> {
        let option = self.match_regexp.captures(input);
        option.map(self.parser).map(Box::new)
    }

    /// Returns a built operation if matches a full operation
    fn push(&mut self, next: char) -> Option<Box<OperationType>> {
        self.chars.push(next);

        if !self.matches(&self.chars) {
            log::trace!("{} do not match. Clearing.", self.chars);
            self.chars.clear();
            None
        } else {
            log::trace!("{} partially matches.", self.chars);
            let ope_parsed = self.parse_operation(&self.chars);
            // If matches full we need to clear the buffer
            if ope_parsed.is_some() {
                let result = ope_parsed.unwrap();
                log::info!(
                    "{} fully matches! Clearing and returning new operation {:?}",
                    self.chars,
                    result
                );
                self.chars.clear();
                return Some(result);
            }
            None
        }
    }
}

fn new_mul_builder() -> OperationBuilder {
    let build_regexp = Regex::new(r"^m(u(l(\((\d{0,3}(,(\d{1,3}(\)?)?)?)?)?)?)?)?$").unwrap();
    let match_regexp = Regex::new(r"^mul\((\d{0,3}),(\d{1,3})\)$").unwrap();
    OperationBuilder {
        build_regexp,
        match_regexp,
        chars: String::new(),
        parser: |captures: Captures| {
            OperationType::MULTIPLY(captures[1].parse().unwrap(), captures[2].parse().unwrap())
        },
    }
}

fn new_do_builder() -> OperationBuilder {
    let build_regexp = Regex::new(r"^d(o(\((\))?)?)?$").unwrap();
    let match_regexp = Regex::new(r"^do\(\)$").unwrap();
    OperationBuilder {
        build_regexp,
        match_regexp,
        chars: String::new(),
        parser: |captures: Captures| OperationType::DO,
    }
}

fn new_dont_builder() -> OperationBuilder {
    let build_regexp = Regex::new(r"^d(o(n('(t(\((\))?)?)?)?)?)?$").unwrap();
    let match_regexp = Regex::new(r"^don't\(\)$").unwrap();
    OperationBuilder {
        build_regexp,
        match_regexp,
        chars: String::new(),
        parser: |captures: Captures| OperationType::DO,
    }
}

struct OperationsParser {
    mul_builder: OperationBuilder,
    do_builder: OperationBuilder,
    dont_builder: OperationBuilder,
    operations: Vec<Box<OperationType>>,
    should_take: bool,
}

impl OperationsParser {
    fn new() -> OperationsParser {
        OperationsParser {
            mul_builder: new_mul_builder(),
            do_builder: new_do_builder(),
            dont_builder: new_dont_builder(),
            operations: Vec::new(),
            should_take: true,
        }
    }

    fn push(&mut self, next: char) {
        let do_option = self.do_builder.push(next);
        if do_option.is_some() {
            self.should_take = true;
        }
        let dont_option = self.dont_builder.push(next);
        if dont_option.is_some() {
            self.should_take = false;
        }
        if self.should_take {
            let mul_option = self.mul_builder.push(next);
            if mul_option.is_some() {
                self.operations.push(mul_option.unwrap());
            }
        }
    }

    fn sum_results(&self) -> usize {
        self.operations
            .iter()
            .filter_map(|op| {
                if let OperationType::MULTIPLY(left, right) = **op {
                    Some(left * right)
                } else {
                    None
                }
            })
            .sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let input =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

        let result = day3(&input);

        assert_eq!(String::from("161"), result);
    }

    #[test]
    fn test_day3_2() {
        let input =
            String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");

        let result = day3(&input);

        assert_eq!(String::from("48"), result);
    }
}

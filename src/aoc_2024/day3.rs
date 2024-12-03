use std::fmt::Debug;
use regex::Regex;

pub fn day3(input: &String) -> String {
    input
        .chars()
        .scan(OperationsParser::new(), |p, i| p.push(i))
        .map(|arg0: Box<dyn Operation>| arg0.compute())
        .sum::<usize>()
        .to_string()
}

pub fn day3_2(_input: &String) -> String {
    String::from("not yet implemented")
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

struct MultiplicationBuilder {
    build_regexp: Regex,
    match_regexp: Regex,
    chars: String,
}

impl MultiplicationBuilder {
    fn new() -> Self {
        let build_regexp = Regex::new(r"^m(u(l(\((\d{0,3}(,(\d{1,3}(\)?)?)?)?)?)?)?)?$").unwrap();
        let match_regexp = Regex::new(r"^mul\((\d{0,3}),(\d{1,3})\)$").unwrap();
        MultiplicationBuilder {
            build_regexp,
            match_regexp,
            chars: String::new(),
        }
    }

    fn matches(&self, input: &str) -> bool {
        self.build_regexp.is_match(input)
    }

    fn parse_operation(&self, input: &String) -> Option<Box<dyn Operation>> {
        let option = self.match_regexp.captures(input);
        option
            .map(|captures| Multiplication {
                left: captures[1].parse().unwrap(),
                right: captures[2].parse().unwrap(),
            })
            .map(Box::new)
            .map(|t| t as Box<dyn Operation>)
    }

    /// Returns a built operation if matches a full operation
    fn push(&mut self, next: char) -> Option<Box<dyn Operation>> {
        self.chars.push(next);

        if !self.matches(&self.chars) {
            log::error!("{} do not match. Clearing.", self.chars);
            self.chars.clear();
            None
        } else {
            log::error!("{} partially matches.", self.chars);
            let ope_parsed = self.parse_operation(&self.chars);
            // If matches full we need to clear the buffer
            if ope_parsed.is_some() {
                let result = ope_parsed.unwrap();
                log::error!(
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

struct OperationsParser {
    mul_builder: MultiplicationBuilder,
}

impl OperationsParser {
    fn new() -> OperationsParser {
        OperationsParser {
            mul_builder: MultiplicationBuilder::new(),
        }
    }

    fn push(&mut self, next: char) -> Option<Box<dyn Operation>> {
        self.mul_builder.push(next)
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
}

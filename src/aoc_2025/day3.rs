use std::cmp::{max, min};
use std::collections::HashSet;
use std::isize;
use std::str::{Chars, FromStr};
use log::{debug, info};
use strum_macros::{Display, ToString};
use crate::aoc_2025::common::Range;


#[derive(Debug)]
struct PowerBank {
    batteries: Vec<usize>
}

impl PowerBank {
    /// Get the max from the power bank
    fn max_joltage(&self) -> usize {
        let batteries_nb = self.batteries.len();
        if batteries_nb == 0 {
            return 0;
        } else if batteries_nb == 1 {
            return self.batteries[0];
        } else {
            let max_index = batteries_nb - 1;
            let mut max_value: usize = 0;
            let mut max_value_index: usize = 0;
            for ( index, value) in self.batteries.iter().enumerate() {
                if index < max_index && *value > max_value {
                    max_value = *value;
                    max_value_index = index;
                }
            }
            let next_max_after = *(self.batteries.iter().skip(max_value_index + 1).max().unwrap());
            return (max_value * 10) + next_max_after;
        }
    }

}

pub fn day3(input: &String) -> String {
    let mut inputs = parse_input(input);
    let sum: usize = inputs.iter()
        .map(PowerBank::max_joltage)
        .sum();

    sum.to_string()
}

pub fn day3_2(input: &String) -> String {
    String::from("todo")
}

fn parse_input(input: &str) -> Vec<PowerBank> {
    input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| {
            let batteries = s.chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .map(|x| x as usize)
                .collect();
            let bank = PowerBank { batteries };
            println!("parsed Power bank: {bank:?}");
            debug!("parsed Power bank: {bank:?}");
            bank
        }
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_day1() {
        let input = String::from(TEST_INPUT);

        let result = day3(&input);

        assert_eq!(String::from("357"), result);
    }

    #[test]
    fn test_day1_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day3_2(&input);

        assert_eq!(String::from("4174379265"), result);
    }
}

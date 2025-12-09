use crate::aoc_2025::common::Range;
use log::info;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::isize;
use std::str::FromStr;

pub fn day2(input: &String) -> String {
    let mut inputs = parse_input(input);
    let sum: usize = inputs
        .iter()
        .map(|r| r)
        .flat_map(|r| get_all_values(&r))
        .filter(|x| has_repeating_pattern(x, 2))
        .sum();

    sum.to_string()
}

pub fn day2_2(input: &String) -> String {
    let mut inputs = parse_input(input);
    let sum: usize = inputs
        .iter()
        .map(|r| r)
        .flat_map(|r| get_all_values(&r))
        .filter(|x| has_repeating_pattern(x, usize::MAX))
        .sum();

    sum.to_string()
}

fn get_all_values(input: &Range) -> Vec<usize> {
    (input.start..=input.end).collect()
}

fn has_repeating_pattern(input: &usize, max_limit: usize) -> bool {
    let input_str: String = input.to_string();
    let input_len = input_str.len();

    let max_parts = min(max_limit, input_len);
    for parts in 2..=max_parts {
        // is dividable
        if input_len % parts == 0 {
            let part_size = input_len / parts;

            let chunks: HashSet<String> = input_str
                .chars()
                .collect::<Vec<_>>()
                .chunks(part_size)
                .map(|chunk| chunk.iter().collect())
                .collect();

            if chunks.len() == 1 {
                return true;
            }
        }
    }

    false
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .split(",")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| {
            let sp: Vec<&str> = s.split('-').collect();
            let start = usize::from_str(sp[0]).unwrap();
            let end = usize::from_str(sp[1]).unwrap();
            Range { start, end }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_day1() {
        let input = String::from(TEST_INPUT);

        let result = day2(&input);

        assert_eq!(String::from("1227775554"), result);
    }

    #[test]
    fn test_day1_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day2_2(&input);

        assert_eq!(String::from("4174379265"), result);
    }
}

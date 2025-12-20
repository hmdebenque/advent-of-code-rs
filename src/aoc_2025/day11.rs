use crate::aoc_2024::common::Direction::{East, South, West};
use crate::aoc_2024::common::{CharMatrix, Coordinates, Direction};
#[cfg(not(test))]
use log::info;
use std::collections::HashSet;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
use std::time::Instant;
use crate::aoc_2025::common::graph::Graph;

pub fn day11(input: &String) -> String {
    
    let mut graph = Graph::new();

    input.split("\n")
        .filter(|x| !x.is_empty())
        .map(|line| {
            let (name, _) = line.split_once(":").unwrap();
            name
        })
        .for_each(|node_name| graph.addNode(node_name));




    String::new()
}

pub fn day11_2(input: &String) -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_day11() {
        let input = String::from(TEST_INPUT);

        let result = day11(&input);

        assert_eq!(String::from("5"), result);
    }

    #[test]
    fn test_day11_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day11_2(&input);

        assert_eq!(String::from("40"), result);
    }
}

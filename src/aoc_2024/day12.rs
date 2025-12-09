use crate::aoc_2024::common::CharMatrix;
#[cfg(not(test))]
use log::info;
// Use log crate when building application
#[cfg(test)]
use std::println as info;
use std::str::FromStr;

pub fn day12(input: &String) -> String {
    let char_matrix = CharMatrix::from_str(input).unwrap();
    info!("Parsed input:\n{}", char_matrix.print());

    String::from("NYI")
}

pub fn day12_2(input: &String) -> String {
    let char_matrix = CharMatrix::from_str(input).unwrap();
    info!("Parsed input:\n{}", char_matrix.print());
    String::from("NYI")
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT_SMALL: &'static str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    const PUZZLE_INPUT_BIG: &'static str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    #[test]
    #[test_log::test]
    #[ignore]
    fn test_day10_small() {
        let input = String::from(PUZZLE_INPUT_SMALL);

        let result = day12(&input);

        assert_eq!(String::from("140"), result);
    }

    #[test]
    #[test_log::test]
    #[ignore]
    fn test_day10_big() {
        let input = String::from(PUZZLE_INPUT_BIG);

        let result = day12(&input);

        assert_eq!(String::from("1930"), result);
    }

    #[test]
    #[test_log::test]
    #[ignore]
    fn test_day10_2() {
        let input = String::from(PUZZLE_INPUT_BIG);

        let result = day12_2(&input);

        assert_eq!(String::from("81"), result);
    }
}

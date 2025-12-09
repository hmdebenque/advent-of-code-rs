use crate::aoc_2024::common::{CharMatrix, Coordinates, Direction};
#[cfg(not(test))]
use log::{info, warn};
// Use log crate when building application
use std::collections::HashSet;
use std::str::FromStr;
#[cfg(test)]
use std::{println as info, println as warn};

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


fn explore(map: &CharMatrix, previous_level: isize, location: &Coordinates) -> Vec<Coordinates> {
    let level_opt = map.get_char_at(location);
    if level_opt.is_err() {
        return vec![];
    }
    let level = char::to_digit(level_opt.unwrap(), 10).unwrap() as isize;
    if level != previous_level + 1 {
        return vec![];
    }
    println!(
        "Exploring level (prev: {}) {} at {:?}",
        previous_level, level, location
    );

    if level == 9 {
        return vec![location.clone()];
    }

    explore(map, level, &Direction::North.advance(location))
        .into_iter()
        .chain(explore(map, level, &Direction::East.advance(location)).into_iter())
        .chain(explore(map, level, &Direction::South.advance(location)).into_iter())
        .chain(explore(map, level, &Direction::West.advance(location)).into_iter())
        .collect()
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

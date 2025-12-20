use crate::aoc_2024::common::{CharMatrix, Coordinates2D, Direction};
#[cfg(not(test))]
use log::info;
// Use log crate when building application
use std::collections::HashSet;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;

pub fn day10(input: &String) -> String {
    let char_matrix = CharMatrix::from_str(input).unwrap();
    info!("Parsed input:\n{}", char_matrix.print());
    char_matrix
        .search_chars(&'0')
        .iter()
        .map(|x| {
            println!("New exploration starting at: {:?}", x);
            explore(&char_matrix, -1, x)
                .into_iter()
                .collect::<HashSet<Coordinates2D>>()
                .iter()
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn day10_2(input: &String) -> String {
    let char_matrix = CharMatrix::from_str(input).unwrap();
    info!("Parsed input:\n{}", char_matrix.print());
    char_matrix
        .search_chars(&'0')
        .iter()
        .map(|x| {
            println!("New exploration starting at: {:?}", x);
            explore(&char_matrix, -1, x).iter().count()
        })
        .sum::<usize>()
        .to_string()
}

fn explore(
    map: &CharMatrix,
    previous_level: isize,
    location: &Coordinates2D,
) -> Vec<Coordinates2D> {
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

    const PUZZLE_INPUT: &'static str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    #[test_log::test]
    fn test_day10() {
        let input = String::from(PUZZLE_INPUT);

        let result = day10(&input);

        assert_eq!(String::from("36"), result);
    }

    #[test]
    #[test_log::test]
    fn test_day10_2() {
        let input = String::from(PUZZLE_INPUT);

        let result = day10_2(&input);

        assert_eq!(String::from("81"), result);
    }
}

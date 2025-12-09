use crate::aoc_2024::common::{CharMatrix, Coordinates, Direction};
use std::str::FromStr;
use strum::IntoEnumIterator;

pub fn day4(input: &String) -> String {
    let matrix = parse_input(input);
    matrix
        .search_chars(&'@')
        .iter()
        .map(|x| {
            Direction::iter()
                .map(|dir| matrix.get_char_at(&dir.advance(x)))
                .filter(|result| result.is_ok())
                .map(|result| result.unwrap())
                .filter(|result| *result == '@')
                .count()
        })
        .filter(|adjacent_count| *adjacent_count < 4usize)
        .count()
        .to_string()
}

const SLOT_FILE_PRESENT_CHAR: char = '@';
const EMPTY_SLOT_CHAR: char = '.';

pub fn day4_2(input: &String) -> String {
    let mut matrix = parse_input(input);

    let mut removed_total = 0;

    loop {
        let removed: Vec<Coordinates> = matrix
            .search_chars(&SLOT_FILE_PRESENT_CHAR)
            .iter()
            .map(|x| {
                (
                    *x,
                    Direction::iter()
                        .map(|dir| matrix.get_char_at(&dir.advance(x)))
                        .filter(|result| result.is_ok())
                        .map(|result| result.unwrap())
                        .filter(|result| *result == SLOT_FILE_PRESENT_CHAR)
                        .count(),
                )
            })
            .filter(|(_, adjacent_count)| *adjacent_count < 4usize)
            .map(|(x, _)| x)
            .collect();
        if removed.is_empty() {
            break;
        } else {
            removed_total += removed.len();
            for coordinate in removed {
                matrix.set_char(EMPTY_SLOT_CHAR, &coordinate);
            }
        }
    }

    removed_total.to_string()
}

fn parse_input(input: &str) -> CharMatrix {
    CharMatrix::from_str(input).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_day1() {
        let input = String::from(TEST_INPUT);

        let result = day4(&input);

        assert_eq!(String::from("13"), result);
    }

    #[test]
    fn test_day1_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day4_2(&input);

        assert_eq!(String::from("43"), result);
    }
}

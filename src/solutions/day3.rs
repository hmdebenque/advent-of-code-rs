use std::collections::{HashMap, HashSet};

use log::info;

#[derive(Hash, PartialEq, Eq, Debug)]
struct Coordinate {
    line: usize,
    column: usize,
}

impl Coordinate {
    fn new(line: usize, column: usize) -> Coordinate {
        Coordinate { line, column }
    }

    fn neighbours(&self) -> Vec<Coordinate> {
        let mut neighbours = Vec::new();
        if self.line > 0 {
            if self.column > 0 {
                // Top left
                neighbours.push(Coordinate::new(self.line - 1, self.column - 1));
            }
            // Top
            neighbours.push(Coordinate::new(self.line - 1, self.column));
            // Top-right
            neighbours.push(Coordinate::new(self.line - 1, self.column + 1));
        }
        if self.column > 0 {
            // Left
            neighbours.push(Coordinate::new(self.line, self.column - 1));
            // Bottom-left
            neighbours.push(Coordinate::new(self.line + 1, self.column - 1));
        }
        // Right
        neighbours.push(Coordinate::new(self.line, self.column + 1));
        // Bottom
        neighbours.push(Coordinate::new(self.line + 1, self.column));
        // Bottom-right
        neighbours.push(Coordinate::new(self.line + 1, self.column + 1));
        // And return the filled vector:
        neighbours
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct MarkedValue<T> {
    marked: bool,
    value: T,
}

impl<T> MarkedValue<T> {
    fn new(value: T) -> MarkedValue<T> {
        MarkedValue {
            marked: false,
            value,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn is_marked(&self) -> bool {
        self.marked
    }
}

pub fn day3(input: &String) -> String {
    let mut cur_line = String::new();
    let mut cur_num = String::new();
    let mut numbers = Vec::new();
    let mut numbers_coords_index: HashMap<Coordinate, usize> = HashMap::new();
    let mut symbol_coords: Vec<Coordinate> = Vec::new();
    let mut line = 0;
    let mut column: usize = 0;
    for char in input.chars() {
        // Complete numeric value or insert it into map
        if char.is_numeric() {
            cur_num.push(char);
        } else {
            if !cur_num.is_empty() {
                // Then handle numeric value pass
                // get number
                let parsed = cur_num.parse::<usize>().unwrap();
                // store value
                let value = MarkedValue::new(parsed);
                numbers.push(value);
                // store location with coordinate for indexing
                let val_index = numbers.len() - 1;
                for col in (column - cur_num.len() as usize)..column {
                    numbers_coords_index.insert(Coordinate::new(line, col), val_index);
                }
                cur_num = String::new();
            }
        }
        // And proceed with the non-numeric char handling
        if char == '\n' {
            line += 1;
            column = 0;
            cur_line = String::new();
        } else {
            if !(char == '.') && !char.is_numeric() {
                // This a symbol ! We need to check locations
                symbol_coords.push(Coordinate::new(line, column))
            }

            // At the end on a non new line
            column += 1;
        }
    }
    info!(
        "Finished parsing input, result are:\n{:?}\nand:\n{:?}",
        numbers_coords_index, symbol_coords
    );

    let mut coord_to_use = Vec::new();

    for coord in symbol_coords {
        for neighbour in coord.neighbours() {
            let loc = numbers_coords_index.get(&neighbour);
            if loc.is_some() {
                let val_opt = numbers.get_mut(*loc.unwrap());
                if val_opt.is_some() {
                    let mut val = val_opt.unwrap();
                    if !val.is_marked() {
                        coord_to_use.push(val.value);
                        val.mark();
                    }
                } else {
                    panic!("Should not be possible or index is corrupted.")
                }
            }
        }
    }

    info!("Found coordinates:\n{:?}", coord_to_use);

    coord_to_use.iter().map(|f| *f).sum::<usize>().to_string()
}

pub fn day3_2(input: &String) -> String {
    // Dirty copy paste, I know and am sorry
    let mut cur_line = String::new();
    let mut cur_num = String::new();
    let mut numbers = Vec::new();
    let mut numbers_coords_index: HashMap<Coordinate, usize> = HashMap::new();
    let mut symbol_coords: Vec<Coordinate> = Vec::new();
    let mut line = 0;
    let mut column: usize = 0;
    for char in input.chars() {
        // Complete numeric value or insert it into map
        if char.is_numeric() {
            cur_num.push(char);
        } else {
            if !cur_num.is_empty() {
                // Then handle numeric value pass
                // get number
                let parsed = cur_num.parse::<usize>().unwrap();
                // store value
                let value = MarkedValue::new(parsed);
                numbers.push(value);
                // store location with coordinate for indexing
                let val_index = numbers.len() - 1;
                for col in (column - cur_num.len() as usize)..column {
                    numbers_coords_index.insert(Coordinate::new(line, col), val_index);
                }
                cur_num = String::new();
            }
        }
        // And proceed with the non-numeric char handling
        if char == '\n' {
            line += 1;
            column = 0;
            cur_line = String::new();
        } else {
            if !(char == '.') && !char.is_numeric() {
                // This a symbol ! We need to check locations
                symbol_coords.push(Coordinate::new(line, column))
            }

            // At the end on a non new line
            column += 1;
        }
    }
    info!(
        "Finished parsing input, result are:\n{:?}\nand:\n{:?}",
        numbers_coords_index, symbol_coords
    );

    let mut coord_to_use: Vec<usize> = Vec::new();

    for coord in symbol_coords {
        let mut neighboring_vals_idx = HashSet::new();

        for neighbour in coord.neighbours() {
            let loc = numbers_coords_index.get(&neighbour);
            if loc.is_some() {
                neighboring_vals_idx.insert(*loc.unwrap());
            }
        }
        if neighboring_vals_idx.len() == 2 {
            coord_to_use.push(
                neighboring_vals_idx
                    .iter()
                    .map(|idx| numbers.get(*idx).unwrap().value)
                    .product(),
            )
        }
    }

    info!("Found coordinates:\n{:?}", coord_to_use);

    coord_to_use.iter().map(|f| *f).sum::<usize>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn part1() {
        let input = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!("4361", day3(&input));
    }

    #[test_log::test]
    fn part1_double_count() {
        let input = String::from(
            "467..114..
...*......
.=35..633.
......#...
617*......
.......58.
.+592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!("4361", day3(&input));
    }

    #[test_log::test]
    fn part2() {
        let input = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );

        assert_eq!("467835", day3_2(&input));
    }

    #[test_log::test]
    fn test_maps() {
        let mut strings: HashMap<usize, String> = HashMap::new();

        strings.insert(1, String::from("toto"));

        (*strings.get_mut(&1).unwrap()).push('s');

        let value = strings.get_mut(&1).unwrap();
        assert_eq!(String::from("totos"), *value);
    }
}

use crate::aoc_2024::day4::Direction::{NorthEast, SouthEast};
use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn day4(input: &String) -> String {
    let matrix = CharMatrix::parse(input);
    println!("Matrix parsed:\n{:?}", matrix);
    matrix.search_text().to_string()
}

#[derive(Debug)]
struct CharMatrix {
    matrix: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone)]
struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn new(x: isize, y: isize) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug)]
struct Rectangle {
    location: Coordinates,
    width: usize,
    height: usize,
}

impl Rectangle {
    fn new(location: Coordinates, width: usize, height: usize) -> Rectangle {
        Rectangle {
            location,
            width,
            height,
        }
    }

    fn in_bound(&self, location: &Coordinates) -> bool {
        location.x >= self.location.x
            && location.y >= self.location.y
            && location.x < (self.location.x + self.width as isize)
            && location.y < (self.location.y + self.height as isize)
    }
}

#[derive(EnumIter, Debug)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn advance(&self, coordinates: &Coordinates) -> Coordinates {
        match self {
            Direction::North => Coordinates::new(coordinates.x, coordinates.y - 1),
            Direction::NorthEast => Coordinates::new(coordinates.x + 1, coordinates.y - 1),
            Direction::East => Coordinates::new(coordinates.x + 1, coordinates.y),
            Direction::SouthEast => Coordinates::new(coordinates.x + 1, coordinates.y + 1),
            Direction::South => Coordinates::new(coordinates.x, coordinates.y + 1),
            Direction::SouthWest => Coordinates::new(coordinates.x - 1, coordinates.y + 1),
            Direction::West => Coordinates::new(coordinates.x - 1, coordinates.y),
            Direction::NorthWest => Coordinates::new(coordinates.x - 1, coordinates.y - 1),
        }
    }
}

impl CharMatrix {
    fn new() -> Self {
        CharMatrix { matrix: Vec::new() }
    }

    fn insert(&mut self, row: Vec<char>) {
        self.matrix.push(row);
    }

    pub fn parse(input: &String) -> Self {
        input
            .split("\n")
            .filter(|l| !l.is_empty())
            .fold(CharMatrix::new(), CharMatrix::parse_and_insert_line)
    }

    fn parse_and_insert_line(mut self, line: &str) -> Self {
        self.insert(line.chars().collect());
        self
    }

    fn search_text(&self) -> usize {
        let central_char = 'A';
        let search_direct = String::from("MAS");
        let search_reversed = String::from("SAM");

        let text_len = 3;
        let mut total_matchs = 0;
        for ordinate in 1..(self.matrix.len() - 1) {
            for abscissa in 1..(self.matrix[ordinate].len() - 1) {
                let char = self.matrix[ordinate][abscissa];
                if char == central_char {
                    println!("Found char at {},{}", ordinate, abscissa);

                    // first diagonal
                    let south_east = self.read_text(
                        &Coordinates::new((abscissa - 1) as isize, (ordinate - 1) as isize),
                        &SouthEast,
                        text_len,
                    );
                    // second diagonal
                    let north_east = self.read_text(
                        &Coordinates::new((abscissa - 1) as isize, (ordinate + 1) as isize),
                        &NorthEast,
                        text_len,
                    );
                    let x_mas_present = (south_east == search_direct
                        || south_east == search_reversed)
                        && (north_east == search_direct || north_east == search_reversed);
                    // println!("south_east={}, north_east={}, result={}",south_east, north_east, x_mas_present);
                    if x_mas_present {
                        total_matchs += 1;
                    }
                }
            }
        }
        total_matchs
    }

    fn read_text(&self, start: &Coordinates, dir: &Direction, len: usize) -> String {
        let mut buffer = String::new();
        let mut read_loc: Coordinates = start.clone();
        let search_bounds = self.get_bounds();
        for _i in 0..len {
            if !search_bounds.in_bound(&read_loc) {
                break;
            }
            buffer.push(self.read_value(&read_loc));
            read_loc = dir.advance(&read_loc);
        }
        buffer
    }

    fn read_value(&self, coordinates: &Coordinates) -> char {
        self.matrix[coordinates.y as usize][coordinates.x as usize]
    }

    fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            Coordinates::new(0, 0),
            self.matrix[0].len(), // breaks if no line
            self.matrix.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        let result = day4(&input);

        assert_eq!(String::from("18"), result);
    }

    #[test]
    fn test_day3_2() {
        let input = String::from(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
        );

        let result = day4(&input);

        assert_eq!(String::from("9"), result);
    }
}

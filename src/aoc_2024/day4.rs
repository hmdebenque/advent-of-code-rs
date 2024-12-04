use std::fmt::Debug;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn day4(input: &String) -> String {
    let matrix = CharMatrix::parse(input);
    println!("Matrix parsed:\n{:?}", matrix);
    matrix.search_text("XMAS").to_string()
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

    fn search_text(&self, text: &str) -> usize {
        let first_char = text.chars().next().unwrap();
        let text_len = text.len();
        let mut total_matchs = 0;
        for ordinate in 0..self.matrix.len() {
            for abscissa in 0..self.matrix[ordinate].len() {
                let char = self.matrix[ordinate][abscissa];
                if char == first_char {
                    println!("Found char at {},{}", ordinate, abscissa);

                    let matching_text_count = Direction::iter()
                        .map(|direction| {
                            self.read_text(
                                &Coordinates::new(abscissa as isize, ordinate as isize),
                                &direction,
                                text_len,
                            )
                        })
                        .filter(|s| text == s)
                        .count();
                    total_matchs += matching_text_count;
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
                println!("Read location out of boud: {:?}", read_loc);
                break;
            }
            buffer.push(self.read_value(&read_loc));
            read_loc = dir.advance(&read_loc);
        }
        println!("From {:?} with len {}, read: {}", start, len, buffer);
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
}

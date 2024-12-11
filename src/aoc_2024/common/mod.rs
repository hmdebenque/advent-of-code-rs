// Regroup code used in several puzzles

use crate::aoc_2024::common::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: isize,
    pub y: isize,
}

impl Coordinates {
    pub fn new(x: isize, y: isize) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}

impl Vector {
    pub fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }

    pub fn new_from_to(from: &Coordinates, to: &Coordinates) -> Vector {
        Vector {
            x: to.x - from.x,
            y: to.y - from.y,
        }
    }

    pub fn move_from(&self, from: &Coordinates) -> Coordinates {
        Coordinates::new(from.x + self.x, from.y + self.y)
    }

    pub fn reverse(&self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub location: Coordinates,
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn new(location: Coordinates, width: usize, height: usize) -> Rectangle {
        Rectangle {
            location,
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, location: &Coordinates) -> bool {
        location.x >= self.location.x
            && location.y >= self.location.y
            && location.x < (self.location.x + self.width as isize)
            && location.y < (self.location.y + self.height as isize)
    }
}

#[derive(EnumIter, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Direction {
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
    /// Returns the coordinate immediately next in this direction
    pub fn advance(&self, coordinates: &Coordinates) -> Coordinates {
        match self {
            North => Coordinates::new(coordinates.x, coordinates.y - 1),
            NorthEast => Coordinates::new(coordinates.x + 1, coordinates.y - 1),
            East => Coordinates::new(coordinates.x + 1, coordinates.y),
            SouthEast => Coordinates::new(coordinates.x + 1, coordinates.y + 1),
            South => Coordinates::new(coordinates.x, coordinates.y + 1),
            SouthWest => Coordinates::new(coordinates.x - 1, coordinates.y + 1),
            West => Coordinates::new(coordinates.x - 1, coordinates.y),
            NorthWest => Coordinates::new(coordinates.x - 1, coordinates.y - 1),
        }
    }

    pub fn right(&self) -> Direction {
        match self {
            North => East,
            NorthEast => SouthEast,
            East => South,
            SouthEast => SouthWest,
            South => West,
            SouthWest => NorthWest,
            West => North,
            NorthWest => NorthEast,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharMatrix {
    matrix: Vec<Vec<char>>,
}

impl CharMatrix {
    pub fn copy_filled(&self, replacement: char) -> CharMatrix {
        let mut new_matrix = crate::aoc_2024::common::CharMatrix::new();

        self.matrix
            .iter()
            .map(|row| row.iter().map(|_| replacement).collect())
            .for_each(|row| new_matrix.insert_row(row));
        new_matrix
    }

    pub fn print(&self) -> String {
        self.matrix
            .iter()
            .map(|row| row.iter().collect::<String>())
            .map(|row| format!("{}\n", row))
            .collect()
    }
}

impl FromStr for CharMatrix {
    type Err = ();

    // parse lines to create a matrix of chars
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(input
            .split("\n")
            .filter(|l| !l.is_empty())
            .fold(CharMatrix::new(), CharMatrix::parse_and_insert_line))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CharMatrixElement {
    pub coordinates: Coordinates,
    pub value: char,
}

impl CharMatrix {
    pub fn new() -> Self {
        CharMatrix { matrix: Vec::new() }
    }

    pub fn insert_row(&mut self, row: Vec<char>) {
        self.matrix.push(row);
    }

    pub fn insert_char(&mut self, value: char, location: Coordinates) {
        self.matrix[location.y as usize][location.x as usize] = value;
    }

    fn parse_and_insert_line(mut self, line: &str) -> Self {
        self.insert_row(line.chars().collect());
        self
    }

    /// get coordinates of the first char matching search
    pub fn search_char(&self, search: &char) -> Option<Coordinates> {
        for ordinate in 1..(self.matrix.len() - 1) {
            for abscissa in 1..(self.matrix[ordinate].len() - 1) {
                let char = self.matrix[ordinate][abscissa];
                if search.eq(&char) {
                    return Some(Coordinates::new(abscissa as isize, ordinate as isize));
                }
            }
        }
        None
    }

    pub fn search_chars(&self, search: &char) -> Vec<Coordinates> {
        self.get_all_chars()
            .iter()
            .filter(|c| search.eq(&c.value))
            .map(|c| c.coordinates)
            .collect()
    }

    pub fn get_char_at(&self, location: &Coordinates) -> Result<char, ()> {
        if self.get_bounds().is_in_bounds(location) {
            Ok(self.matrix[location.y as usize][location.x as usize])
        } else {
            Err(())
        }
    }

    pub fn get_all_chars(&self) -> Vec<CharMatrixElement> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(y, chars)| {
                chars
                    .iter()
                    .enumerate()
                    .map(|(x, ch)| CharMatrixElement {
                        coordinates: Coordinates::new(x as isize, y as isize),
                        value: *ch,
                    })
                    .collect::<Vec<CharMatrixElement>>()
            })
            .collect()
    }

    pub fn search_text(&self) -> usize {
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
            if !search_bounds.is_in_bounds(&read_loc) {
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

    // maybe should cache this if we have a performance problem
    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            Coordinates::new(0, 0),
            self.matrix[0].len(), // breaks if no line
            self.matrix.len(),
        )
    }

    pub fn is_in_bounds(&self, location: &Coordinates) -> bool {
        self.get_bounds().is_in_bounds(location)
    }
}

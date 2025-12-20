// Regroup code used in several puzzles

use crate::aoc_2024::common::Direction::{
    East, North, NorthEast, NorthWest, South, SouthEast, SouthWest, West,
};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates2D {
    pub x: isize,
    pub y: isize,
}

impl Coordinates2D {
    pub fn new(x: isize, y: isize) -> Coordinates2D {
        Coordinates2D { x, y }
    }

    pub fn advance(&self, direction: Direction) -> Coordinates2D {
        direction.advance(&self)
    }
}

impl FromStr for Coordinates2D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(",");
        let x_opt = split
            .next()
            .map(str::parse)
            .filter(Result::is_ok)
            .map(Result::unwrap);
        if x_opt.is_none() {
            return Err(());
        }
        let x: isize = x_opt.unwrap();
        let y_opt = split
            .next()
            .map(str::parse)
            .filter(Result::is_ok)
            .map(Result::unwrap);
        if y_opt.is_none() {
            return Err(());
        }
        let y: isize = y_opt.unwrap();
        Ok(Coordinates2D { x, y })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Segment {
    from: Coordinates2D,
    to: Coordinates2D,
}

impl Segment {
    pub fn new(from: Coordinates2D, to: Coordinates2D) -> Segment {
        Segment { from, to }
    }

    pub fn intersect(&self, other: &Segment) -> bool {
        // Cross product to determine orientation of triplet (p, q, r)
        // Returns: 0 = collinear, positive = clockwise, negative = counter-clockwise
        fn cross(p: &Coordinates2D, q: &Coordinates2D, r: &Coordinates2D) -> isize {
            (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x)
        }

        // Check if point q lies on segment pr (when collinear)
        fn on_segment(p: &Coordinates2D, q: &Coordinates2D, r: &Coordinates2D) -> bool {
            q.x >= p.x.min(r.x) && q.x <= p.x.max(r.x) && q.y >= p.y.min(r.y) && q.y <= p.y.max(r.y)
        }

        let p1 = &self.from;
        let q1 = &self.to;
        let p2 = &other.from;
        let q2 = &other.to;

        let d1 = cross(p1, q1, p2);
        let d2 = cross(p1, q1, q2);
        let d3 = cross(p2, q2, p1);
        let d4 = cross(p2, q2, q1);

        // General case: segments straddle each other
        if ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
        {
            return true;
        }

        // Special cases: collinear points
        if d1 == 0 && on_segment(p1, p2, q1) {
            return true;
        }
        if d2 == 0 && on_segment(p1, q2, q1) {
            return true;
        }
        if d3 == 0 && on_segment(p2, p1, q2) {
            return true;
        }
        if d4 == 0 && on_segment(p2, q1, q2) {
            return true;
        }

        false
    }

    /// Check if this segment intersects with any edge of the rectangle
    pub fn intersects_rectangle(&self, rect: &Rectangle) -> bool {
        rect.edges().iter().any(|edge| self.intersect(edge))
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

    pub fn new_from_to(from: &Coordinates2D, to: &Coordinates2D) -> Vector {
        Vector {
            x: to.x - from.x,
            y: to.y - from.y,
        }
    }

    pub fn move_from(&self, from: &Coordinates2D) -> Coordinates2D {
        Coordinates2D::new(from.x + self.x, from.y + self.y)
    }

    pub fn reverse(&self) -> Vector {
        Vector::new(-self.x, -self.y)
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub location: Coordinates2D,
    pub width: usize,
    pub height: usize,
}

impl Rectangle {
    pub fn new(location: Coordinates2D, width: usize, height: usize) -> Rectangle {
        Rectangle {
            location,
            width,
            height,
        }
    }

    pub fn from_bounds(location: Coordinates2D, location2: Coordinates2D) -> Rectangle {
        Rectangle {
            location: Coordinates2D::new(location.x.min(location2.x), location.y.min(location2.y)),
            width: location.x.abs_diff(location2.x) + 1,
            height: location.y.abs_diff(location2.y) + 1,
        }
    }

    pub fn is_in_bounds(&self, location: &Coordinates2D) -> bool {
        location.x >= self.location.x
            && location.y >= self.location.y
            && location.x < (self.location.x + self.width as isize)
            && location.y < (self.location.y + self.height as isize)
    }

    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Returns the 4 edges of the rectangle as segments
    pub fn edges(&self) -> [Segment; 4] {
        let x1 = self.location.x;
        let y1 = self.location.y;
        let x2 = self.location.x + self.width as isize - 1;
        let y2 = self.location.y + self.height as isize - 1;

        let top_left = Coordinates2D::new(x1, y1);
        let top_right = Coordinates2D::new(x2, y1);
        let bottom_left = Coordinates2D::new(x1, y2);
        let bottom_right = Coordinates2D::new(x2, y2);

        [
            Segment::new(top_left, top_right),       // Top edge
            Segment::new(top_right, bottom_right),   // Right edge
            Segment::new(bottom_right, bottom_left), // Bottom edge
            Segment::new(bottom_left, top_left),     // Left edge
        ]
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
    pub fn advance(&self, coordinates: &Coordinates2D) -> Coordinates2D {
        match self {
            North => Coordinates2D::new(coordinates.x, coordinates.y - 1),
            NorthEast => Coordinates2D::new(coordinates.x + 1, coordinates.y - 1),
            East => Coordinates2D::new(coordinates.x + 1, coordinates.y),
            SouthEast => Coordinates2D::new(coordinates.x + 1, coordinates.y + 1),
            South => Coordinates2D::new(coordinates.x, coordinates.y + 1),
            SouthWest => Coordinates2D::new(coordinates.x - 1, coordinates.y + 1),
            West => Coordinates2D::new(coordinates.x - 1, coordinates.y),
            NorthWest => Coordinates2D::new(coordinates.x - 1, coordinates.y - 1),
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
        let mut new_matrix = CharMatrix::new();

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
    pub coordinates: Coordinates2D,
    pub value: char,
}

impl CharMatrix {
    pub fn new() -> Self {
        CharMatrix { matrix: Vec::new() }
    }

    pub fn insert_row(&mut self, row: Vec<char>) {
        self.matrix.push(row);
    }

    pub fn set_char(&mut self, value: char, location: &Coordinates2D) {
        self.ensure_y(1 + location.y as usize);
        self.ensure_x(1 + location.x as usize);
        self.matrix[location.y as usize][location.x as usize] = value;
    }

    pub fn ensure_y(&mut self, height: usize) {
        if self.matrix.len() < height {
            for _ in self.matrix.len()..height {
                self.matrix.push(Vec::new());
            }
        }
    }

    pub fn ensure_x(&mut self, width: usize) {
        for row in self.matrix.iter_mut() {
            if row.len() < width {
                for _ in row.len()..width {
                    row.push('.');
                }
            }
        }
    }

    fn parse_and_insert_line(mut self, line: &str) -> Self {
        self.insert_row(line.chars().collect());
        self
    }

    /// get coordinates of the first char matching search
    pub fn search_char(&self, search: &char) -> Option<Coordinates2D> {
        for ordinate in 1..(self.matrix.len() - 1) {
            for abscissa in 1..(self.matrix[ordinate].len() - 1) {
                let char = self.matrix[ordinate][abscissa];
                if search.eq(&char) {
                    return Some(Coordinates2D::new(abscissa as isize, ordinate as isize));
                }
            }
        }
        None
    }

    pub fn search_chars(&self, search: &char) -> Vec<Coordinates2D> {
        self.get_all_chars()
            .iter()
            .filter(|c| search.eq(&c.value))
            .map(|c| c.coordinates)
            .collect()
    }

    pub fn get_char_at(&self, location: &Coordinates2D) -> Result<char, ()> {
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
                        coordinates: Coordinates2D::new(x as isize, y as isize),
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
                        &Coordinates2D::new((abscissa - 1) as isize, (ordinate - 1) as isize),
                        &SouthEast,
                        text_len,
                    );
                    // second diagonal
                    let north_east = self.read_text(
                        &Coordinates2D::new((abscissa - 1) as isize, (ordinate + 1) as isize),
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

    fn read_text(&self, start: &Coordinates2D, dir: &Direction, len: usize) -> String {
        let mut buffer = String::new();
        let mut read_loc: Coordinates2D = start.clone();
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

    fn read_value(&self, coordinates: &Coordinates2D) -> char {
        self.matrix[coordinates.y as usize][coordinates.x as usize]
    }

    // maybe should cache this if we have a performance problem
    pub fn get_bounds(&self) -> Rectangle {
        Rectangle::new(
            Coordinates2D::new(0, 0),
            self.matrix[0].len(), // breaks if no line
            self.matrix.len(),
        )
    }

    pub fn is_in_bounds(&self, location: &Coordinates2D) -> bool {
        self.get_bounds().is_in_bounds(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection_crossing() {
        // Horizontal and vertical segments crossing
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(8, 0));
        let segment2 = Segment::new(Coordinates2D::new(3, -1), Coordinates2D::new(3, 1));
        assert!(segment1.intersect(&segment2));
        assert!(segment2.intersect(&segment1)); // Symmetric
    }

    #[test]
    fn test_intersection_diagonal_crossing() {
        // Two diagonal segments that cross
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(4, 4));
        let segment2 = Segment::new(Coordinates2D::new(0, 4), Coordinates2D::new(4, 0));
        assert!(segment1.intersect(&segment2));
    }

    #[test]
    fn test_intersection_no_intersect_parallel() {
        // Parallel segments that don't intersect
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(4, 0));
        let segment2 = Segment::new(Coordinates2D::new(0, 2), Coordinates2D::new(4, 2));
        assert!(!segment1.intersect(&segment2));
    }

    #[test]
    fn test_intersection_no_intersect_apart() {
        // Non-parallel segments that don't intersect
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(2, 0));
        let segment2 = Segment::new(Coordinates2D::new(3, 1), Coordinates2D::new(5, 3));
        assert!(!segment1.intersect(&segment2));
    }

    #[test]
    fn test_intersection_collinear_overlapping() {
        // Collinear overlapping segments
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(4, 0));
        let segment2 = Segment::new(Coordinates2D::new(2, 0), Coordinates2D::new(6, 0));
        assert!(segment1.intersect(&segment2));
    }

    #[test]
    fn test_intersection_shared_endpoint() {
        // Segments sharing an endpoint
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(2, 2));
        let segment2 = Segment::new(Coordinates2D::new(2, 2), Coordinates2D::new(4, 0));
        assert!(segment1.intersect(&segment2));
    }

    #[test]
    fn test_intersection_t_junction() {
        // T-junction: one segment ends on another
        let segment1 = Segment::new(Coordinates2D::new(0, 0), Coordinates2D::new(4, 0));
        let segment2 = Segment::new(Coordinates2D::new(2, 0), Coordinates2D::new(2, 3));
        assert!(segment1.intersect(&segment2));
    }
}

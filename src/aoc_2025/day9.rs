use crate::aoc_2024::common::{CharMatrix, Coordinates2D, Rectangle, Segment};
use clap::builder::Str;
#[cfg(not(test))]
use log::info;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
// Solution

pub fn day9(input: &String) -> String {
    let tiles: Vec<Coordinates2D> = input
        .split("\n")
        .map(Coordinates2D::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    print_as_matrix(&tiles);

    let mut rect = build_rect_from_red_tiles(&tiles);

    rect.iter().map(Rectangle::area).max().unwrap().to_string()
}

pub fn day9_2(input: &String) -> String {
    let tiles: Vec<Coordinates2D> = input
        .split("\n")
        .map(Coordinates2D::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .collect();

    print_as_matrix(&tiles);

    let mut rectangles: Vec<Rectangle> = build_rect_from_red_tiles(&tiles);

    rectangles.retain(|r| {
        for i in 1..tiles.len() {
            let path_segment = Segment::new(tiles[i - 1], tiles[i]);
            if path_segment.intersects_rectangle(r) {
                return false;
            }
        }
        true
    });

    rectangles
        .iter()
        .map(Rectangle::area)
        .max()
        .unwrap()
        .to_string()
}

fn print_as_matrix(tiles: &Vec<Coordinates2D>) {
    let mut matrix = CharMatrix::new();
    for tile in tiles {
        matrix.set_char('#', tile);
    }
    let matrix_str = matrix.print();
    info!("Matrix:\n{matrix_str}");
}

fn contains_inside(rect: &Rectangle, tiles: &Vec<Coordinates2D>) -> bool {
    tiles.iter().any(|t| rect.is_in_bounds_exclusive(t))
}

fn build_rect_from_red_tiles(tiles: &Vec<Coordinates2D>) -> Vec<Rectangle> {
    let mut rect: Vec<Rectangle> = Vec::new();

    for i in 0..tiles.len() {
        let first = tiles[i];
        for j in i..tiles.len() {
            let second = tiles[j];
            rect.push(Rectangle::from_bounds(first, second));
        }
    }
    rect
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_day8() {
        let input = String::from(TEST_INPUT);

        let result = day9(&input);

        assert_eq!(String::from("50"), result);
    }

    #[test]
    fn test_day8_2() {
        let input = String::from(TEST_INPUT);

        let result = day9_2(&input);

        assert_eq!(String::from("24"), result);
    }
}

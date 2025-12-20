use crate::aoc_2024::common::Direction::{East, North, South, West};
use crate::aoc_2024::common::{CharMatrix, Coordinates2D, Direction};
use std::collections::HashSet;
use std::fmt::Debug;
use std::str::FromStr;

pub fn day6(input: &String) -> String {
    let map = CharMatrix::from_str(input).unwrap();

    // now we have our map and guards, let's draw our patrol

    let visited_locations = get_guard_path(&map);

    visited_locations.len().to_string()
}

fn get_guard_path(map: &CharMatrix) -> HashSet<Coordinates2D> {
    let mut guard = find_guard(&map);
    let mut visited_locations: HashSet<Coordinates2D> = HashSet::new();
    let coordinates = guard.location.clone();
    visited_locations.insert(coordinates);
    while map.is_in_bounds(&guard.location) {
        let next_step = guard.next_step();
        let result = map.get_char_at(&next_step);
        if result.is_err() {
            println!("Guard went out of bound: {:?}", guard);
            break;
        }
        let next_step_content = result.unwrap();
        match next_step_content {
            '#' => guard.turn_right(),
            _ => guard.advance(),
        }
        println!("Guard located: {:?}", guard);
        visited_locations.insert(guard.location.clone());
    }
    visited_locations
}

fn find_guard(map: &CharMatrix) -> Guard {
    let guard_char = map
        .search_char(&'^')
        .or_else(|| {
            map.search_char(&'>')
                .or_else(|| map.search_char(&'v').or_else(|| map.search_char(&'<')))
        })
        .unwrap();

    let guard = Guard::new(
        guard_char,
        Direction::from(map.get_char_at(&guard_char).unwrap()),
    );
    guard
}

const MAP_OBJECTS: [char; 5] = ['#', '^', '>', 'v', '<'];

pub fn day6_2(input: &String) -> String {
    let map = CharMatrix::from_str(input).unwrap();
    let guard = find_guard(&map);
    println!("Guard found: {:?}", guard);

    // get guard path to avoid repetition
    let visited_locations = get_guard_path(&map);

    // now we have our map and guards, let's drw our patrol
    let mut loop_count: usize = 0;
    // This is brute forcing, we could do it by just checking location on the guard path.
    for coordinates in visited_locations {
        let at_loc = map.get_char_at(&coordinates).unwrap();
        if !MAP_OBJECTS.contains(&at_loc) {
            // We crate a map copy and test if there is a loop
            let mut map_copy = map.clone();
            map_copy.set_char('#', &coordinates);
            if is_map_loop(&map_copy, &guard) {
                log::info!("Loop created by placing element at {:?}", coordinates);
                loop_count += 1;
            }
        }
    }
    loop_count.to_string()
}

fn is_map_loop(map: &CharMatrix, guard_original: &Guard) -> bool {
    // mutable copy to move her around
    let mut guard: Guard = guard_original.clone();

    let mut visited_locations: HashSet<(Coordinates2D, Direction)> = HashSet::new();
    visited_locations.insert((guard.location.to_owned(), guard.direction.to_owned()));
    while map.is_in_bounds(&guard.location) {
        let next_step = guard.next_step();
        let result = map.get_char_at(&next_step);
        if result.is_err() {
            // guard is out of map, we are not in a loop
            return false;
        }
        let next_step_content = result.unwrap();
        match next_step_content {
            '#' => guard.turn_right(),
            _ => guard.advance(),
        }
        // check looping
        let guard_new_position = (guard.location.to_owned(), guard.direction.to_owned());
        if visited_locations.contains(&guard_new_position) {
            return true;
        }
        visited_locations.insert(guard_new_position);
    }

    false
}

#[derive(Debug, Copy, Clone)]
struct Guard {
    location: Coordinates2D,
    direction: Direction,
}

impl Guard {
    fn new(location: Coordinates2D, direction: Direction) -> Self {
        Guard {
            location,
            direction,
        }
    }

    /// coordinates in front of the guard
    fn next_step(&self) -> Coordinates2D {
        self.direction.advance(&self.location)
    }

    pub fn advance(&mut self) {
        self.location = self.next_step();
    }

    pub fn turn_right(&mut self) {
        self.direction = self.direction.right()
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => North,
            '>' => East,
            'v' => South,
            '<' => West,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &'static str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn test_day6() {
        let input = String::from(PUZZLE_INPUT);

        let result = day6(&input);

        assert_eq!(String::from("41"), result);
    }

    #[test]
    fn test_day6_2() {
        let input = String::from(PUZZLE_INPUT);
        let result = day6_2(&input);

        assert_eq!(String::from("6"), result);
    }
}

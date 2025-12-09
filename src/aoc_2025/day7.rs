use crate::aoc_2024::common::Direction::{East, South, West};
use crate::aoc_2024::common::{CharMatrix, Coordinates, Direction};
#[cfg(not(test))]
use log::info;
use std::collections::HashSet;
#[cfg(test)]
use std::println as info;
use std::str::FromStr;
use std::time::Instant;

pub fn day7(input: &String) -> String {
    let map = CharMatrix::from_str(input).unwrap();
    let map_bounds = map.get_bounds();

    let map_str = map.print();
    info!("Map:\n{map_str}");

    let mut beams: HashSet<Coordinates> = map.search_chars(&'S').into_iter().collect();
    let mut nb_of_split = 0;
    loop {
        beams = beams
            .iter()
            .map(|b| b.advance(South))
            .filter(|b| map_bounds.is_in_bounds(b))
            .flat_map(|b| {
                if map.get_char_at(&b).unwrap().eq(&'^') {
                    // split
                    info!("Split happened!");
                    nb_of_split += 1;
                    vec![b.advance(West), b.advance(East)]
                } else {
                    vec![b]
                }
            })
            .filter(|b| map_bounds.is_in_bounds(b))
            .collect();
        if beams.is_empty() {
            break;
        }
    }

    nb_of_split.to_string()
}

#[derive(Debug)]
struct Beam {
    coordinates: Coordinates,
    combi: usize,
}

impl Beam {
    fn advance(&self, direction: Direction) -> Beam {
        Beam {
            coordinates: self.coordinates.advance(direction),
            combi: self.combi,
        }
    }

    fn fuse(&mut self, other: Beam) {
        self.combi = self.combi + other.combi;
    }

    fn new(coordinates: Coordinates) -> Beam {
        Beam {
            coordinates,
            combi: 1,
        }
    }
}

pub fn day7_2(input: &String) -> String {
    let map = CharMatrix::from_str(input).unwrap();

    let map_str = map.print();
    info!("Map:\n{map_str}");

    let mut beams: Vec<Beam> = map
        .search_chars(&'S')
        .into_iter()
        .map(|x| Beam::new(x))
        .collect();

    let mut nb_of_split = 1;
    let mut loop_nb = 0;
    loop {
        let start_time = Instant::now();
        let (beams_advanced, splits) = advance_beams(&map, beams);
        beams = beams_advanced;
        nb_of_split += splits;

        let mut matrix_clone = map.clone();
        for beam in &beams {
            matrix_clone.set_char('|', &beam.coordinates);
        }

        let duration_milli = start_time.elapsed().as_millis();

        info!("beams {beams:?}");
        info!(
            "Iter {}, beams {} advanced:{}",
            loop_nb,
            beams.len(),
            duration_milli
        );

        if beams.is_empty() {
            break;
        }
        loop_nb += 1;
        if loop_nb > 1000 {
            break;
        }
    }

    nb_of_split.to_string()
}

fn advance_beams(map: &CharMatrix, beams: Vec<Beam>) -> (Vec<Beam>, usize) {
    let map_bounds = map.get_bounds();
    let mut beams_advanced: Vec<Beam> = Vec::with_capacity(beams.len() * 2);
    let mut splits = 0;

    for beam in beams
        .iter()
        .map(|b| b.advance(South))
        .filter(|b| map_bounds.is_in_bounds(&b.coordinates))
    {
        if map.get_char_at(&beam.coordinates).unwrap().eq(&'^') {
            splits += beam.combi;
            let west = beam.advance(West);
            let east = beam.advance(East);
            if map_bounds.is_in_bounds(&west.coordinates) {
                let mut existing: Vec<&mut Beam> = beams_advanced
                    .iter_mut()
                    .filter(|x| x.coordinates == west.coordinates)
                    .collect();
                if existing.is_empty() {
                    beams_advanced.push(west);
                } else {
                    let val = existing.first_mut().unwrap();
                    val.fuse(west);
                }
            }
            if map_bounds.is_in_bounds(&east.coordinates) {
                let mut existing: Vec<&mut Beam> = beams_advanced
                    .iter_mut()
                    .filter(|x| x.coordinates == east.coordinates)
                    .collect();
                if existing.is_empty() {
                    beams_advanced.push(east);
                } else {
                    let val = existing.first_mut().unwrap();
                    val.fuse(east);
                }
            }
        } else {
            beams_advanced.push(beam);
        }
    }

    (beams_advanced, splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";

    #[test]
    fn test_day7() {
        let input = String::from(TEST_INPUT);

        let result = day7(&input);

        assert_eq!(String::from("21"), result);
    }

    #[test]
    fn test_day7_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day7_2(&input);

        assert_eq!(String::from("40"), result);
    }
}

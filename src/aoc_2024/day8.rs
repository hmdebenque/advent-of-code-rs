use crate::aoc_2024::common::{CharMatrix, Coordinates, Rectangle, Vector};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub fn day8(input: &String) -> String {
    find_antinodes_count(input, false)
}

pub fn day8_2(input: &String) -> String {
    find_antinodes_count(input, true)
}

fn find_antinodes_count(input: &String, include_resonating: bool) -> String {
    let map = CharMatrix::from_str(input).unwrap();
    let distinct: HashMap<char, Vec<Coordinates>> = map
        .get_all_chars()
        .iter()
        .map(|x| x.to_owned())
        .filter(|cme| cme.value != '.')
        .fold(HashMap::new(), |mut acc, char_mat_elem| {
            let hash_map_vec = acc.get_mut(&char_mat_elem.value);
            if hash_map_vec.is_some() {
                hash_map_vec.unwrap().push(char_mat_elem.coordinates);
            } else {
                acc.insert(char_mat_elem.value, vec![char_mat_elem.coordinates]);
            }
            acc
        });
    println!("Distinct values: {:?}", distinct);
    let maps_bounds = map.get_bounds();

    distinct
        .iter()
        .map(|(char, coords)| {
            println!("Searching all antinodes for {}: {:?}", char, coords);
            let vec = coords
                .iter()
                .flat_map(|coord| {
                    let vec1 = coords
                        .iter()
                        .filter(|x| **x != *coord)
                        .flat_map(|x| get_antinodes(coord, x, &maps_bounds, include_resonating))
                        .collect::<Vec<Coordinates>>();
                    vec1
                })
                .filter(|c| map.is_in_bounds(c))
                .collect::<HashSet<Coordinates>>();
            println!("Antinodes for {}: {:?}", char, vec);
            vec
        })
        .fold(map.copy_filled('.'), |mut acc, set| {
            set.iter().for_each(|c| {
                acc.set_char('X', c);
            });
            acc
        })
        .get_all_chars()
        .iter()
        .filter(|cme| cme.value == 'X')
        .count()
        .to_string()
}
fn get_antinodes(
    c1: &Coordinates,
    c2: &Coordinates,
    bounds: &Rectangle,
    include_resonating: bool,
) -> Vec<Coordinates> {
    if include_resonating {
        get_antinodes_resonating(c1, c2, bounds)
    } else {
        get_antinodes_non_resonating(c1, c2)
    }
}

fn get_antinodes_non_resonating(c1: &Coordinates, c2: &Coordinates) -> Vec<Coordinates> {
    let from_c1_to_c2 = Vector::new_from_to(c1, c2);

    vec![
        from_c1_to_c2.reverse().move_from(c1),
        from_c1_to_c2.move_from(c2),
    ]
}

fn get_antinodes_resonating(
    c1: &Coordinates,
    c2: &Coordinates,
    bounds: &Rectangle,
) -> Vec<Coordinates> {
    let from_c1_to_c2 = Vector::new_from_to(c1, c2);
    let from_c2_to_c1 = from_c1_to_c2.reverse();

    let mut coords = Vec::new();

    let mut antinode_c1 = c1.to_owned();
    while bounds.is_in_bounds(&antinode_c1) {
        coords.push(antinode_c1);
        antinode_c1 = from_c2_to_c1.move_from(&antinode_c1);
    }

    let mut antinode_c2 = c2.to_owned();
    while bounds.is_in_bounds(&antinode_c2) {
        coords.push(antinode_c2);
        antinode_c2 = from_c1_to_c2.move_from(&antinode_c2);
    }
    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn test_day8() {
        let input = String::from(PUZZLE_INPUT);

        let result = day8(&input);

        assert_eq!(String::from("14"), result);
        //    0 1 2 3 4 5 6 7 8 9 10 11
        // 0  . . . . . . # . . . .  #
        // 1  . . . # . . . . 0 . .  .
        // 2  . . . . # 0 . . . . #  .
        // 3  . . # . . . . 0 . . .  .
        // 4  . . . . 0 . . . . # .  .
        // 5  . # . . . . A . . . .  .
        // 6  . . . # . . . . . . .  .
        // 7  # . . . . . . # . . .  .
        // 8  . . . . . . . . A . .  .
        // 9  . . . . . . . . . A .  .
        // 10 . . . . . . . . . . #  .
        // 11 . . . . . . . . . . #  .

        //    0 1 2 3 4  5 6 7 8 9 10 11
        // 0  . . . . .  . 4 . . . .  10
        // 1  . . . 5 .  . . . 0 . .  .
        // 2  . . . . 11 0 . . . . 9  .
        // 3  . . 6 . .  . . 0 . . .  .
        // 4  . . . . 0  . . . . 3 .  .
        // 5  . 8 . . .  . 7 . . . .  .
        // 6  . . . 2 .  . . . . . .  .
        // 7  1 . . . .  . . 13 . . .  .
        // 8  . . . . .  . . . A . .  .
        // 9  . . . . .  . . . . A .  .
        // 10 . . . . .  . . . . . 12  .
        // 11 . . . . .  . . . . . 14  .

        // Antinodes for 0: {Coordinates { x: 0, y: 7 },
        //                   Coordinates { x: 3, y: 6 },
        //                   Coordinates { x: 9, y: 4 },
        //                   Coordinates { x: 6, y: 0 },
        //                   Coordinates { x: 3, y: 1 },
        //                   Coordinates { x: 2, y: 3 },
        //                   Coordinates { x: 6, y: 5 },
        //                   Coordinates { x: 1, y: 5 },
        //                   Coordinates { x: 10, y: 2 },
        //                   Coordinates { x: 11, y: 0 }}
        // Antinodes for A: {Coordinates { x: 4, y: 2 },
        //                   Coordinates { x: 10, y: 10 },
        //                   Coordinates { x: 7, y: 7 },
        //                   Coordinates { x: 10, y: 11 },
        //                   Coordinates { x: 3, y: 1 }}
    }

    #[test]
    fn test_day8_2() {
        let input = String::from(PUZZLE_INPUT);

        let result = day8_2(&input);

        assert_eq!(String::from("34"), result);
    }

    #[test]
    fn test_day8_2_tees() {
        let input = String::from(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        );

        let result = day8_2(&input);

        assert_eq!(String::from("9"), result);
    }

    #[test]
    fn test_get_antinodes() {
        let c1 = Coordinates::new(2, 3);
        let c2 = Coordinates::new(4, 5);
        //0 ......
        //1 #.....
        //2 ......
        //3 ..X...
        //4 ......
        //5 ....X.
        //6 ......
        //7 ......#
        //8 ......

        let antinodes = get_antinodes_non_resonating(&c1, &c2);

        assert_eq!(antinodes.len(), 2);
        assert_eq!(antinodes[0], Coordinates::new(0, 1));
        assert_eq!(antinodes[1], Coordinates::new(6, 7));
    }

    #[test]
    fn test_get_antinodes_a() {
        let c1 = Coordinates::new(8, 8);
        let c2 = Coordinates::new(9, 9);

        let antinodes = get_antinodes_non_resonating(&c1, &c2);

        assert_eq!(antinodes.len(), 2);
        assert_eq!(antinodes[0], Coordinates::new(7, 7));
        assert_eq!(antinodes[1], Coordinates::new(10, 10));
    }
}

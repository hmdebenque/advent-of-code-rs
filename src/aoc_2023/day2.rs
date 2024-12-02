use log::{info, warn};
use std::str::FromStr;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Set {
    red: u8,
    blue: u8,
    green: u8,
}
impl Set {
    fn new(red: u8, blue: u8, green: u8) -> Set {
        return Set { red, blue, green };
    }

    fn power(&self) -> u16 {
        fn one_when_zero(value: u8) -> u16 {
            if value == 0 {
                1
            } else {
                value as u16
            }
        }
        let res = one_when_zero(self.red) * one_when_zero(self.blue) * one_when_zero(self.green);
        println!("power for {:?} = {:?}", self, res);
        res
    }
}
impl FromStr for Set {
    /// Expects string as "1 red, 2 green, 6 blue"
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut red: u8 = 0;
        let mut blue: u8 = 0;
        let mut green: u8 = 0;

        for color_block in input.split(", ") {
            let mut block_split = color_block.split(" ");
            let fst_part = block_split.next();
            if fst_part.is_none() {
                warn!("Invalid set pattern: {color_block}");
                continue;
            } else {
                let scd_part = block_split.next();
                if scd_part.is_none() {
                    warn!("Invalid set pattern: {color_block}");
                    continue;
                }
                let fst_part_parsed = fst_part.unwrap().parse();
                if fst_part_parsed.is_err() {
                    warn!("Invalid set pattern: {color_block}");
                    continue;
                }
                let nbr = fst_part_parsed.unwrap();
                let color = scd_part.unwrap();
                if color.eq("red") {
                    red = nbr;
                } else if color.eq("blue") {
                    blue = nbr;
                } else if color.eq("green") {
                    green = nbr;
                } else {
                    warn!("Invalid set pattern: {color_block}");
                    continue;
                }
            }
        }
        Ok(Set::new(red, blue, green))
    }

    type Err = String;
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Game {
    id: u16,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u16, sets: Vec<Set>) -> Game {
        return Game { id, sets };
    }

    /// Get the minimum set needed for this game
    fn min_set(&self) -> Set {
        let mut red: u8 = 0;
        let mut blue: u8 = 0;
        let mut green: u8 = 0;
        for set in &self.sets {
            if set.red > red {
                red = set.red
            }
            if set.blue > blue {
                blue = set.blue
            }
            if set.green > green {
                green = set.green
            }
        }
        let res = Set::new(red, blue, green);
        println!("min set for {:?} = {:?}", self.sets, res);
        res
    }
}

impl FromStr for Game {
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let capt = Regex::new(r"^Game (?<id>\d+):(?<sets>(( \d+ (red|green|blue),?)+;?)+)$")
            .unwrap()
            .captures(s);
        if capt.is_none() {
            return Err(format!("invalid variant: {s}"));
        }
        let captures = capt.unwrap();
        let id_found = captures
            .name("id")
            .unwrap()
            .as_str()
            .parse::<u16>()
            .unwrap();

        let sets_found = captures.name("sets").unwrap().as_str();
        warn!("sets found: {sets_found}");
        let sets: Vec<Set> = sets_found
            .split(";")
            .map(str::trim)
            .map(Set::from_str)
            .filter(|res| res.is_ok())
            .map(Result::unwrap)
            .collect();

        Ok(Game::new(id_found, sets))
    }

    type Err = String;
}

pub fn day2(input: &String) -> String {
    // Puzzle constants
    // 12 red cubes, 13 green cubes, and 14 blue cubes
    const MAX_RED: u8 = 12;
    const MAX_GREEN: u8 = 13;
    const MAX_BLUE: u8 = 14;

    input
        .split("\n")
        .map(Game::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .inspect(|g| info!("Found and parsed game: {:?}", g))
        .filter(|g| {
            let ok = g
                .sets
                .iter()
                .all(|s| s.red <= MAX_RED && s.green <= MAX_GREEN && s.blue <= MAX_BLUE);
            info!("Sets {:?} is {:?}", g.sets, ok);
            ok
        })
        .inspect(|g| info!("Game was deemed OK: {:?}", g))
        .map(|g| g.id)
        .sum::<u16>()
        .to_string()
}

pub fn day2_2(input: &String) -> String {
    input
        .split("\n")
        .map(Game::from_str)
        .filter(Result::is_ok)
        .map(Result::unwrap)
        .map(|g| g.min_set())
        .map(|s| s.power())
        .map(|p| p as u32)
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_parse() {
        assert_eq!(
            Set::new(1, 4, 3),
            Set::from_str("3 green, 4 blue, 1 red").unwrap()
        );
    }

    #[test]
    fn game_parse() {
        assert_eq!(
            Game::new(1, vec!(Set::new(1, 4, 3), Set::new(3, 0, 32))),
            Game::from_str("Game 1: 3 green, 4 blue, 1 red; 3 red, 32 green").unwrap()
        );
    }

    #[test]
    fn part1() {
        let input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!("8", day2(&input));
    }

    #[test]
    fn game_min_set() {
        assert_eq!(
            Game::new(1, vec![Set::new(1, 4, 3), Set::new(3, 0, 32)]).min_set(),
            Set::new(3, 4, 32)
        );
        assert_eq!(
            Game::new(
                3,
                vec![Set::new(20, 6, 8), Set::new(4, 5, 13), Set::new(5, 0, 5)]
            )
            .min_set(),
            Set::new(20, 6, 13)
        );
    }

    #[test]
    fn set_power() {
        assert_eq!(Set::new(1, 3, 4).power(), 12);
        assert_eq!(Set::new(20, 13, 6).power(), 1560);
        assert_eq!(Set::new(0, 3, 6).power(), 18);
    }

    #[test]
    fn part2() {
        let input = String::from(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );

        assert_eq!("2286", day2_2(&input));
    }
}

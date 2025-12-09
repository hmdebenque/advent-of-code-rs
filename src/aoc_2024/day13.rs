use crate::aoc_2024::common::{CharMatrix, Coordinates, Vector};
#[cfg(not(test))]
use log::{info};
// Use log crate when building application
use std::str::FromStr;
#[cfg(test)]
use std::{println as info};

pub fn day13(input: &String) -> String {
    let machines = parse_machines(input);
    info!("Parsed input:\n{:?}", machines);
    machines
        .iter()
        .filter_map(Machine::solve)
        .sum::<usize>()
        .to_string()
}

pub fn day13_2(input: &String) -> String {
    let char_matrix = CharMatrix::from_str(input).unwrap();
    info!("Parsed input:\n{}", char_matrix.print());
    String::from("NYI")
}

fn parse_machines(input: &String) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(Machine::from_str)
        .map(Result::unwrap)
        .collect()
}

#[derive(Debug, Clone)]
struct Machine {
    a_button: Vector,
    b_button: Vector,
    prize_loc: Coordinates,
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("\n");

        let vector_a = Self::parse_button(split.next().unwrap());
        let vector_b = Self::parse_button(split.next().unwrap());

        let prize = split.next();
        let mut prize_iter = prize.unwrap().split(' ').skip(1);
        let prize_x: isize = prize_iter.next().unwrap()[2..]
            .strip_suffix(",")
            .unwrap()
            .parse()
            .unwrap();
        let prize_y: isize = prize_iter.next().unwrap()[2..].parse().unwrap();

        Ok(Machine::new(
            vector_a,
            vector_b,
            Coordinates::new(prize_x, prize_y),
        ))
    }
}

impl Machine {
    fn new(a_button: Vector, b_button: Vector, prize_loc: Coordinates) -> Machine {
        Machine {
            a_button,
            b_button,
            prize_loc,
        }
    }

    /// X * a.x + Y * b.x = p.x
    /// X * a.y + Y * b.y = p.y
    fn solve(&self) -> Option<usize> {
        let deter = self.a_button.x * self.b_button.y - self.b_button.x * self.a_button.y;
        if deter == 0 {
            return None;
        }
        let x = (self.prize_loc.x * self.b_button.y - self.prize_loc.y * self.b_button.x) / deter;
        let x_rem =
            (self.prize_loc.x * self.b_button.y - self.prize_loc.y * self.b_button.x) % deter;
        if x_rem != 0 {
            return None;
        }
        let y = (self.prize_loc.y * self.a_button.x - self.prize_loc.x * self.a_button.y) / deter;
        let y_rem =
            (self.prize_loc.y * self.a_button.x - self.prize_loc.x * self.a_button.y) % deter;
        if y_rem != 0 {
            return None;
        }
        Some(x as usize * 3 + y as usize)
    }

    fn parse_button(button_raw: &str) -> Vector {
        let mut split_a = button_raw.split(" ").skip(2);
        let option = split_a.next();
        let split_a_x_str = option.unwrap().strip_suffix(",").unwrap();
        let split_a_x: isize = split_a_x_str[2..].parse().unwrap();
        let split_a_y_str = split_a.next().unwrap();
        let split_a_y: isize = split_a_y_str[2..].parse().unwrap();
        Vector::new(split_a_x, split_a_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT_SMALL: &'static str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    #[test_log::test]
    #[ignore]
    fn test_day13_small() {
        let input = String::from(PUZZLE_INPUT_SMALL);

        let result = day13(&input);

        assert_eq!(String::from("140"), result);
    }
}

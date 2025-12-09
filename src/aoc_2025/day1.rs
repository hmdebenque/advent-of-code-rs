use log::info;
use std::isize;
use std::str::FromStr;

pub fn day1(input: &String) -> String {
    let inputs = parse_input(input);

    let mut dial: isize = 50;
    let mut zero_touched = 0;
    for combi in inputs {
        if combi.starts_with("L") {
            let option = combi
                .strip_prefix("L")
                .map(|s| isize::from_str(s))
                .map(|r| r.unwrap())
                .unwrap();
            dial = (dial - option) % 100;
        } else if combi.starts_with("R") {
            let option = combi
                .strip_prefix("R")
                .map(|s| isize::from_str(s))
                .map(|r| r.unwrap())
                .unwrap();
            dial = (dial + option) % 100;
        }

        info!("Instruction: {combi}, dial={dial}");
        if dial == 0 {
            zero_touched += 1;
        }
    }

    zero_touched.to_string()
}

pub fn day1_2(input: &String) -> String {
    let inputs = parse_input(input);

    let mut dial: isize = 50;
    let mut zero_touched = 0;
    for combi in inputs {
        if combi.starts_with("L") {
            let option = combi
                .strip_prefix("L")
                .map(|s| isize::from_str(s))
                .map(|r| r.unwrap())
                .unwrap_or_else(|| {
                    panic!("Cannot parse input {combi}");
                });
            for _ in 0..option {
                dial = if dial == 0 { 99 } else { dial - 1 };
                if dial == 0 {
                    zero_touched += 1;
                    info!("Zero touched");
                }
            }
        } else {
            let option = combi
                .strip_prefix("R")
                .map(|s| isize::from_str(s))
                .map(|r| r.unwrap())
                .unwrap_or_else(|| {
                    panic!("Cannot parse input {combi}");
                });
            for _ in 0..option {
                dial = if dial == 99 { 0 } else { dial + 1 };
                if dial == 0 {
                    zero_touched += 1;
                    info!("Zero touched");
                }
            }
        };

        info!("Instruction: {combi}, dial={dial}");
    }

    zero_touched.to_string()
}

fn parse_input(input: &str) -> Vec<&str> {
    input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_day1() {
        let input = String::from(TEST_INPUT);

        let result = day1(&input);

        assert_eq!(String::from("3"), result);
    }

    #[test]
    fn test_day1_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day1_2(&input);

        assert_eq!(String::from("6"), result);
    }
}

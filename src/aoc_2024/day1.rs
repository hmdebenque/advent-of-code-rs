pub fn day1(input: &String) -> String {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let sum: usize = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    sum.to_string()
}

pub fn day1_2(_input: &String) -> String {
    String::from("not yet implemented")
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .split("\n")
        .map(|l| l.trim())
        .map(|l| parse_line(l))
        .filter(|l| l.is_some())
        .map(|l| l.unwrap())
        .fold(
            (Vec::new(), Vec::new()),
            |mut acc: (Vec<usize>, Vec<usize>), val| {
                acc.0.push(val.0);
                acc.1.push(val.1);
                acc
            },
        )
}

fn parse_line(l: &str) -> Option<(usize, usize)> {
    l.split_once(" ")
        .map(|(left, right)| (left.trim().parse().unwrap(), right.trim().parse().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1() {
        let input = String::from(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );

        let result = day1(&input);

        assert_eq!(String::from("11"), result);
    }

    #[test]
    fn test_day1_2() {
        let input = String::from(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );

        let result = day1_2(&input);

        assert_eq!(String::from("281"), result);
    }
}

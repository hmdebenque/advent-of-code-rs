use ranges::{GenericRange, Ranges};
use std::ops::{Bound, RangeBounds};

pub fn day5(input: &String) -> String {
    let (ranges, ids) = parse_input(input);

    ids.iter()
        .filter(|id| ranges.contains(&id))
    .count()
    .to_string()
}

pub fn day5_2(input: &String) -> String {
    let (ranges, _) = parse_input(input);

    ranges.as_slice().into_iter()
        .map(|x| {
            let start = extract(x.start_bound());
            let end = extract(x.end_bound());
            println!("range between {start} and {end}");
            return end - start + 1;
        })
        .sum::<usize>()
        .to_string()
}

fn extract(input: Bound<&usize>) -> usize {
    match input {
        Bound::Included(included) => {*included}
        Bound::Excluded(excluded) => {*excluded}
        Bound::Unbounded => { panic!()}
    }
}

/// Parse Range from "123-456" pattern
fn parse_range(s: &str) -> GenericRange<usize> {
    let (left, right) = s.split_once("-").unwrap();
    let start = left.parse().unwrap();
    let end = right.parse().unwrap();
    GenericRange::new_closed(start, end)
}

fn parse_input(input: &str) -> (Ranges<usize>, Vec<usize>) {
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();
    let ranges: Ranges<usize> = ranges_str.split("\n")
        .filter(|x| !x.is_empty())
        .map(parse_range)
        .collect();

    let ids: Vec<usize> = ids_str.split("\n")
        .filter(|x| !x.is_empty())
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    (ranges, ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";

    #[test]
    fn test_day5() {
        let input = String::from(TEST_INPUT);

        let result = day5(&input);

        assert_eq!(String::from("3"), result);
    }

    #[test]
    #[ignore]
    fn test_day5_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day5_2(&input);

        assert_eq!(String::from("14"), result);
    }
}

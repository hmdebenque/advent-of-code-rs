use regex::Regex;

pub fn day1(input: &String) -> String {
    let mut result = Vec::new();
    for coord in input.split("\n") {
        let extracted = extract_coord_digits(&coord);
        result.push(extracted);
        log::info!("day1 intermediate result: {in} => {out}", in=coord, out=extracted);
    }
    log::info!("Day 1 intermediate result: {:?}", result);
    let fold = result
        .iter()
        .sum::<u16>()
        .to_string();
    return fold;
}


pub fn day1_2(input: &String) -> String {
    let mut result = Vec::new();
    for coord in input.split("\n") {
        let extracted = extract_coord_str(&coord);
        result.push(extracted);
        log::info!("day1 intermediate result: {in} => {out}", in=coord, out=extracted);
    }
    log::info!("Day 1 intermediate result: {:?}", result);
    let fold = result
        .iter()
        .sum::<u16>()
        .to_string();
    return fold;
}


fn extract_coord_digits(input: &str) -> u16 {
    let mut first_digit: u16 = 0;
    let mut last_digit: u16 = 0;
    let mut first_found = false;
    for character in input.chars() {
        if character.is_numeric() {
            if !first_found {
                first_found = true;
                first_digit = character.to_digit(10).unwrap() as u16;
            }
            last_digit = character.to_digit(10).unwrap() as u16;
        }
    }
    return first_digit * 10 + last_digit;
}

const STR_TO_NUM: &[&str; 10] = &[
    r#"zero$"#, r#"one$"#, r#"two$"#, r#"three$"#, r#"four$"#, r#"five$"#, r#"six$"#, r#"seven$"#, r#"eight$"#, r#"nine$"#];

fn extract_coord_str(input: &str) -> u16 {
    let mut first_digit: u16 = 0;
    let mut last_digit: u16 = 0;
    let mut first_found = false;
    let mut text_found= String::new();

    for character in input.chars() {
        if character.is_numeric() {
            if !first_found {
                first_found = true;
                first_digit = character.to_digit(10).unwrap() as u16;
            }
            last_digit = character.to_digit(10).unwrap() as u16;
        } else {
            text_found.push(character);
            let match_digit_str = match_digit_str(&text_found);
            if match_digit_str.is_some() {
                if !first_found {
                    first_found = true;
                    first_digit = match_digit_str.unwrap();
                }
                last_digit = match_digit_str.unwrap();
            }
        }
    }
    return first_digit * 10 + last_digit;
}

fn match_digit_str(input: &String) -> Option<u16> {
    for (index, strnum) in STR_TO_NUM.iter().enumerate() {
        // This is highly unefficient but Rust do not authorize me to have regex as constants
        let pattern = Regex::new(*strnum).unwrap();
        if pattern.is_match(input) {
            return Option::Some(index as u16);
        }
    }
    return Option::None;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day1() {
        let input = String::from("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");

        let result = day1(&input);

        assert_eq!(String::from("142"), result);
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
7pqrstsixteen");

        let result = day1_2(&input);

        assert_eq!(String::from("281"), result);
    }

    #[test]
    fn extract_coord_str() {
        assert_eq!(29, super::extract_coord_str("two1nine"));
        assert_eq!(83, super::extract_coord_str("eightwothree"));
        assert_eq!(13, super::extract_coord_str("abcone2threexyz"));
        assert_eq!(24, super::extract_coord_str("xtwone3four"));
        assert_eq!(42, super::extract_coord_str("4nineeightseven2"));
        assert_eq!(14, super::extract_coord_str("zoneight234"));
        assert_eq!(76, super::extract_coord_str("7pqrstsixteen"));
        assert_eq!(66, super::extract_coord_str("stsixte"));
        // case overlapping number strings
        assert_eq!(28, super::extract_coord_str("2fiveshtds4oneightsj"));
        assert_eq!(22, super::extract_coord_str("qxtbbtwo7jrdgxlcpxbczxhnpjthreetwogcfl"));
    }

    #[test]
    fn test_regexp() {
        let pattern = Regex::new("(hel|lo)$").unwrap();
        assert!(pattern.is_match("kjdshgfqilucregliulo"));
        assert!(pattern.is_match("kjdshgfqilucregliuhel"));
        assert!(!pattern.is_match("kjdshgfqilucregliu"));
    }

}

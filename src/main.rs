use std::{char, future::IntoFuture, fmt::format};
use error_chain::error_chain;
use std::io::Read;
use std::env;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() {
    let day = 1;
    let authentCookie = format!("session={}", env::var("AOC_COOKIE").unwrap());
    download_input(authentCookie, day);
}

fn download_input(authCookie: String, day: i32) -> Result<()> {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    let mut body = String::new();
    let mut res = reqwest::blocking::get(url)?;
    res.read_to_string(&mut body)?;

    println!("response: {}", body);
    return Result::Ok(());
}

fn day1(input: &[& str]) -> Vec<u32> {
    let mut result = Vec::new();
    for coord in input {
        result.push(extract_coordinates(&coord))
    }
    result
}

fn extract_coordinates(input: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;
    let mut first_found = false;
    for character in input.chars() {
        if character.is_numeric() {
            if !first_found {
                first_found = true;
                first_digit = character.to_digit(10).unwrap();
            }
            last_digit = character.to_digit(10).unwrap();
        }
    }

    return first_digit * 10 + last_digit;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day1() {
        let input = &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];

        let result = day1(input);

        assert_eq!(vec![12, 38, 15, 77], result);
    }

}

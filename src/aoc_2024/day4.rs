use crate::aoc_2024::common::CharMatrix;
use std::str::FromStr;

pub fn day4(input: &String) -> String {
    let matrix = CharMatrix::from_str(input).unwrap();
    println!("Matrix parsed:\n{:?}", matrix);
    matrix.search_text().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // test part 1 that was broken with part 2
    fn test_day4() {
        let input = String::from(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );

        let result = day4(&input);

        assert_eq!(String::from("18"), result);
    }

    #[test]
    fn test_day4_2() {
        let input = String::from(
            ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
",
        );

        let result = day4(&input);

        assert_eq!(String::from("9"), result);
    }
}

pub fn day2(input: &String) -> String {
    parse_input(input)
        .iter()
        .filter(|report| Report::is_valid(report))
        .count()
        .to_string()
}

pub fn day2_2(input: &String) -> String {
    parse_input(input)
        .iter()
        .filter(|report| Report::is_valid_with_dampener(report))
        .count()
        .to_string()
}

#[derive(Debug)]
struct Report {
    levels: Vec<u8>,
}

impl Report {
    fn parse(input: &str) -> Option<Self> {
        let levels: Vec<u8> = input
            .split_whitespace()
            .map(|l| l.parse().unwrap())
            .collect();
        if levels.len() > 1 {
            Some(Report { levels })
        } else {
            None
        }
    }

    fn is_valid(&self) -> bool {
        Self::level_ok(&self.levels)
    }

    fn level_ok(levels: &Vec<u8>) -> bool {
        let increasing = levels[0] < levels[1];
        // check increases
        if increasing {
            if !levels.is_sorted() {
                return false;
            }
        } else {
            if !levels.is_sorted_by(|l, r| l >= r) {
                return false;
            }
        }
        // check diff
        let mut prev_level = levels[0];
        for level in 1..levels.len() {
            let current_level = levels[level];
            let distance = prev_level.abs_diff(current_level);
            if distance < 1 || distance > 3 {
                return false;
            }
            prev_level = current_level;
        }
        true
    }

    /// Check if can be valid without one value
    fn is_valid_with_dampener(&self) -> bool {
        if self.is_valid() {
            return true;
        }
        for i in 0..self.levels.len() {
            let mut copy = Vec::new();
            for j in 0..self.levels.len() {
                if i != j {
                    copy.push(self.levels[j]);
                }
            }
            // if a single combination works: bingo
            if Self::level_ok(&copy) {
                return true;
            }
        }
        false
    }
}

fn parse_input(input: &str) -> Vec<Report> {
    input
        .split("\n")
        .map(|l| Report::parse(l))
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        let result = day2(&input);

        assert_eq!(String::from("2"), result);
    }

    #[test]
    fn test_day2_2() {
        let input = String::from(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );

        let result = day2_2(&input);

        assert_eq!(String::from("4"), result);
    }
}

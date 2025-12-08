use log::{debug};


#[derive(Debug)]
struct PowerBank {
    batteries: Vec<usize>
}

impl PowerBank {
    /// Get the max from the power bank
    /// nb_of_batteries = nb of batteries that must be activated.
    fn max_joltage(&self, batteries_max_active: usize) -> usize {
        let batteries_len = self.batteries.len();
        if batteries_len < batteries_max_active {
            self.batteries.iter().rev().enumerate().map(|(index, val)| *val * (index + 1)).sum()
        } else {
            let mut last_max_index = 0;

            let mut total = 0;
            let batteries_max_index = batteries_len - 1;

            for i in 0..batteries_max_active {
                let end_index = batteries_max_index - batteries_max_active + i + 1;

                let search_slice: &[usize] = &self.batteries[last_max_index..=end_index];
                let (relative_max_index, max_value) = search_slice.iter().enumerate().max_by(|(index1, val), (index2, val2)| { val.cmp(val2).then(index2.cmp(index1)) }
                ).unwrap();
                total += *max_value * 10usize.pow((batteries_max_active - i - 1) as u32);
                last_max_index = last_max_index + relative_max_index + 1;
            }
            println!("Max jolt found for {self:?} is {total}");
            total
        }
    }

}

pub fn day3(input: &String) -> String {
    parse_input(input).iter()
        .map(|pb| pb.max_joltage(2))
        .sum::<usize>()
        .to_string()
}

pub fn day3_2(input: &String) -> String {
    parse_input(input).iter()
        .map(|pb| pb.max_joltage(12))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &str) -> Vec<PowerBank> {
    input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|s| {
            let batteries = s.chars()
                .map(|c| c.to_digit(10))
                .map(Option::unwrap)
                .map(|x| x as usize)
                .collect();
            let bank = PowerBank { batteries };
            println!("parsed Power bank: {bank:?}");
            debug!("parsed Power bank: {bank:?}");
            bank
        }
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_day1() {
        let input = String::from(TEST_INPUT);

        let result = day3(&input);

        assert_eq!(String::from("357"), result);
    }

    #[test]
    fn test_day1_2() {
        let _ = env_logger::builder().is_test(true).try_init();
        let input = String::from(TEST_INPUT);

        let result = day3_2(&input);

        assert_eq!(String::from("3121910778619"), result);
    }
}

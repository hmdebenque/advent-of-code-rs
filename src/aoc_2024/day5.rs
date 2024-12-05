use std::fmt::Debug;
use std::str::FromStr;

pub fn day5(input: &String) -> String {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update: &&Update| rules.matches(update))
        .map(|update| update.get_center_pages())
        .sum::<usize>()
        .to_string()
}

pub fn day5_2(input: &String) -> String {
    let (rules, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update: &&Update| rules.matches(update))
        // .map(|update: &mut Update| rules.correct(update))
        .map(|update| update.get_center_pages())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split_once("|").unwrap();
        Ok(Rule {
            before: usize::from_str(split.0).unwrap(),
            after: usize::from_str(split.1).unwrap(),
        })
    }
}

#[derive(Debug)]
struct Update {
    pages: Vec<usize>,
}

impl FromStr for Update {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(());
        }
        let pages: Vec<usize> = input
            .split(",")
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        Ok(Update { pages })
    }
}

impl Update {
    fn get_center_pages(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

impl Matches for Rule {
    /// Rule is OK if pages are absent or are present in the correct order.
    fn matches(&self, update: &Update) -> bool {
        // search last before
        let index_before = self.last_left_element_index(update);
        // search first after
        let index_after = self.first_right_element_index(update);
        let is_ok = index_before.is_none()
            || index_after.is_none()
            || index_before.unwrap() < index_after.unwrap();
        if !is_ok {
            println!("Update {:?} do not pass rule {:?}!", update, &self)
        }
        is_ok
    }
}

impl Rule {
    fn last_left_element_index(&self, update: &Update) -> Option<usize> {
        update.pages.iter().rposition(|e| self.before.eq(e))
    }

    fn first_right_element_index(&self, update: &Update) -> Option<usize> {
        update.pages.iter().position(|e| self.after.eq(e))
    }
}

impl Correct for Rule {
    fn correct(&self, update: &mut Update) -> bool {
        if !self.matches(update) {
            // try correction
            let to_go_left_index = self.last_left_element_index(update).unwrap();
            let to_go_right_index = self.first_right_element_index(update).unwrap();
            let to_go_left = update.pages[to_go_left_index];
            let to_go_right = update.pages[to_go_right_index];
            update.pages[to_go_right_index] = to_go_left;
            update.pages[to_go_left_index] = to_go_right;
            return true;
        }
        false
    }
}

impl Correct for Vec<Rule> {
    fn correct(&self, update: &mut Update) -> bool {
        self.iter().all(|rule| rule.correct(update))
    }
}

trait Matches {
    fn matches(&self, update: &Update) -> bool;
}

trait Correct {
    fn correct(&self, update: &mut Update) -> bool;
}

impl Matches for Vec<Rule> {
    fn matches(&self, update: &Update) -> bool {
        self.iter().all(|rule| rule.matches(update))
    }
}

fn parse_input(input: &String) -> (Vec<Rule>, Vec<Update>) {
    let parts: Vec<&str> = input.split("\n\n").collect();
    (
        parse_lines(parts[0], Rule::from_str),
        parse_lines(parts[1], Update::from_str),
    )
}

fn parse_lines<Type>(input: &str, parser: fn(&str) -> Result<Type, ()>) -> Vec<Type> {
    input.lines().map(parser).map(Result::unwrap).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &'static str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test_day5() {
        let input = String::from(PUZZLE_INPUT);

        let result = day5(&input);

        assert_eq!(String::from("143"), result);
    }

    #[test]
    fn test_day5_2() {
        let input = String::from(PUZZLE_INPUT);

        let result = day5(&input);

        assert_eq!(String::from("123"), result);
    }
}

#[macro_use]
extern crate lazy_static;

use aoc2020::lines;

use regex::Regex;

#[derive(Debug)]
struct Policy {
    lhs_requirement: usize,
    rhs_requirement: usize,
    character: char,
}

#[derive(Debug)]
struct PasswordCheck {
    policy: Policy,
    password: String,
}

impl<T: AsRef<str>> From<T> for PasswordCheck {
    fn from(line: T) -> Self {
        let line = line.as_ref();

        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+)\s([[:alpha:]]):\s([[:alpha:]]+)")
                .expect("Unable to parse regex");
        }

        let captures = REGEX.captures(&line).expect("Unable to capture inputs");

        PasswordCheck {
            policy: Policy {
                lhs_requirement: captures[1]
                    .parse()
                    .expect("Unable to convert the left requirement to a number"),
                rhs_requirement: captures[2]
                    .parse()
                    .expect("Unable to convert the right requirement to a number"),
                character: captures[3]
                    .chars()
                    .next()
                    .expect("Unable to parse character"),
            },
            password: captures[4].to_string(),
        }
    }
}

trait SledRentalDownTheStreet {
    fn is_valid(&self) -> bool;
}

impl SledRentalDownTheStreet for PasswordCheck {
    fn is_valid(&self) -> bool {
        let policy_char = self.policy.character;
        let occurrences = self.password.chars().filter(|c| *c == policy_char).count();

        occurrences >= self.policy.lhs_requirement && occurrences <= self.policy.rhs_requirement
    }
}

trait TobogganRental {
    fn is_valid(&self) -> bool;
}

impl TobogganRental for PasswordCheck {
    fn is_valid(&self) -> bool {
        let policy_char = self.policy.character;

        let li = self.policy.lhs_requirement.checked_sub(1);
        let ri = self.policy.rhs_requirement.checked_sub(1);

        let left_char = li.and_then(|n| self.password.chars().nth(n));
        let right_char = ri.and_then(|n| self.password.chars().nth(n));

        match (left_char, right_char) {
            (Some(l), Some(r)) if (l == policy_char) ^ (r == policy_char) => true,
            (Some(l), None) if l == policy_char => true,
            (None, Some(r)) if r == policy_char => true,
            _ => false,
        }
    }
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_02");
    let part1 = lines(path)
        .unwrap()
        .map(PasswordCheck::from)
        .filter(SledRentalDownTheStreet::is_valid)
        .count();

    let part2 = lines(path)
        .unwrap()
        .map(PasswordCheck::from)
        .filter(TobogganRental::is_valid)
        .count();

    println!("exercise 1: {}", part1);
    println!("exercise 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_02_example");
        let part1 = lines(path)
            .unwrap()
            .map(PasswordCheck::from)
            .filter(SledRentalDownTheStreet::is_valid)
            .count();

        assert_eq!(part1, 2);
    }

    #[test]
    fn example2() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_02_example");
        let part2 = lines(path)
            .unwrap()
            .map(PasswordCheck::from)
            .filter(TobogganRental::is_valid)
            .count();

        assert_eq!(part2, 1);
    }
}

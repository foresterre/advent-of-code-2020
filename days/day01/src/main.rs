use aoc2020::{lines, AdventError, TResult};
use std::collections::HashSet;

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_01");
    let input = lines(path)
        .unwrap()
        .filter_map(|item| item.parse::<i32>().ok())
        .collect::<HashSet<_>>();

    println!("exercise 1: {}", part1(&input, 2020).unwrap());
    println!("exercise 2: {}", part2(&input, 2020).unwrap());
}

fn part1(set: &HashSet<i32>, expense: i32) -> TResult<i32> {
    set.iter()
        .find_map(|u| {
            let v = expense - u;
            find_value(&set, v).map(|v| u * v)
        })
        .ok_or(AdventError::UnableToCompute)
}

fn part2(set: &HashSet<i32>, expense: i32) -> TResult<i32> {
    // this only works because the input is small enough, but why optimise if not necessary? <3
    for u in set {
        for v in set {
            for w in set {
                if u + v + w == expense {
                    return Ok(u * v * w);
                }
            }
        }
    }
    Err(AdventError::UnableToCompute)
}

fn find_value(set: &HashSet<i32>, target: i32) -> Option<i32> {
    if set.contains(&target) {
        Some(target)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_01_example");
        let lines = lines(path)
            .unwrap()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect::<HashSet<_>>();

        assert_eq!(part1(&lines, 2020).unwrap(), 514579);
    }

    #[test]
    fn example2() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_01_example");
        let lines = lines(path)
            .unwrap()
            .filter_map(|item| item.parse::<i32>().ok())
            .collect::<HashSet<_>>();

        assert_eq!(part2(&lines, 2020).unwrap(), 241861950);
        assert_eq!(979 * 366 * 675, 241861950);
    }
}

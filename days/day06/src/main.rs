use aoc2020::{read_file, TResult};
use std::collections::HashMap;

type Output = usize;

fn main() -> TResult<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input");
    let input = read_file(path)?;

    println!("exercise 1: {}", part1(&input)?);
    println!("exercise 2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> TResult<Output> {
    let answer: usize = input
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::<char, usize>::new();
            group.split_whitespace().for_each(|person| {
                person.chars().for_each(|c| {
                    *map.entry(c).or_default() += 1;
                });
            });
            map
        })
        .map(|map| map.iter().count())
        .sum();

    Ok(answer)
}

fn part2(input: &str) -> TResult<Output> {
    let answer: usize = input
        .split("\n\n")
        .map(|group| {
            let mut map = HashMap::<char, usize>::new();
            let mut persons = 0usize;
            group.split_whitespace().for_each(|person| {
                person.chars().for_each(|c| {
                    *map.entry(c).or_default() += 1;
                });
                persons += 1;
            });
            (map, persons)
        })
        .map(|(map, amount)| map.iter().filter(|(_k, v)| **v == amount).count())
        .sum();

    Ok(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_example");
        let input = read_file(path).unwrap();

        assert_eq!(part1(&input).unwrap(), 9999);
    }

    #[test]
    fn example2() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_example");
        let input = read_file(path).unwrap();

        assert_eq!(part2(&input).unwrap(), 9999);
    }
}

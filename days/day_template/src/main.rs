use aoc2020::{lines, TResult, AdventError, read_file};

type Output = usize;

fn main() -> TResult<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input");
    let input = read_file(path)?;

    println!("exercise 1: {}", part1(&input)?);
    println!("exercise 2: {}", part2(&input)?);

    Ok(())
}

fn part1(_input: &str) -> TResult<Output> {
    Err(AdventError::UnableToCompute)
}

fn part2(_input: &str) -> TResult<Output> {
    Err(AdventError::UnableToCompute)
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

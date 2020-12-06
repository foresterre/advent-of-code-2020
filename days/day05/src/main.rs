use aoc2020::{TResult, AdventError, read_file};

fn main() -> TResult<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input");
    let input = read_file(path)?;

    println!("exercise 1: {}", part1(&input)?);
    println!("exercise 2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> TResult<usize> {
    let seats = seats(input)?;

    seats
        .into_iter()
        .map(|(col, row)| col + row * 8)
        .max()
        .ok_or(AdventError::UnableToCompute)
}

fn part2(input: &str) -> TResult<usize> {
    let mut ids = seats(input)?
        .into_iter()
        .map(|(col, row)| col + row * 8)
        .collect::<Vec<_>>();

    ids.sort();

    ids.windows(2)
        .find(|seats| seats[0] + 2 == seats[1])
        .map(|seats| seats[0] + 1)
        .ok_or(AdventError::UnableToCompute)
}

fn seats(input: &str) -> TResult<impl IntoIterator<Item=(usize, usize)>> {
    input.lines()
        .map(|pass| {
            let pass = replace(pass)?;

            let div = pass.len() - 3;
            let row = &pass[0..div];
            let col = &pass[div..];


            let col = usize::from_str_radix(col, 2).map_err(|_| AdventError::ParseError {
                day: 5,
                msg: "can't parse col to num".to_string()
            })?;

            let row = usize::from_str_radix(row, 2).map_err(|_| AdventError::ParseError {
                day: 5,
                msg: "can't parse row to num".to_string()
            })?;

            Ok((col, row))

        }).collect::<TResult<Vec<_>>>()
}

fn replace(text: &str) -> TResult<String> {
    text.chars().map(|c| {
        match c {
            'B' => Ok('1'),
            'F' => Ok('0'),
            'L' => Ok('0'),
            'R' => Ok('1'),
            _ => Err(AdventError::ParseError {
                day: 5,
                msg: "unsupported character found".to_string()
            })
        }
    }).collect()
}
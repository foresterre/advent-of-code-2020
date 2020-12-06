use aoc2020::{read_file, AdventError, TResult};
use std::collections::HashMap;
use std::convert::TryFrom;

type Output = usize;

fn main() -> TResult<()> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input");
    let input = read_file(path)?;

    println!("exercise 1: {}", part1(&input)?);
    println!("exercise 2: {}", part2(&input)?);

    Ok(())
}

fn part1(input: &str) -> TResult<Output> {
    let passports = input
        .split("\n\n")
        .map(Passport::try_from)
        .collect::<TResult<Vec<_>>>()?;

    let required_fields: &'static [&'static str] =
        &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    Ok(passports
        .iter()
        .filter(|p| p.is_valid(required_fields))
        .count())
}

fn part2(input: &str) -> TResult<Output> {
    let passports = input
        .split("\n\n")
        .map(Passport::try_from)
        .collect::<TResult<Vec<_>>>()?;

    let mut requirements: HashMap<&'static str, &dyn Fn(&str) -> bool> = HashMap::with_capacity(8);

    let range_validator = &move |from: u16, up_to_and_including: u16| {
        // the first check is to make sure we have a valid date in range; this check will reject most cases where the length is not 4
        // the second check is too make sure we don't allow leading zeros, which FromStr::<u16>::parse accepts
        move |input: &str| {
            input
                .parse::<u16>()
                .map(|year| year >= from && year <= up_to_and_including)
                .unwrap_or(false)
                && input.len() == 4
        }
    };

    let v = range_validator(1920, 2002);
    requirements.insert("byr", &v);

    let v = range_validator(2010, 2020);
    requirements.insert("iyr", &v);

    let v = range_validator(2020, 2030);
    requirements.insert("eyr", &v);

    requirements.insert("hgt", &move |hgt: &str| {
        let unit = &hgt[hgt.len() - 2..];
        if let Ok(value) = hgt[0..hgt.len() - 2].parse::<u8>() {
            match unit {
                "cm" if value >= 150 && value <= 193 => true,
                "in" if value >= 59 && value <= 76 => true,
                _ => false,
            }
        } else {
            false
        }
    });

    requirements.insert("hcl", &move |hcl: &str| {
        let color = &hcl[1..];

        hcl.starts_with('#')
            && color.len() == 6
            && color
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    });

    requirements.insert("ecl", &move |ecl: &str| {
        let allowed: &'static [&'static str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        allowed.contains(&ecl)
    });

    requirements.insert("pid", &move |pid: &str| {
        pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
    });

    Ok(passports
        .iter()
        .filter(|p| p.is_extra_valid(&requirements))
        .count())
}

#[derive(Debug)]
struct Passport<'source> {
    fields: HashMap<&'source str, &'source str>,
}

impl<'source> TryFrom<&'source str> for Passport<'source> {
    type Error = AdventError;

    fn try_from(scanned_fields: &'source str) -> Result<Self, Self::Error> {
        let fields = scanned_fields
            .split_ascii_whitespace() // split on fields
            .map(split_key_value_pair) // map each to a key value pair
            .collect::<TResult<HashMap<&'source str, &'source str>>>()?;

        Ok(Self { fields })
    }
}

fn split_key_value_pair(pair: &str) -> TResult<(&str, &str)> {
    let mut pair = pair.splitn(2, ':');
    let key = pair.next().ok_or_else(|| AdventError::ParseError {
        day: 4,
        msg: "unable to obtain field name".to_string(),
    })?;
    let value = pair.next().ok_or_else(|| AdventError::ParseError {
        day: 4,
        msg: "unable to obtain field value".to_string(),
    })?;

    Ok((key, value))
}

impl<'source> Passport<'source> {
    fn is_valid(&self, required_fields: &'static [&'static str]) -> bool {
        required_fields
            .iter()
            .all(|expected| self.fields.contains_key(expected))
    }

    fn is_extra_valid(&self, requirements: &HashMap<&'static str, &dyn Fn(&str) -> bool>) -> bool {
        requirements.iter().all(|(field, validator)| {
            if let Some(found_field_value) = self.fields.get(field) {
                // here we found the field, and we'll let the validator decide whether it is actually valid
                validator(found_field_value)
            } else {
                // in this case the field was not found, but it was required
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_example");
        let input = read_file(path).unwrap();

        assert_eq!(part1(&input).unwrap(), 2);
    }

    #[test]
    fn example2() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/", "input_example");
        let input = read_file(path).unwrap();

        assert_eq!(part2(&input).unwrap(), 2);
    }
}

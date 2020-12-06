use thiserror::Error;
use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, BufRead, Read};

pub type TResult<T> = std::result::Result<T, AdventError>;

#[derive(Debug, Error)]
pub enum AdventError {

    #[error("Unable to parse day {day} input: {msg}")]
    ParseError {
        day: u8,
        msg: String,
    },

    #[error("Unable to open file: {0}")]
    FileError(std::io::Error),

    #[error("No answer could be found")]
    UnableToCompute,
}

pub fn lines(path: impl AsRef<Path>) -> TResult<impl Iterator<Item=String>> {
    File::open(path)
        .map(|f| BufReader::new(f).lines().filter_map(|f| f.ok()))
        .map_err(AdventError::FileError)
}

pub fn read_file(path: impl AsRef<Path>) -> TResult<String> {
    File::open(path)
        .and_then(|f| {
            let mut content = String::new();
            BufReader::new(f).read_to_string(&mut content)
                .map(|_| content)
        })
        .map_err(AdventError::FileError)
}

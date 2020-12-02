use thiserror::Error;
use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, BufRead};

pub type TResult<T> = std::result::Result<T, AdventError>;

#[derive(Debug, Error)]
pub enum AdventError {
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

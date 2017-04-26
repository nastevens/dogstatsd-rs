use std::error::Error;
use std::{fmt, io};

/// Errors returned by library
#[derive(Debug)]
pub enum DogstatsdError {
    /// Error was an io::Error
    IoError(io::Error),
}

use self::DogstatsdError::*;

impl fmt::Display for DogstatsdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for DogstatsdError {
    fn description(&self) -> &str {
        match *self {
            IoError(ref error) => error.description(),
        }
    }
}

impl From<io::Error> for DogstatsdError {
    fn from(e: io::Error) -> Self {
        IoError(e)
    }
}

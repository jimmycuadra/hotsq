use std::error::Error as StdError;
use std::io::Error as IoError;
use std::num::ParseIntError;

use toml::{DecodeError, ParserError};

#[derive(Debug)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn new<S>(error: S) -> Error where S: Into<String> {
        Error {
            error: error.into(),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.error)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.error
    }
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::new(error.description())
    }
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Error {
        Error::new(error.description())
    }
}

impl From<DecodeError> for Error {
    fn from(error: DecodeError) -> Error {
        Error::new(format!("{}", error))
    }
}

impl<'a> From<&'a ParserError> for Error {
    fn from(error: &'a ParserError) -> Error {
        Error::new(format!("{}", error))
    }
}

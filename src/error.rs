use std::fmt;
use std::error::Error as StdError;

/// The `tracery` error type
#[derive(Debug, Clone)]
pub enum Error {
    /// Error encountered while parsing a rule
    ParseError(String),

    /// A referenced key does not exist
    MissingKeyError(String),

    /// Error encountered while parsing JSON input
    JsonError(String),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::JsonError(format!("{}", e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError(ref s) => write!(f, "parse error: {}", s),
            Error::MissingKeyError(ref s) => write!(f, "missing key error: {}", s),
            Error::JsonError(ref s) => write!(f, "json error: {}", s),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseError(ref s) => s,
            Error::MissingKeyError(ref s) => s,
            Error::JsonError(ref s) => s,
        }
    }
}

extern crate yaml_rust;

use std;
use std::error::Error;
use std::fmt;


/// The ParseError describes any error that can occur while parsing the scene description file:
/// - YAML syntax error
/// - I/O Error (open/read file, etc...)
/// - Unknown Value Error: a value needed for scene construction is missing
#[derive(Debug)]
pub enum ParseError {
    YamlScanError(yaml_rust::ScanError),
    IOError(std::io::Error),
    UnknownValueError(String)
}

impl From<yaml_rust::ScanError> for ParseError {
    fn from(err: yaml_rust::ScanError) -> ParseError {
        ParseError::YamlScanError(err)
    }
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> ParseError {
        ParseError::IOError(err)
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::YamlScanError(ref err) => err.fmt(f),
            ParseError::IOError(ref err) => err.fmt(f),
            ParseError::UnknownValueError(ref err) => err.fmt(f)
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::YamlScanError(ref err) => err.description(),
            ParseError::IOError(ref err) => err.description(),
            ParseError::UnknownValueError(ref err_string) => err_string,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::YamlScanError(ref err) => Some(err),
            ParseError::IOError(ref err) => Some(err),
            ParseError::UnknownValueError(ref err) => None,
        }
    }
}
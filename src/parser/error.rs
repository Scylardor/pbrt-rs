extern crate yaml_rust;

use std::error::Error;
use std::fmt;


/// The ParseError describes any error that can occur while parsing the scene description file:
/// - YAML syntax error
/// - I/O Error (open/read file, etc...)
#[derive(Debug)]
pub enum ParseError {
    YamlScanError(yaml_rust::ScanError)
}

impl From<yaml_rust::ScanError> for ParseError {
    fn from(err: yaml_rust::ScanError) -> ParseError {
        ParseError::YamlScanError(err)
    }
}


impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::YamlScanError(ref err) => err.fmt(f),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::YamlScanError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::YamlScanError(ref err) => Some(err)
        }
    }
}
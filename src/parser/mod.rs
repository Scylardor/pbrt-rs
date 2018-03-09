extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml};

use std::fs::File; // File::open etc.
use std::io::{self, Read}; // stdin, read_to_string etc.

mod error;
pub use self::error::ParseError;

// The rpbrt parser uses the YAML format.
// We use yaml-rust to decode the scene file.
// Just like original PBRT, we reeuse the UNIX file name syntax:
// if "-" is passed as a filename, we'll read the file from standard input.
pub struct Parser {
    // yaml_rust supports multi-docs but we currently only use one (the first).
    // We still keep the Vec around because it's easier and avoids a necessary clone
    // at loading time... (otherwise Rust won't let us index the temporary vector... or you need to clone the Yaml object)
    scene_desc: Vec<Yaml>
}

impl Parser {

    pub fn from(name: &str) -> Result<Self, ParseError> {
        let yamlStr = match name {
            "-" => try!(Self::read_stdin()),
            _ =>  try!(Self::read_file(name)),
        };

        let docs = try!(YamlLoader::load_from_str(&name));

        Ok( Parser { scene_desc: docs } )
    }


    fn read_stdin() -> Result<String, ParseError> {
        let mut buffer = String::new();
        try!(io::stdin().read_to_string(&mut buffer));

        Ok(buffer)
    }


    fn read_file(filename: &str) -> Result<String, ParseError> {
        let mut file = try!(File::open(filename));
        let mut contents = String::new();
        try!(file.read_to_string(&mut contents));

        Ok(contents)
    }


}
extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml, yaml};

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
        let yaml_str = match name {
            "-" => try!(Self::read_stdin()),
            _ =>  try!(Self::read_file(name)),
        };

        let docs = try!(YamlLoader::load_from_str(&yaml_str));

        let p = Parser { scene_desc: docs };
        p.parse();

        Ok( p )
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

    fn print_indent(indent: usize) {
        for _ in 0..indent {
            print!("    ");
        }
    }

    fn dump_node(doc: &yaml::Yaml, indent: usize) {
        match *doc {
            yaml::Yaml::Array(ref v) => {
                for x in v {
                    Parser::dump_node(x, indent + 1);
                }
            },
            yaml::Yaml::Hash(ref h) => {
                for (k, v) in h {
                    Parser::print_indent(indent);
                    println!("{:?}:", k);
                    Parser::dump_node(v, indent + 1);
                }
            },
            _ => {
                Parser::print_indent(indent);
                println!("{:?}", doc);
            }
        }
    }


    pub fn parse(&self) {
        for doc in &self.scene_desc {
            println!("---");
            Parser::dump_node(&doc, 0);
        }
    }


}
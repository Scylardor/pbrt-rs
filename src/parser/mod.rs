extern crate yaml_rust;
use self::yaml_rust::{YamlLoader, Yaml, yaml};

use std::fs::File; // File::open etc.
use std::io::{self, Read}; // stdin, read_to_string etc.

mod error;
pub use self::error::ParseError;

pub mod lexer;

// The rpbrt parser uses the YAML format.
// We use yaml-rust to decode the scene file.
// Just like original PBRT, we reeuse the UNIX file name syntax:
// if "-" is passed as a filename, we'll read the file from standard input.
pub struct Parser {
    /// YAML "documents" are what we use to store the scene information.
    /// Basically a Vec of Yaml::Hash's.
    scene_desc: Vec<Yaml>
}

/// An abstraction of the node type.
/// As much as possible we want the YAML to be encapsulated in the parser
pub type ParserNode = Yaml;

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


    /// The signature of this function is a bit scary but it's not that hard : its goal is to be generic over any wanted value type (that's T).
    /// So we pass a closure to the function that will check if the type variant we find under the key is what it's looking for.
    /// e.g. if we search for a std::String value, T will be String, and F will be (Option<&'a yaml::Yaml>) -> Option(&String).
    /// And since ParserNodes are expected to be the parser's own nodes passed back to it, we must specify they have the same lifetime than self
    /// to make Rust happy!
    fn get_hash_value<'a, T, F>(&'a self, key: &str, parent_node: Option<&'a ParserNode>, match_closure: F) -> Option<&T>
        where F: Fn(Option<&yaml::Yaml>) -> Option<&T> {
        if let &yaml::Yaml::Hash(ref parent_hash) = match parent_node {
            Some(ref x) => x,
            None => &self.scene_desc[0]
        }
        {
            let s = String::from(key);
            return match_closure(parent_hash.get(&yaml::Yaml::String(s)));
        }
        None
    }


    /// Public-facing function to retrieve a String value from a hash under given key.
    pub fn get_string<'a>(&'a self, key: &str, parent_node: Option<&'a ParserNode>) -> Option<&String> {
        self.get_hash_value(key, parent_node, |optYaml| match optYaml {
            Some(&yaml::Yaml::String(ref string)) => Some(string),
            _ => None
        })
    }


    /// Public-facing function to retrieve a Hash value from a hash under given key.
    /// It's a bit different from the others as we return the yaml::Yaml enum variant.
    /// Indeed, the user won't use the hash directly, we need the Yaml variant enum value to retrieve contained data
    /// in subsequent get_* calls. Returning the underlying Hash type wouldn't help from a client code POV.
    pub fn get_hash<'a>(&'a self, key: &str, parent_node: Option<&'a ParserNode>) -> Option<&yaml::Yaml> {
        self.get_hash_value(key, parent_node, |optYaml| match optYaml {
            h @ Some(&yaml::Yaml::Hash(_)) => h,
            _ => None
        })
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
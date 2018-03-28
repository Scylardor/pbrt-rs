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

    /// Used by the lexer to retrieve token values based on YAML documents.
    pub fn get_string_value(&self, token: &str, parent_node: Option<&ParserNode>) -> Option<&String> {
        match parent_node {
            Some(node) => {
                None
            }
            None => {
                for doc in &self.scene_desc {
                    match *doc {
                        yaml::Yaml::String(ref s) => Some(s),
                        _ => None
                    };
                }
                None
            }
        }
    }

    /// Since ParserNodes are expected to be the parser's own nodes passed back to it, we can specify they have the same lifetime
    /// than self to make Rustc happy!
    fn get_hash_value<'a>(&'a self, token: &str, parent_node: Option<&'a ParserNode>, ) -> Option<&String> {
        if let &yaml::Yaml::Hash(ref parent_hash) = match parent_node {
            Some(ref x) => x,
            None => &self.scene_desc[0]
        }
        {
            let s = String::from(token);
            return match parent_hash.get(&yaml::Yaml::String(s)) {
                Some(&yaml::Yaml::String(ref string)) => Some(string),
                _ => None
            };
        }
        None
    }


    /// Retrieve a String value from the hash node.
    fn get_hash_string_value<'a>(hash_node: &'a yaml::Hash, key: &yaml::Yaml) -> Option<&'a String> {
        match hash_node.get(key) {
            Some(&yaml::Yaml::String(ref string)) => Some(string),
            _ => None
        }
    }


    /// Since ParserNodes are expected to be the parser's own nodes passed back to it, we can specify they have the same lifetime
    /// than self to make Rustc happy!
    pub fn get_string_hash<'a>(&'a self, token: &str, parent_node: Option<&'a ParserNode>) -> Option<&String> {
        if let &yaml::Yaml::Hash(ref parent_hash) = match parent_node {
            Some(ref x) => x,
            None => &self.scene_desc[0]
        }
        {
            let key = yaml::Yaml::String(String::from(token));
            return match parent_hash.get(&key) {
                Some(&yaml::Yaml::String(ref string)) => Some(string),
                _ => None
            };
        }
        None
    }

    pub fn get_root_hash(&self, token: &str) -> Option<&ParserNode> {
        if let &yaml::Yaml::Hash(ref hash) = &self.scene_desc[0] {
            let s = String::from(token);
            return hash.get(&yaml::Yaml::String(s));
        }
        None
        // let integrator_doc = match &self.scene_desc[0] {
        //     yaml::Yaml::Hash(ref hash) => hash.get(,
        //     _ => println!("not found!")
        // // if integrator_doc == Some(s) {
        // //     println!("found!");
        // // }
        // // else {
        // //     println!("not found!");
        // // }

        // let doc = &self.scene_desc[0];


        // for doc in &self.scene_desc {
        //     match *doc {
        //         yaml::Yaml::Hash(ref hash) => {
        //             for (k,v) in hash {
        //                 match *k {
        //                     yaml::Yaml::String(ref string) => {
        //                         if *string == token { println!("found!"); }
        //                     }
        //                     _ => ()
        //                 };
        //             }
        //         }
        //         _ => ()
        //     };
        // }
        
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
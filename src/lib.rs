
mod parser;

mod math;

pub mod types;

use parser::{Parser, ParseError};

pub struct Raytracer {
    scene_parser: Parser
}


impl Raytracer {
    pub fn new_scene(scene_path: &str) -> Result<Self, ParseError> {
        let parser = try!(Parser::from(scene_path));


        Ok( Raytracer{scene_parser: parser} )
    }

    pub fn render_scene(&mut self) {
        // Render scene...
    }
}
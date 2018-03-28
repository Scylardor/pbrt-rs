mod math;

pub mod types;

mod parser;
use parser::Parser;
pub use parser::ParseError;

mod integrators;
use integrators::{Integrator, PathIntegrator};

struct SceneData {
    pub integrator: Box<Integrator>
}

pub struct Raytracer {
    scene: SceneData
}


impl Raytracer {
    pub fn from(scene_path: &str) -> Result<Self, ParseError> {
        let parser = try!(Parser::from(scene_path));

        let scene_data = Raytracer::build_scene_from(&parser);
        let rt = Raytracer{scene: scene_data};

        Ok( rt )
    }

    /// Using the given parser, extracts the scene information to build a scene data representation usable by the raytracer.
    /// This is the function responsible for setting all default values and stuff.
    fn build_scene_from(parser: &Parser) -> SceneData {
        let integrator_box = Raytracer::build_integrator(parser);

        SceneData{ integrator: integrator_box }
    }

    fn build_integrator(parser: &Parser) -> Box<Integrator> {
        parser::lexer::lex_integrator(parser);
        Box::new(PathIntegrator::new())
    }


    pub fn render_scene(&mut self) {
        // Render scene...
    }
}
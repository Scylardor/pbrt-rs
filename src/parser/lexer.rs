use parser::{Parser, ParserNode};

use integrators::{Integrator, PathIntegrator};

static TYPE_TOKEN: &'static str = "type";

static INTEGRATOR_TOKEN: &'static str = "integrator";
static PATH_INTEGRATOR_TOKEN: &'static str = "path";

static SAMPLER_TOKEN: &'static str = "sampler";
static HALTON_SAMPLER_TOKEN: &'static str = "halton";


fn get_default_integrator() -> Box<Integrator> {
    Box::new(PathIntegrator::new())
}

fn get_integrator(parser: &Parser, hash_node: &ParserNode) -> Box<Integrator> {
    let integ_type = parser.get_string(TYPE_TOKEN, Some(hash_node));

    match integ_type {
        Some(ref s) => println!("Type: {}", s),
        None => println!("No type specified!")
    }

    Box::new(PathIntegrator::new())
}

pub fn lex_integrator(parser: &Parser) {
    let integrator_box : Box<i32> = Box::new(42);
    let integrator_h = parser.get_hash(INTEGRATOR_TOKEN, None);

    let boxed_integ = match integrator_h {
        Some(ref hash_node) => get_integrator(parser, hash_node),
        None => get_default_integrator()
    };
}
extern crate nalgebra as na;

pub use na::zero;

pub mod types;
pub use types::*;

mod math;


pub struct Raytracer {
    pub x: i32
}

impl Raytracer {
    pub fn new() -> Raytracer {
        Raytracer{x:42}
    }

}

pub fn hello() -> String {
    String::from("hello!")
}


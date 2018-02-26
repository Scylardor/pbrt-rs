
use super::{Float, Vector3, Vector3f};
use super::math::{Real, zero, dot};

/// The Normal type is almost like a regular vector, but:
/// - you can't transform a point using a normal
/// - you can't compute the cross product of two normals
/// This is purely a "conceptual" type, here to remind you that a normal isn't _quite_ a vector.
/// Use it wisely!
pub struct Normal3<T> where T: Real {
    vec: Vector3<T>
}

impl<T> Normal3<T> where T: Real {

    pub fn new() -> Self {
        // Using na::zero() works.
        Normal3 { vec: zero() }
    }

    /// Copy a vector
    pub fn from_vec(&mut self, vector: &Vector3<T>) -> &mut Self {
        self.vec = *vector;
        self
    }


    /// Construct from three values.
    pub fn from_xyz(&mut self, x: T, y: T, z: T) -> &mut Self {
        self.vec[0] = x;
        self.vec[1] = y;
        self.vec[2] = z;
        self
    }
    
    /// The const vector accessor.
    /// I'm a bit lazy because what _should_ be done instead is a complete reimplementation of the Vector3
    /// minus functions like Cross and point transform...
    pub fn vector(&self) -> &Vector3<T> {
        &self.vec
    }
}


pub type Normal3f = Normal3<Float>;

impl Normal3f {
    /// An additional method of the Normal is the Faceforward method, useful for normals because
    /// we often need to flip a surface normal to make it lie on the same hemisphere of an outgoing ray.
    /// NB: I am in misery trying to implement this with a generic Normal type because of the float comparison,
    /// so implementing it only for the floating one for now.
    pub fn face_forward(&mut self, v: &Vector3f) -> &Self {
        if dot(self.vector(), v) < 0. {
            self.vec = -self.vector()
        }

        self
    }

}
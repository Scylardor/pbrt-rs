extern crate nalgebra as na;

pub use self::na::{Vector2, Vector3};

use types::Float;

/// Re-exporting the nalgebra types as pbrt types.
pub type Vector2i = Vector2<i32>;
pub type Vector2f = Vector2<Float>;

pub type Vector3i = Vector3<i32>;
pub type Vector3f = Vector3<Float>;

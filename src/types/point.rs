extern crate nalgebra as na;

use self::na::{Point2, Point3};

use types::Float;

/// Re-exporting the nalgebra types as pbrt types.
pub type Point2i = Point2<i32>;
pub type Point2f = Point2<Float>;

pub type Point3i = Point3<i32>;
pub type Point3f = Point3<Float>;


/// Global Type declarations


/// PBRT uses double precision (f64) for its Float type as default.
/// The rationale being that it's nowadays approximately as fast as f32s,
/// and more precise. But WRT original pbrt, you can specify a compile flag
/// to make it use f32 instead.
#[cfg(feature = "use_f32")]
pub type Float = f32;
#[cfg(not(feature = "use_f32"))]
pub type Float = f64;

/// Separating into multiple files to find everything more easily.
pub mod normal;
pub use self::normal::*;

pub mod vector;
pub use self::vector::*;

pub mod point;
pub use self::point::*;

/// Make common math operations available to children modules.
use super::math;
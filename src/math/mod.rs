pub mod matrix4x4;
pub mod transform;
pub mod vec3;

pub mod prelude {
    pub use crate::math::matrix4x4::*;
    pub use crate::math::transform::*;
    pub use crate::math::vec3::*;
}

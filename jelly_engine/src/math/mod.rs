mod matrix4x4;
mod transform;
mod vec3;
mod vec2;

pub mod prelude {
    pub use crate::math::matrix4x4::*;
    pub use crate::math::transform::*;
    pub use crate::math::vec3::*;
    pub use crate::math::vec2::*;
}

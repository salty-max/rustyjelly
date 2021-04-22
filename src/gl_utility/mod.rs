pub mod gl_buffer;
pub mod shader;

pub mod prelude {
    pub use crate::gl_utility::gl_buffer::*;
    pub use crate::gl_utility::shader::*;
}

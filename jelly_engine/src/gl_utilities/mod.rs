pub mod gl_buffer;
pub mod shader;

pub mod prelude {
    pub use crate::gl_utilities::gl_buffer::*;
    pub use crate::gl_utilities::shader::*;
}

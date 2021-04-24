mod color;
mod material;
mod sprite;
mod texture;
mod vertex;

pub mod prelude {
    pub use crate::graphics::color::*;
    pub use crate::graphics::material::*;
    pub use crate::graphics::sprite::*;
    pub use crate::graphics::texture::*;
    pub use crate::graphics::vertex::*;
}

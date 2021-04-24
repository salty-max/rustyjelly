use super::{color::Color, texture::Texture};

#[derive(Debug, Clone, Copy)]
pub struct Material<'a> {
    pub tint: Color,
    pub texture: &'a Texture,
}

impl<'a> Material<'a> {
    pub fn new(tint: Color, texture: &'a Texture) -> Material {
        Material { tint, texture }
    }
}

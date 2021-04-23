use crate::math::prelude::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32) -> Vertex {
        Vertex {
            position: Vec3::new(x, y, z),
        }
    }
}

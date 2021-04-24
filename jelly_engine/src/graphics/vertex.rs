#![allow(clippy::many_single_char_names)]
use crate::math::prelude::{Vec2, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub tex_coord: Vec2,
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32, v: f32) -> Vertex {
        Vertex {
            position: Vec3::new(x, y, z),
            tex_coord: Vec2::new(u, v),
        }
    }
}

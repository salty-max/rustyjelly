use super::{matrix4x4::Matrix4x4, vec3::Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vec3::zero(),
            rotation: Vec3::zero(),
            scale: Vec3::one(),
        }
    }

    pub fn get_transformation_matrix(&self) -> Matrix4x4 {
        let translation = Matrix4x4::translation(self.position);
        let rotation = Matrix4x4::rotation(self.rotation);
        let scale = Matrix4x4::scale(self.scale);

        translation * rotation * scale
    }
}

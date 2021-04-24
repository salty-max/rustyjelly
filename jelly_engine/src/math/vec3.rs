use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }

    pub fn offset(self, x: f32, y: f32, z: f32) -> Vec3 {
        let x = self.x + x;
        let y = self.y + y;
        let z = self.z + z;

        Vec3::new(x, y, z)
    }

    pub fn scale(self, f: f32) -> Vec3 {
        Vec3::new(self.x * f, self.y * f, self.z * f)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self {
        self.offset(other.x, other.y, other.z)
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Vec3 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        Vec3::new(x, y, z)
    }
}

impl Mul<i32> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: i32) -> Self {
        self.scale(f as f32)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f32) -> Self {
        self.scale(f)
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Eq for Vec3 {}

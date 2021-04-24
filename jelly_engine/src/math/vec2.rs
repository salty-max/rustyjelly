use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2::new(0.0, 0.0)
    }

    pub fn one() -> Vec2 {
        Vec2::new(1.0, 1.0)
    }

    pub fn offset(self, x: f32, y: f32) -> Vec2 {
        let x = self.x + x;
        let y = self.y + y;

        Vec2::new(x, y)
    }

    pub fn scale(self, f: f32) -> Vec2 {
        Vec2::new(self.x * f, self.y * f)
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Self) -> Self {
        self.offset(other.x, other.y)
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, other: Self) -> Vec2 {
        let x = self.x * other.x;
        let y = self.y * other.y;

        Vec2::new(x, y)
    }
}

impl Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, f: i32) -> Self {
        self.scale(f as f32)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, f: f32) -> Self {
        self.scale(f)
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Vec2 {}

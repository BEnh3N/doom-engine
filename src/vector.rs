use std::ops::{AddAssign, Sub};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub fn dist<V: Into<Vec2>>(&self, v: V) -> i32 {
        let v = v.into();
        (((v.x - self.x).pow(2) + (v.y - self.y).pow(2)) as f32).sqrt() as i32
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

pub fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

pub fn vec3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3 { x, y, z }
}

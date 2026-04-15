#[derive(Default, Debug)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    // pub const ZERO: Self = Self { x: 0, y: 0 };

    // pub fn new(x: i32, y: i32) -> Self {
    //     Self { x, y }
    // }
}

pub fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Default, Debug)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };
}

pub fn vec3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3 { x, y, z }
}

use std::f32::consts::PI;

pub mod engine;
mod player;
mod texture;
mod vector;

pub const WIDTH: i32 = 160;
pub const HEIGHT: i32 = 120;
pub const SCALE: i32 = 6;

pub const FOV: i32 = 200;

pub const W2: i32 = WIDTH / 2;
pub const H2: i32 = HEIGHT / 2;

fn cos(d: i32) -> f32 {
    (d as f32 * PI / 180.0).cos()
}

fn sin(d: i32) -> f32 {
    (d as f32 * PI / 180.0).sin()
}

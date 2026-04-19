use std::mem::swap;

use crate::{
    cos,
    engine::{clip_behind_player, draw_surface, sector::Sector},
    player::Player,
    sin,
    texture::Texture,
    vector::{Vec2, Vec3},
    FOV, H2, W2,
};

use raylib::prelude::*;

pub struct Wall {
    p1: Vec2, // bottom line point 1
    p2: Vec2, // bottom line point 2
    // c: Color,
    pub wt: usize,
    pub u: i32,
    pub v: i32,
    pub shade: i32,
}

impl Wall {
    pub fn new(p1: Vec2, p2: Vec2, wt: usize, u: i32, v: i32, shade: i32) -> Self {
        Self {
            p1,
            p2,
            // c: Color::default(),
            wt,
            u,
            v,
            shade,
        }
    }

    pub fn draw(
        &self,
        sector: &mut Sector,
        front_back: i32,
        p: &Player,
        textures: &Vec<Texture>,
        d: &mut RaylibDrawHandle,
    ) {
        let cs = cos(p.a);
        let sn = sin(p.a);

        // offset bottom 2 points by player
        let mut p1 = self.p1 - p.pos.xy();
        let mut p2 = self.p2 - p.pos.xy();

        if front_back == 1 {
            swap(&mut p1, &mut p2);
        }

        let mut w = [Vec3::ZERO; 4];

        // world x position
        w[0].x = (p1.x as f32 * cs - p1.y as f32 * sn) as i32;
        w[1].x = (p2.x as f32 * cs - p2.y as f32 * sn) as i32;
        w[2].x = w[0].x;
        w[3].x = w[1].x;

        // world y position
        w[0].y = (p1.y as f32 * cs + p1.x as f32 * sn) as i32;
        w[1].y = (p2.y as f32 * cs + p2.x as f32 * sn) as i32;
        w[2].y = w[0].y;
        w[3].y = w[1].y;

        // world z height
        w[0].z = ((sector.z1 - p.pos.z) as f32 + ((p.l * w[0].y) as f32 / 32.0)) as i32;
        w[1].z = ((sector.z1 - p.pos.z) as f32 + ((p.l * w[1].y) as f32 / 32.0)) as i32;
        w[2].z = ((sector.z2 - p.pos.z) as f32 + ((p.l * w[0].y) as f32 / 32.0)) as i32;
        w[3].z = ((sector.z2 - p.pos.z) as f32 + ((p.l * w[1].y) as f32 / 32.0)) as i32;

        sector.d += Vec2::ZERO.dist(((w[0].x + w[1].x) / 2, (w[0].y + w[1].y) / 2)); // store this wall distance

        // dont draw if behind player
        if w[0].y < 1 && w[1].y < 1 {
            return; // wall behind player, don't draw
        }
        if w[0].y < 1 {
            let p2 = w[1];
            clip_behind_player(&mut w[0], p2);
            let p2 = w[3];
            clip_behind_player(&mut w[2], p2);
        }
        if w[1].y < 1 {
            let p2 = w[0];
            clip_behind_player(&mut w[1], p2);
            let p2 = w[2];
            clip_behind_player(&mut w[3], p2);
        }

        // screen x, screen y positions
        w[0].x = w[0].x * FOV / w[0].y + W2;
        w[0].y = w[0].z * FOV / w[0].y + H2;
        w[1].x = w[1].x * FOV / w[1].y + W2;
        w[1].y = w[1].z * FOV / w[1].y + H2;
        w[2].x = w[2].x * FOV / w[2].y + W2;
        w[2].y = w[2].z * FOV / w[2].y + H2;
        w[3].x = w[3].x * FOV / w[3].y + W2;
        w[3].y = w[3].z * FOV / w[3].y + H2;

        draw_surface(
            w[0].x, w[1].x, w[0].y, w[1].y, w[2].y, w[3].y, front_back, sector, self, textures, p,
            d,
        );
    }
}

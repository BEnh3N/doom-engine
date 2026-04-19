use std::ops::Range;

use raylib::prelude::*;

use crate::{engine::wall::Wall, player::Player, texture::Texture, HEIGHT, WIDTH};

pub struct Sector {
    w: Range<usize>, // wall number start and end
    pub z1: i32,     // height of bottom and top
    pub z2: i32,
    pub d: i32, // add y distances to sort drawing order
    // pub c1: Color, // bottom and top color
    // pub c2: Color,
    pub st: usize,
    pub ss: i32,
    pub surf: [i32; WIDTH as usize], // to hold points for surfaces
    pub surface: i32,                // is there a surface to draw?
}

impl Sector {
    pub fn new(w: Range<usize>, z1: i32, z2: i32, st: usize, ss: i32) -> Self {
        Self {
            w,
            z1,
            z2,
            d: 0,
            // c1: Color::default(),
            // c2: Color::default(),
            st,
            ss,
            surf: [0; _],
            surface: 0,
        }
    }

    pub fn draw(
        &mut self,
        walls: &Vec<Wall>,
        p: &Player,
        textures: &Vec<Texture>,
        d: &mut RaylibDrawHandle,
    ) {
        self.d = 0; // clear distance
        let cycles;

        if p.pos.z < self.z1 {
            self.surface = 1; // bottom surface
            cycles = 2;
            for x in 0..WIDTH {
                self.surf[x as usize] = HEIGHT;
            }
        } else if p.pos.z > self.z2 {
            self.surface = 2; // top surface
            cycles = 2;
            for x in 0..WIDTH {
                self.surf[x as usize] = 0;
            }
        } else {
            self.surface = 0; // no surfaces
            cycles = 1
        }

        for front_back in 0..cycles {
            for w in self.w.clone() {
                walls[w].draw(self, front_back, p, textures, d);
            }

            self.d /= (self.w.end - self.w.start) as i32;
        }
    }
}

use std::{fs, mem::swap, ops::Range, path::Path};

use raylib::prelude::*;

use crate::{
    cos, dist,
    player::Player,
    sin,
    vector::{vec2, Vec2},
    H2, HEIGHT, W2, WIDTH,
};

const COLORS: [Color; 9] = [
    Color::new(255, 255, 0, 255), // yellow
    Color::new(160, 160, 0, 255), // yellow darker
    Color::new(0, 255, 0, 255),   // green
    Color::new(0, 160, 0, 255),   // green darker
    Color::new(0, 255, 255, 255), // cyan
    Color::new(0, 160, 160, 255), // cyan darker
    Color::new(160, 100, 0, 255), // brown
    Color::new(110, 50, 0, 255),  // brown darker
    Color::new(0, 60, 130, 255),  // background
];

pub struct Engine {
    pub p: Player,
    walls: Vec<Wall>,
    sectors: Vec<Sector>,
}

impl Engine {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(COLORS[8]);

        // order sectors by distance
        self.sectors.sort_by(|a, b| b.d.cmp(&a.d));

        // draw sectors
        for sector in &mut self.sectors {
            sector.draw(&self.walls, &self.p, d);
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            p: Player::default(),
            walls: vec![],
            sectors: vec![],
        }
    }

    pub fn read_world_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let file = fs::read_to_string(path).unwrap();

        for line in file.lines() {
            let mut tokens = line.split_whitespace();

            // take the first token; should be the type identifier
            if let Some(i) = tokens.next() {
                // collect remaining tokens into an array of ints
                let v = tokens
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                // assign new sector or wall based on first token value
                match i {
                    // sector
                    "s" => self.sectors.push(Sector::new(
                        v[0] as usize..v[1] as usize,
                        v[2],
                        v[3],
                        COLORS[v[4] as usize],
                        COLORS[v[5] as usize],
                    )),
                    // wall
                    "w" => self.walls.push(Wall::new(
                        vec2(v[0], v[1]),
                        vec2(v[2], v[3]),
                        COLORS[v[4] as usize],
                    )),
                    // ignore everything else
                    _ => {}
                }
            }
        }
    }
}

struct Wall {
    p1: Vec2, // bottom line point 1
    p2: Vec2, // bottom line point 2
    c: Color,
}

impl Wall {
    fn new(p1: Vec2, p2: Vec2, c: Color) -> Self {
        Self { p1, p2, c }
    }

    fn draw(&self, sector: &mut Sector, flip: i32, p: &Player, d: &mut RaylibDrawHandle) {
        let mut wx = [0; 4];
        let mut wy = [0; 4];
        let mut wz = [0; 4];

        let cs = cos(p.a);
        let sn = sin(p.a);

        // offset bottom 2 points by player
        let mut x1 = self.p1.x - p.x;
        let mut y1 = self.p1.y - p.y;
        let mut x2 = self.p2.x - p.x;
        let mut y2 = self.p2.y - p.y;

        if flip == 0 {
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        // world x position
        wx[0] = (x1 as f32 * cs - y1 as f32 * sn) as i32;
        wx[1] = (x2 as f32 * cs - y2 as f32 * sn) as i32;
        wx[2] = wx[0];
        wx[3] = wx[1];

        // world y position
        wy[0] = (y1 as f32 * cs + x1 as f32 * sn) as i32;
        wy[1] = (y2 as f32 * cs + x2 as f32 * sn) as i32;
        wy[2] = wy[0];
        wy[3] = wy[1];

        sector.d += dist(0, 0, (wx[0] + wx[1]) / 2, (wy[0] + wy[1]) / 2); // store this wall distance

        // world z height
        wz[0] = ((sector.z1 - p.z) as f32 + ((p.l * wy[0]) as f32 / 32.0)) as i32;
        wz[1] = ((sector.z1 - p.z) as f32 + ((p.l * wy[1]) as f32 / 32.0)) as i32;
        wz[2] = wz[0] + sector.z2;
        wz[3] = wz[1] + sector.z2;

        // dont draw if behind player
        if wy[0] < 1 && wy[1] < 1 {
            return; // wall behind player, don't draw
        }
        if wy[0] < 1 {
            let x2 = wx[1];
            let y2 = wy[1];
            let z2 = wz[1];
            clip_behind_player(&mut wx[0], &mut wy[0], &mut wz[0], x2, y2, z2);
            let x2 = wx[3];
            let y2 = wy[3];
            let z2 = wz[3];
            clip_behind_player(&mut wx[2], &mut wy[2], &mut wz[2], x2, y2, z2);
        }
        if wy[1] < 1 {
            let x2 = wx[0];
            let y2 = wy[0];
            let z2 = wz[0];
            clip_behind_player(&mut wx[1], &mut wy[1], &mut wz[1], x2, y2, z2);
            let x2 = wx[2];
            let y2 = wy[2];
            let z2 = wz[2];
            clip_behind_player(&mut wx[3], &mut wy[3], &mut wz[3], x2, y2, z2);
        }

        // screen x, screen y positions
        wx[0] = wx[0] * 200 / wy[0] + W2;
        wy[0] = wz[0] * 200 / wy[0] + H2;
        wx[1] = wx[1] * 200 / wy[1] + W2;
        wy[1] = wz[1] * 200 / wy[1] + H2;
        wx[2] = wx[2] * 200 / wy[2] + W2;
        wy[2] = wz[2] * 200 / wy[2] + H2;
        wx[3] = wx[3] * 200 / wy[3] + W2;
        wy[3] = wz[3] * 200 / wy[3] + H2;

        draw_surface(d, wx[0], wx[1], wy[0], wy[1], wy[2], wy[3], self.c, sector);
    }
}

struct Sector {
    w: Range<usize>, // wall number start and end
    z1: i32,         // height of bottom and top
    z2: i32,
    d: i32,    // add y distances to sort drawing order
    c1: Color, // bottom and top color
    c2: Color,
    surf: [i32; WIDTH as usize], // to hold points for surfaces
    surface: i32,                // is there a surface to draw?
}

impl Sector {
    fn new(w: Range<usize>, z1: i32, z2: i32, c1: Color, c2: Color) -> Self {
        Self {
            w,
            z1,
            z2,
            d: 0,
            c1,
            c2,
            surf: [0; _],
            surface: 0,
        }
    }

    fn draw(&mut self, walls: &Vec<Wall>, p: &Player, d: &mut RaylibDrawHandle) {
        self.d = 0; // clear distance

        if p.z < self.z1 {
            self.surface = 1; // bottom surface
        } else if p.z > self.z2 {
            self.surface = 2; // top surface
        } else {
            self.surface = 0; // no surfaces
        }

        for flip in 0..=1 {
            for w in self.w.clone() {
                let wall = &walls[w];

                wall.draw(self, flip, p, d);
            }

            self.d /= (self.w.end - self.w.start) as i32;
            self.surface *= -1;
        }
    }
}

fn draw_surface(
    d: &mut RaylibDrawHandle,
    mut x1: i32,
    mut x2: i32,
    b1: i32,
    b2: i32,
    t1: i32,
    t2: i32,
    c: Color,
    sector: &mut Sector,
) {
    let dyb = b2 - b1;
    let dyt = t2 - t1;
    let mut dx = x2 - x1;
    if dx == 0 {
        dx = 1
    }
    let xs = x1;

    // clip x
    if x1 < 1 {
        x1 = 1
    }
    if x2 < 1 {
        x2 = 1
    }
    if x1 > WIDTH - 1 {
        x1 = WIDTH - 1
    }
    if x2 > WIDTH - 1 {
        x2 = WIDTH - 1
    }

    for x in x1..x2 {
        let mut y1 = (dyb as f32 * ((x - xs) as f32 + 0.5) / dx as f32 + b1 as f32) as i32;
        let mut y2 = (dyt as f32 * ((x - xs) as f32 + 0.5) / dx as f32 + t1 as f32) as i32;

        // clip y
        if y1 < 1 {
            y1 = 1
        }
        if y2 < 1 {
            y2 = 1
        }
        if y1 > HEIGHT - 1 {
            y1 = HEIGHT - 1
        }
        if y2 > HEIGHT - 1 {
            y2 = HEIGHT - 1
        }

        if sector.surface == 1 {
            sector.surf[x as usize] = y1;
            continue;
        }
        if sector.surface == 2 {
            sector.surf[x as usize] = y2;
            continue;
        }
        if sector.surface == -1 {
            for y in sector.surf[x as usize]..y1 {
                d.draw_pixel(x, y, sector.c1);
            }
        }
        if sector.surface == -2 {
            for y in y2..sector.surf[x as usize] {
                d.draw_pixel(x, y, sector.c2);
            }
        }

        for y in y1..y2 {
            d.draw_pixel(x, y, c);
        }
    }
}

fn clip_behind_player(x1: &mut i32, y1: &mut i32, z1: &mut i32, x2: i32, y2: i32, z2: i32) {
    let da = *y1 as f32;
    let db = y2 as f32;

    // let mut d = da - db;
    // if d == 0.0 {
    //     d = 1.0
    // }

    let s = da / (da - db);

    *x1 = (*x1 as f32 + s * (x2 - *x1) as f32) as i32;
    *y1 = (*y1 as f32 + s * (y2 - *y1) as f32) as i32;
    if *y1 == 0 {
        *y1 = 1
    }
    *z1 = (*z1 as f32 + s * (z2 - *z1) as f32) as i32;
}

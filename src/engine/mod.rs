use std::{fs, path::Path};

use raylib::prelude::*;

mod sector;
mod wall;

use crate::{
    cos,
    engine::{sector::Sector, wall::Wall},
    player::Player,
    sin,
    texture::Texture,
    vector::{vec2, vec3, Vec3},
    FOV, H2, HEIGHT, W2, WIDTH,
};

pub struct Engine {
    pub p: Player,
    walls: Vec<Wall>,
    sectors: Vec<Sector>,
    textures: Vec<Texture>,
}

impl Engine {
    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(0, 60, 130, 255));

        // self.floors(d);

        // order sectors by distance
        self.sectors.sort_by(|a, b| b.d.cmp(&a.d));

        // draw sectors
        for sector in &mut self.sectors {
            sector.draw(&self.walls, &self.p, &self.textures, d);
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            p: Player::default(),
            walls: vec![],
            sectors: vec![],
            textures: vec![],
        }
    }

    pub fn load_level_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let file = fs::read_to_string(path).unwrap();
        self.sectors = vec![];
        self.walls = vec![];

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
                        v[4] as usize,
                        v[5],
                    )),
                    // wall
                    "w" => self.walls.push(Wall::new(
                        vec2(v[0], v[1]),
                        vec2(v[2], v[3]),
                        v[4] as usize,
                        v[5],
                        v[6],
                        v[7],
                    )),
                    "p" => {
                        self.p.pos.x = v[0];
                        self.p.pos.y = v[1];
                        self.p.pos.z = v[2];
                        self.p.a = v[3];
                        self.p.l = v[4];
                    }
                    // ignore everything else
                    _ => {}
                }
            }
        }
    }

    pub fn load_default_level(&mut self) {
        self.load_level_from_file("assets/levels/level.lvl");
    }

    pub fn load_texture_from_file<P: AsRef<Path>>(&mut self, path: P) {
        let file = fs::read_to_string(path).unwrap();

        let mut tokens = file.split(',');
        let height = tokens.next().unwrap().trim().parse::<i32>().unwrap();
        let width = tokens.next().unwrap().trim().parse::<i32>().unwrap();

        let data = tokens
            .map(|s| s.trim().parse().unwrap())
            .collect::<Vec<u8>>()
            .chunks_exact(width as usize * 3)
            .map(|row| {
                row.chunks_exact(3)
                    .map(|c| Color::new(c[0], c[1], c[2], 0xff))
                    .collect::<Vec<Color>>()
            })
            .collect::<Vec<Vec<Color>>>();

        let t = Texture {
            data,
            width,
            height,
        };
        self.textures.push(t);
    }

    // fn floors(&self, d: &mut RaylibDrawHandle) {
    //     let xo = W2;
    //     let yo = H2;
    //     let mut look_up_down = -self.p.l as f32 * 2.0;
    //     if look_up_down > HEIGHT as f32 {
    //         look_up_down = HEIGHT as f32;
    //     }
    //     let mut move_up_down = self.p.pos.z as f32 / 16.0;
    //     if move_up_down == 0.0 {
    //         move_up_down = 0.0001;
    //     }
    //     let mut ys = -yo;
    //     let mut ye = -look_up_down as i32;
    //     if move_up_down < 0.0 {
    //         ys = -look_up_down as i32;
    //         ye = yo + look_up_down as i32
    //     }

    //     for y in ys..ye as i32 {
    //         for x in -xo..xo {
    //             let mut z = y as f32 + look_up_down;
    //             if z == 0.0 {
    //                 z = 0.0001;
    //             }
    //             let fx = x as f32 / z * move_up_down;
    //             let fy = FOV as f32 / z * move_up_down;
    //             let mut rx = fx * sin(self.p.a) - fy * cos(self.p.a) + (self.p.pos.y as f32 / 30.0);
    //             let mut ry = fx * cos(self.p.a) + fy * sin(self.p.a) - (self.p.pos.x as f32 / 30.0);

    //             if rx < 0.0 {
    //                 rx = -rx + 1.0;
    //             }
    //             if ry < 0.0 {
    //                 ry = -ry + 1.0;
    //             }
    //             if rx <= 0.0 || ry <= 0.0 || rx >= 5.0 || ry >= 5.0 {
    //                 continue;
    //             }
    //             if (rx as i32) % 2 == (ry as i32) % 2 {
    //                 d.draw_pixel(x + xo, y + yo, Color::new(255, 0, 0, 255));
    //             } else {
    //                 d.draw_pixel(x + xo, y + yo, Color::new(0, 255, 0, 255));
    //             }
    //         }
    //     }
    // }
}

fn draw_surface(
    mut x1: i32,
    mut x2: i32,
    b1: i32,
    b2: i32,
    t1: i32,
    t2: i32,
    front_back: i32,
    sector: &mut Sector,
    wall: &Wall,
    textures: &Vec<Texture>,
    p: &Player,
    d: &mut RaylibDrawHandle,
) {
    let texture = &textures[wall.wt as usize];

    let mut ht = 0.0;
    let ht_step = (texture.width * wall.u) as f32 / (x2 - x1) as f32;

    let dyb = b2 - b1;
    let dyt = t2 - t1;
    let mut dx = x2 - x1;
    if dx == 0 {
        dx = 1
    }
    let xs = x1;

    // clip x
    if x1 < 0 {
        ht -= ht_step * x1 as f32;
    }
    x1 = x1.clamp(0, WIDTH);
    x2 = x2.clamp(0, WIDTH);

    for x in x1..x2 {
        let mut y1 = (dyb as f32 * ((x - xs) as f32 + 0.5) / dx as f32 + b1 as f32) as i32;
        let mut y2 = (dyt as f32 * ((x - xs) as f32 + 0.5) / dx as f32 + t1 as f32) as i32;

        let mut vt = 0.0;
        let vt_step = (texture.height * wall.v) as f32 / (y2 - y1) as f32;

        // clip y
        if y1 < 0 {
            vt -= vt_step * y1 as f32;
        }
        y1 = y1.clamp(0, HEIGHT);
        y2 = y2.clamp(0, HEIGHT);

        if front_back == 0 {
            if sector.surface == 1 {
                sector.surf[x as usize] = y1;
            }
            if sector.surface == 2 {
                sector.surf[x as usize] = y2;
            }
            for y in y1..y2 {
                let c = texture.data[(texture.height - (vt as i32 % texture.height) - 1) as usize]
                    [ht as usize % texture.width as usize];
                let shaded_c = Color::new(
                    (c.r as i32 - wall.shade / 2).clamp(0, 255) as u8,
                    (c.g as i32 - wall.shade / 2).clamp(0, 255) as u8,
                    (c.b as i32 - wall.shade / 2).clamp(0, 255) as u8,
                    0xff,
                );
                // normal wall
                d.draw_pixel(x, y, shaded_c);
                vt += vt_step;
            }
            ht += ht_step;
        }

        if front_back == 1 {
            let xo = W2; // x offset
            let yo = H2; // y offset
            let x2 = x - xo; // x - x offset
            let mut wo = 0; // wall offset
            let tile = sector.ss as f32 * 7.0; // imported surface tile

            if sector.surface == 1 {
                y2 = sector.surf[x as usize]; // bottom surface
                wo = sector.z1;
            }
            if sector.surface == 2 {
                y1 = sector.surf[x as usize]; // top surface
                wo = sector.z2;
            }

            let mut look_up_down = -p.l as f32 * 6.2;
            if look_up_down > HEIGHT as f32 {
                look_up_down = HEIGHT as f32;
            }
            let mut move_up_down = (p.pos.z - wo) as f32 / yo as f32;
            if move_up_down == 0.0 {
                move_up_down = 0.001;
            }
            let ys = y1 - yo; // y start and y end
            let ye = y2 - yo;

            for y in ys..ye as i32 {
                let mut z = y as f32 + look_up_down;
                if z == 0.0 {
                    z = 0.0001;
                }
                let fx = x2 as f32 / z * move_up_down * tile;
                let fy = FOV as f32 / z * move_up_down * tile;
                let mut rx = fx * sin(p.a) - fy * cos(p.a) + (p.pos.y as f32 / 60.0 * tile);
                let mut ry = fx * cos(p.a) + fy * sin(p.a) - (p.pos.x as f32 / 60.0 * tile);

                if rx < 0.0 {
                    rx = -rx + 1.0;
                }
                if ry < 0.0 {
                    ry = -ry + 1.0;
                }

                // textures
                let st = sector.st; // surface texture
                let t = &textures[st];
                let c = t.data[(t.height - (ry as i32 % t.height) - 1) as usize]
                    [rx as usize % t.width as usize];
                d.draw_pixel(x2 + xo, y + yo, c);
            }
        }
    }
}

fn clip_behind_player(p1: &mut Vec3, p2: Vec3) {
    let da = p1.y as f32;
    let db = p2.y as f32;

    // let mut d = da - db;
    // if d == 0.0 {
    //     d = 1.0
    // }

    let s = da / (da - db);

    *p1 += vec3(
        (s * (p2.x - p1.x) as f32) as i32,
        (s * (p2.y - p1.y) as f32) as i32,
        (s * (p2.z - p1.z) as f32) as i32,
    );
    if p1.y == 0 {
        p1.y = 1
    }
}

use std::f32::consts::PI;

use raylib::prelude::*;

const WIDTH: i32 = 160;
const HEIGHT: i32 = 120;
const SCALE: i32 = 4;
const SCREEN_WIDTH: i32 = WIDTH * SCALE;
const SCREEN_HEIGHT: i32 = HEIGHT * SCALE;

const COLORS: [Color; 9] = [
    Color::new(255, 255, 0, 255),
    Color::new(160, 160, 0, 255),
    Color::new(0, 255, 0, 255),
    Color::new(0, 160, 0, 255),
    Color::new(0, 255, 255, 255),
    Color::new(0, 160, 160, 255),
    Color::new(160, 100, 0, 255),
    Color::new(110, 50, 0, 255),
    Color::new(0, 60, 130, 255),
];

struct Engine {
    k: Keys,
    m: Math,
    p: Player,
}

struct Math {
    cos: [f32; 360],
    sin: [f32; 360],
}

struct Player {
    x: i32,
    y: i32,
    z: i32,
    a: i32,
    l: i32,
}

#[derive(Default)]
struct Keys {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
    sl: bool,
    sr: bool,
    m: bool,
}

fn main() {
    // Set up the window
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Doom Engine")
        .build();

    // Create a render texture to draw to for pixel-perfect scaling
    let mut target = rl
        .load_render_texture(&thread, WIDTH as u32, HEIGHT as u32)
        .unwrap();

    let mut engine = Engine::init();

    rl.set_target_fps(20);

    while !rl.window_should_close() {
        engine.k.w = rl.is_key_down(KeyboardKey::KEY_W);
        engine.k.a = rl.is_key_down(KeyboardKey::KEY_A);
        engine.k.s = rl.is_key_down(KeyboardKey::KEY_S);
        engine.k.d = rl.is_key_down(KeyboardKey::KEY_D);

        engine.k.sl = rl.is_key_down(KeyboardKey::KEY_COMMA);
        engine.k.sr = rl.is_key_down(KeyboardKey::KEY_PERIOD);
        engine.k.m = rl.is_key_down(KeyboardKey::KEY_M);

        engine.move_player();

        let mut d = rl.begin_drawing(&thread);

        // Do drawing here on the render texture instead of the screen
        {
            let mut texture = d.begin_texture_mode(&thread, &mut target);
            engine.draw(&mut texture);
        }

        // Red color signifies that the render texture is being drawn incorrectly
        d.clear_background(Color::RED);
        // Draw the render texture to the screen
        d.draw_texture_ex(
            &target.texture(),
            Vector2::zero(),
            0.,
            SCALE as f32,
            Color::WHITE,
        );
        // d.draw_fps(5, 5);
    }
}

impl Engine {
    fn init() -> Self {
        let k = Keys::default();
        let m = Math {
            cos: (0..360)
                .map(|x| (x as f32 / 180.0 * PI).cos())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
            sin: (0..360)
                .map(|x| (x as f32 / 180.0 * PI).sin())
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        };
        let p = Player {
            x: 70,
            y: -110,
            z: 20,
            a: 0,
            l: 0,
        };
        Self { k, m, p }
    }

    fn draw(&mut self, draw: &mut RaylibTextureMode<RaylibDrawHandle>) {
        let p = &mut self.p;
        let m = &mut self.m;

        draw.clear_background(COLORS[8]);

        let cs = m.cos[p.a as usize];
        let sn = m.sin[p.a as usize];

        // offset bottom 2 points by player
        let x1 = 40 - p.x;
        let y1 = 10 - p.y;
        let x2 = 40 - p.x;
        let y2 = 290 - p.y;

        // world x position
        let mut wx = [0; 4];
        wx[0] = (x1 as f32 * cs - y1 as f32 * sn) as i32;
        wx[1] = (x2 as f32 * cs - y2 as f32 * sn) as i32;
        wx[2] = wx[0];
        wx[3] = wx[1];

        // world y position (depth)
        let mut wy = [0; 4];
        wy[0] = (y1 as f32 * cs + x1 as f32 * sn) as i32;
        wy[1] = (y2 as f32 * cs + x2 as f32 * sn) as i32;
        wy[2] = wy[0];
        wy[3] = wy[1];

        // world z height
        let mut wz = [0; 4];
        wz[0] = 0 - p.z + ((p.l * wy[0]) as f32 / 32.) as i32;
        wz[1] = 0 - p.z + ((p.l * wy[1]) as f32 / 32.) as i32;
        wz[2] = wz[0] + 40;
        wz[3] = wz[1] + 40;

        if wy[0] < 1 && wy[1] < 1 {
            return;
        }

        if wy[0] < 1 {
            let wx1 = wx[1];
            let wy1 = wy[1];
            let wz1 = wz[1];
            clip_behind_player(&mut wx[0], &mut wy[0], &mut wz[0], wx1, wy1, wz1);
            let wx3 = wx[3];
            let wy3 = wy[3];
            let wz3 = wz[3];
            clip_behind_player(&mut wx[2], &mut wy[2], &mut wz[2], wx3, wy3, wz3);
        }

        if wy[1] < 1 {
            let wx0 = wx[0];
            let wy0 = wy[0];
            let wz0 = wz[0];
            clip_behind_player(&mut wx[1], &mut wy[1], &mut wz[1], wx0, wy0, wz0);
            let wx2 = wx[2];
            let wy2 = wy[2];
            let wz2 = wz[2];
            clip_behind_player(&mut wx[3], &mut wy[3], &mut wz[3], wx2, wy2, wz2);
        }

        // screen x, screen y position
        wx[0] = wx[0] * 200 / wy[0] + (WIDTH / 2);
        wy[0] = wz[0] * 200 / wy[0] + (HEIGHT / 2);
        wx[1] = wx[1] * 200 / wy[1] + (WIDTH / 2);
        wy[1] = wz[1] * 200 / wy[1] + (HEIGHT / 2);
        wx[2] = wx[2] * 200 / wy[2] + (WIDTH / 2);
        wy[2] = wz[2] * 200 / wy[2] + (HEIGHT / 2);
        wx[3] = wx[3] * 200 / wy[3] + (WIDTH / 2);
        wy[3] = wz[3] * 200 / wy[3] + (HEIGHT / 2);

        // draw points
        draw_wall(wx[0], wx[1], wy[0], wy[1], wy[2], wy[3], draw);
    }

    fn move_player(&mut self) {
        let p = &mut self.p;
        let m = &mut self.m;
        let k = &self.k;

        let dx = (m.sin[p.a as usize] * 10.0) as i32;
        let dy = (m.cos[p.a as usize] * 10.0) as i32;

        if !k.m {
            if k.a {
                p.a -= 4;
                if p.a < 0 {
                    p.a += 360
                }
            }
            if k.d {
                p.a += 4;
                if p.a > 359 {
                    p.a -= 360
                }
            }
            if k.w {
                p.x += dx;
                p.y += dy;
            }
            if k.s {
                p.x -= dx;
                p.y -= dy;
            }
        } else {
            if k.a {
                p.l -= 1
            }
            if k.d {
                p.l += 1
            }
            if k.w {
                p.z -= 4
            }
            if k.s {
                p.z += 4
            }
        }

        if k.sr {
            p.x += dy;
            p.y -= dx;
        }
        if k.sl {
            p.x -= dy;
            p.y += dx;
        }
    }
}

fn clip_behind_player(x1: &mut i32, y1: &mut i32, z1: &mut i32, x2: i32, y2: i32, z2: i32) {
    let da = *y1 as f32;
    let db = y2 as f32;
    let mut d = da - db;
    if d == 0.0 {
        d = 1.0;
    }
    let s = da / (da - db);
    println!("s: {}", s);

    *x1 = *x1 + (s * (x2 - *x1) as f32) as i32;
    *y1 = *y1 + (s * (y2 - *y1) as f32) as i32;
    if *y1 == 0 {
        *y1 = 1;
    }
    *z1 = *z1 + (s * (z2 - *z1) as f32) as i32;
}

fn draw_wall(
    mut x1: i32,
    mut x2: i32,
    b1: i32,
    b2: i32,
    t1: i32,
    t2: i32,
    draw: &mut RaylibTextureMode<RaylibDrawHandle>,
) {
    let dyb = b2 - b1;
    let dyt = t2 - t1;
    let mut dx = x2 - x1;
    if dx == 0 {
        dx = 1
    }
    let xs = x1;
    if x1 < 1 {
        x1 = 1;
    }
    if x2 < 1 {
        x2 = 1;
    }
    if x1 > WIDTH - 1 {
        x1 = WIDTH - 1;
    }
    if x2 > WIDTH - 1 {
        x2 = WIDTH - 1;
    }

    for x in x1..x2 {
        let mut y1 = (dyb as f32 * ((x - xs) as f32 + 0.5) / dx as f32) as i32 + b1;
        let mut y2 = (dyt as f32 * ((x - xs) as f32 + 0.5) / dx as f32) as i32 + t1;

        if y1 < 1 {
            y1 = 1;
        }
        if y2 < 1 {
            y2 = 1;
        }
        if y1 > HEIGHT - 1 {
            y1 = HEIGHT - 1;
        }
        if y2 > HEIGHT - 1 {
            y2 = HEIGHT - 1;
        }

        for y in y1..y2 {
            draw.draw_pixel(x, y, COLORS[0]);
        }
    }
}

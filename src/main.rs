use std::{
    f32::consts::PI,
    time::{Duration, Instant},
};

use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const WIDTH: i32 = 160;
const HEIGHT: i32 = 120;
const SCALE: i32 = 4;

struct Engine {
    tick: i32,
    time: Instant,
    keys: Keys,
    math: Math,
    player: Player,
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
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH * SCALE, HEIGHT * SCALE);
        WindowBuilder::new()
            .with_title("Doom Engine")
            .with_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
    };

    let mut engine = Engine::init();

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if engine.time.elapsed() > Duration::from_millis(50) {
                engine.move_player();

                engine.draw(pixels.frame_mut());
                pixels.render().unwrap();

                engine.time = Instant::now();
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            engine.keys.w = input.key_held(VirtualKeyCode::W);
            engine.keys.a = input.key_held(VirtualKeyCode::A);
            engine.keys.s = input.key_held(VirtualKeyCode::S);
            engine.keys.d = input.key_held(VirtualKeyCode::D);

            engine.keys.sl = input.key_held(VirtualKeyCode::Period);
            engine.keys.sr = input.key_held(VirtualKeyCode::Comma);
            engine.keys.m = input.key_held(VirtualKeyCode::M);

            window.request_redraw();
        }
    });
}

impl Engine {
    fn init() -> Self {
        let tick = 0;
        let time = Instant::now();
        let keys = Keys {
            w: false,
            a: false,
            s: false,
            d: false,
            sl: false,
            sr: false,
            m: false,
        };
        let math = Math {
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
        let player = Player {
            x: 70,
            y: -110,
            z: 20,
            a: 0,
            l: 0,
        };
        Self {
            tick,
            time,
            keys,
            math,
            player,
        }
    }

    fn draw(&mut self, frame: &mut [u8]) {
        let p = &mut self.player;
        let m = &mut self.math;

        clear_background(frame, 8);

        let mut wx = [0; 4];
        let mut wy = [0; 4];
        let mut wz = [0; 4];

        let cs = m.cos[p.a as usize];
        let sn = m.sin[p.a as usize];

        let x1 = 40 - p.x;
        let y1 = 10 - p.y;
        let x2 = 40 - p.x;
        let y2 = 290 - p.y;

        wx[0] = (x1 as f32 * cs - y1 as f32 * sn) as i32;
        wx[1] = (x2 as f32 * cs - y2 as f32 * sn) as i32;
        wx[2] = wx[0];
        wx[3] = wx[1];

        wy[0] = (y1 as f32 * cs + x1 as f32 * sn) as i32;
        wy[1] = (y2 as f32 * cs + x2 as f32 * sn) as i32;
        wy[2] = wy[0];
        wy[3] = wy[1];

        wz[0] = 0 - p.z + ((p.l * wy[0]) as f32 / 32.0) as i32;
        wz[1] = 0 - p.z + ((p.l * wy[1]) as f32 / 32.0) as i32;
        wz[2] = wz[0] + 40;
        wz[3] = wz[1] + 40;

        if wy[0] < 1 && wy[1] < 1 {
            return;
        }

        // if wy[0] < 1 {
        //     clip_behind_player(&mut wx[0], &mut wy[0], &mut wz[0], wx[1], wy[1], wz[1]);
        // }

        wx[0] = wx[0] * 200 / wy[0] + (WIDTH / 2);
        wy[0] = wz[0] * 200 / wy[0] + (HEIGHT / 2);
        wx[1] = wx[1] * 200 / wy[1] + (WIDTH / 2);
        wy[1] = wz[1] * 200 / wy[1] + (HEIGHT / 2);
        wx[2] = wx[2] * 200 / wy[2] + (WIDTH / 2);
        wy[2] = wz[2] * 200 / wy[2] + (HEIGHT / 2);
        wx[3] = wx[3] * 200 / wy[3] + (WIDTH / 2);
        wy[3] = wz[3] * 200 / wy[3] + (HEIGHT / 2);

        draw_wall(wx[0], wx[1], wy[0], wy[1], wy[2], wy[3], frame);
    }

    fn move_player(&mut self) {
        let p = &mut self.player;
        let m = &mut self.math;

        let dx = m.sin[p.a as usize] * 10.0;
        let dy = m.cos[p.a as usize] * 10.0;

        if !self.keys.m {
            if self.keys.a {
                p.a -= 4;
                if p.a < 0 {
                    p.a += 360
                }
            }
            if self.keys.d {
                p.a += 4;
                if p.a > 359 {
                    p.a -= 360
                }
            }
            if self.keys.w {
                p.x += dx as i32;
                p.y += dy as i32
            }
            if self.keys.s {
                p.x -= dx as i32;
                p.y -= dy as i32
            }
        } else {
            if self.keys.a {
                p.l -= 1
            }
            if self.keys.d {
                p.l += 1
            }
            if self.keys.w {
                p.z -= 4
            }
            if self.keys.s {
                p.z += 4
            }
        }

        if self.keys.sr {
            p.x += dy as i32;
            p.y -= dx as i32
        }
        if self.keys.sl {
            p.x -= dy as i32;
            p.y += dx as i32
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

    *x1 = *x1 + (s * (x2 as f32 - *x1 as f32)) as i32;
    *y1 = *y1 + (s * (y2 as f32 - *y1 as f32)) as i32;
    if *y1 == 0 {
        *y1 = 1;
    }
    *z1 = *z1 + (s * (z2 as f32 - *z1 as f32)) as i32;
}

fn draw_wall(x1: i32, x2: i32, b1: i32, b2: i32, t1: i32, t2: i32, frame: &mut [u8]) {
    let mut x1 = x1;
    let mut x2 = x2;

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
            pixel(x, y, frame, 0);
        }
    }
}

fn get_pixel_index(x: i32, y: i32) -> usize {
    (((y * WIDTH) + x) * 4) as usize
}

fn pixel(x: i32, y: i32, frame: &mut [u8], c: usize) {
    if (x < 0) || (y < 0) || (x >= WIDTH) || (y >= HEIGHT) {
        return;
    }

    let y = HEIGHT as i32 - y - 1;

    let rgba = match c {
        0 => [255, 255, 0, 255],
        1 => [160, 160, 0, 255],
        2 => [0, 255, 0, 255],
        3 => [0, 160, 0, 255],
        4 => [0, 255, 255, 255],
        5 => [0, 160, 160, 255],
        6 => [160, 100, 0, 255],
        7 => [110, 50, 0, 255],
        _ => [0, 60, 130, 255],
    };

    let i = get_pixel_index(x, y);
    let pixel = &mut frame[i..i + 4];
    pixel.copy_from_slice(&rgba);
}

fn clear_background(frame: &mut [u8], c: usize) {
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            pixel(x, y, frame, c);
        }
    }
}

use raylib::prelude::*;

use crate::{cos, sin};

const LEFT: KeyboardKey = KeyboardKey::KEY_A;
const RIGHT: KeyboardKey = KeyboardKey::KEY_D;
const UP: KeyboardKey = KeyboardKey::KEY_W;
const DOWN: KeyboardKey = KeyboardKey::KEY_S;
const STRAFE_LEFT: KeyboardKey = KeyboardKey::KEY_COMMA;
const STRAFE_RIGHT: KeyboardKey = KeyboardKey::KEY_PERIOD;
const MODIFIER: KeyboardKey = KeyboardKey::KEY_M;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub a: i32,
    pub l: i32,
}

impl Player {
    pub fn handle_inputs(&mut self, rl: &RaylibHandle) {
        let dx = (sin(self.a) * 10.0) as i32;
        let dy = (cos(self.a) * 10.0) as i32;

        // modifier key not held
        if !rl.is_key_down(MODIFIER) {
            if rl.is_key_down(LEFT) {
                self.a -= 4;
                if self.a < 0 {
                    self.a += 360
                }
            }
            if rl.is_key_down(RIGHT) {
                self.a += 4;
                if self.a > 359 {
                    self.a -= 360
                }
            }
            if rl.is_key_down(UP) {
                self.x += dx;
                self.y += dy;
            }
            if rl.is_key_down(DOWN) {
                self.x -= dx;
                self.y -= dy;
            }
        } else {
            if rl.is_key_down(LEFT) {
                self.l -= 1;
            }
            if rl.is_key_down(RIGHT) {
                self.l += 1;
            }
            if rl.is_key_down(UP) {
                self.z += 4;
            }
            if rl.is_key_down(DOWN) {
                self.z -= 4;
            }
        }

        if rl.is_key_down(STRAFE_LEFT) {
            self.x -= dy;
            self.y += dx;
        }
        if rl.is_key_down(STRAFE_RIGHT) {
            self.x += dy;
            self.y -= dx;
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            x: 70,
            y: -110,
            z: 20,
            a: 0,
            l: 0,
        }
    }
}

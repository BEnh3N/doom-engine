use raylib::prelude::*;

pub struct Texture {
    pub data: Vec<Vec<Color>>,
    pub width: i32,
    pub height: i32,
}

impl Default for Texture {
    fn default() -> Self {
        let data = vec![
            vec![Color::new(255, 0, 255, 255), Color::new(0, 0, 0, 255)],
            vec![Color::new(0, 0, 0, 255), Color::new(255, 0, 255, 255)],
        ];
        Self {
            data,
            width: 2,
            height: 2,
        }
    }
}

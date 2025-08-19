use raylib::prelude::*;

pub struct Framebuffer {
    width: i32,
    height: i32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            background_color: Color::PINK,
            current_color: Color::WHITE,
        }
    }

    pub fn draw_pixel(&self, d: &mut RaylibDrawHandle, x: i32, y: i32) {
        d.draw_pixel(x, y, self.current_color);
    }
}


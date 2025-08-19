use crate::framebuffer::Framebuffer;
use raylib::drawing::RaylibDrawHandle;

pub struct GameOfLife {
    pub grid: Vec<Vec<bool>>,
    pub width: usize,
    pub height: usize,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: vec![vec![false; width]; height],
            width,
            height,
        }
    }

    pub fn step(&mut self) {
        let mut next = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                next[y][x] = match (self.grid[y][x], neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        self.grid = next;
    }


    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if dx == 0 && dy == 0 { continue; }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    if self.grid[ny as usize][nx as usize] { count += 1; }
                }
            }
        }
        count
    }
}

// Dibuja el grid en el framebuffer, escalando cada celda
pub fn draw_grid(
    game: &GameOfLife,
    framebuffer: &Framebuffer,
    d: &mut RaylibDrawHandle,
    scale: i32,
) {
    for y in 0..game.height {
        for x in 0..game.width {
            if game.grid[y][x] {
                for dy in 0..scale {
                    for dx in 0..scale {
                        framebuffer.draw_pixel(d, (x as i32) * scale + dx, (y as i32) * scale + dy);
                    }
                }
            }
        }
    }
}
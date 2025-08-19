mod framebuffer;
mod game;
mod organisms;

use framebuffer::Framebuffer;
use game::{GameOfLife, draw_grid};
use organisms::{
    add_block, add_blinker, add_glider, add_beehive, add_toad, 
    add_loaf, add_beacon, add_lwss, add_boat, add_tub
};
use raylib::prelude::RaylibDraw;
use raylib::prelude::Color;

fn main() {
    // Tamaño del grid y escala
    let grid_width = 85;
    let grid_height = 85;
    let scale = 10;

    let (mut window, rl) = raylib::init()
        .size(grid_width as i32 * scale, grid_height as i32 * scale)
        .title("Game of Life")
        .build();

    let mut framebuffer = Framebuffer::new(grid_width as i32 * scale, grid_height as i32 * scale);
    let mut game = GameOfLife::new(grid_width, grid_height);

    // Región base
    let region_x = 40; 
    let region_y = 40;

    add_block(&mut game.grid,  region_x+3,  region_y+9);
    add_block(&mut game.grid,  region_x-3,  region_y+9);
    add_block(&mut game.grid,  region_x+9,  region_y+3);
    add_block(&mut game.grid,  region_x-9,  region_y+3);
    add_block(&mut game.grid,  region_x+9,  region_y-3);
    add_block(&mut game.grid,  region_x-9,  region_y-3);
    add_block(&mut game.grid,  region_x+3,  region_y-9);
    add_block(&mut game.grid,  region_x-3,  region_y-9);

    add_beehive(&mut game.grid,  region_x-9,  region_y+18);
    add_beehive(&mut game.grid,  region_x-18,  region_y-9);
    add_beehive(&mut game.grid,  region_x+18,  region_y+9);
    add_beehive(&mut game.grid,  region_x+9,  region_y-18);

    add_loaf(&mut game.grid,  region_x+9,  region_y+18);
    add_loaf(&mut game.grid,  region_x-18,  region_y+9);
    add_loaf(&mut game.grid,  region_x+18,  region_y-9);
    add_loaf(&mut game.grid,  region_x-9,  region_y-18);

    add_boat(&mut game.grid,  region_x-9,  region_y+21);
    add_boat(&mut game.grid,  region_x+21,  region_y+9);
    add_boat(&mut game.grid,  region_x+9,  region_y-21);
    add_boat(&mut game.grid,  region_x-21,  region_y-9);

    add_tub(&mut game.grid,  region_x+9,  region_y+21);
    add_tub(&mut game.grid,  region_x-21,  region_y+9);
    add_tub(&mut game.grid,  region_x+21,  region_y-9);
    add_tub(&mut game.grid,  region_x-9,  region_y-21);

    add_blinker(&mut game.grid,  region_x+3,  region_y+6);
    add_blinker(&mut game.grid,  region_x-3,  region_y+6);
    add_blinker(&mut game.grid,  region_x+9,  region_y+6);
    add_blinker(&mut game.grid,  region_x-9,  region_y+6);
    add_blinker(&mut game.grid,  region_x+3,  region_y-6);
    add_blinker(&mut game.grid,  region_x-3,  region_y-6);
    add_blinker(&mut game.grid,  region_x+9,  region_y-6);
    add_blinker(&mut game.grid,  region_x-9,  region_y-6);

    add_toad(&mut game.grid,  region_x+6,  region_y+9);
    add_toad(&mut game.grid,  region_x-6,  region_y+9);
    add_toad(&mut game.grid,  region_x+6,  region_y+3);
    add_toad(&mut game.grid,  region_x-6,  region_y+3);
    add_toad(&mut game.grid,  region_x+6,  region_y-9);
    add_toad(&mut game.grid,  region_x-6,  region_y-9);
    add_toad(&mut game.grid,  region_x+6,  region_y-3);
    add_toad(&mut game.grid,  region_x-6,  region_y-3);

    add_beacon(&mut game.grid,  region_x+9,  region_y+15);
    add_beacon(&mut game.grid,  region_x-9,  region_y+15);
    add_beacon(&mut game.grid,  region_x+15,  region_y+9);
    add_beacon(&mut game.grid,  region_x-15,  region_y+9);
    add_beacon(&mut game.grid,  region_x+9,  region_y-15);
    add_beacon(&mut game.grid,  region_x-9,  region_y-15);
    add_beacon(&mut game.grid,  region_x+15,  region_y-9);
    add_beacon(&mut game.grid,  region_x-15,  region_y-9);

    add_glider(&mut game.grid,  region_x+6,  region_y+15);
    add_glider(&mut game.grid,  region_x-15,  region_y+6);
    add_glider(&mut game.grid,  region_x+15,  region_y-6);
    add_glider(&mut game.grid,  region_x-6,  region_y-15);

    add_lwss(&mut game.grid,  region_x-6,  region_y+15);
    add_lwss(&mut game.grid,  region_x+15,  region_y+6);
    add_lwss(&mut game.grid,  region_x+6,  region_y-15);
    add_lwss(&mut game.grid,  region_x-15,  region_y-6);


    while !window.window_should_close() {
        let mut d = window.begin_drawing(&rl);
        d.clear_background(Color::PINK);

        draw_grid(&game, &framebuffer, &mut d, scale);

        game.step();

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

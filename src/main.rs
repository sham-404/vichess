mod game;
mod piece;


use macroquad::prelude::*;
use crate::game::Game;

#[macroquad::main("Chess")]
async fn main() {
    println!("Hello, world!");
    let mut board = Game::new(8);
    board.setup_standard();

    loop {
        clear_background(WHITE);
        draw_text("HI bros", 20.0, 40.0, 20.0, BLACK);
        next_frame().await;
    }
}

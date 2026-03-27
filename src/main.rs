mod game;
mod piece;
mod gui;

use crate::game::Game;

#[macroquad::main("Chess")]
async fn main() {
    let mut board = Game::new(8);
    board.setup_standard();

    gui::run(board).await;
}

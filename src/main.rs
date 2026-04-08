mod board;
mod game;
mod gui;
mod piece;

use crate::{game::Game, gui::GUI};

#[macroquad::main("Chess")]
async fn main() {
    let mut game = Game::new(8);
    game.setup_standard();

    let mut ui = GUI::new(game);
    ui.run().await;
}

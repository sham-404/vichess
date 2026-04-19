mod board;
mod game;
mod gui;
mod piece;

use crate::{game::Game, gui::GUI};

#[macroquad::main("Vichess")]
async fn main() {
    run_app().await;
}

pub async fn run_app() {
    let mut game = Game::new(8);
    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    game.load_fen(fen);
    let mut ui = GUI::new(game);
    ui.run().await;
}

pub fn debug_app(fen: &str, depth: u32) {
    let mut game = Game::new(8);
    game.load_fen(fen);
    game.perft_divide(depth);
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/2KR3R b Kkq -";

    #[test]
    fn perft_1d() {
        debug_app(FEN, 1);
    }

    #[test]
    fn perft_2d() {
        debug_app(FEN, 2);
    }

    #[test]
    fn perft_3d() {
        debug_app(FEN, 3);
    }

    #[test]
    fn perft_4d() {
        debug_app(FEN, 4);
    }
}

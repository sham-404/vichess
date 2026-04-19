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
    print!("perft: {}: {:#?}", depth, game.perft_debug(depth));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FEN: &str = "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";

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

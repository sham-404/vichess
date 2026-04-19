mod board;
mod game;
mod gui;
mod piece;

use crate::{game::Game, gui::GUI};

// #[macroquad::main("Vichess")]
fn main() {
    debug_app();
}

pub async fn run_app() {
    let mut game = Game::new(8);
    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    game.load_fen(fen);
    let mut ui = GUI::new(game);
    ui.run().await;
}

pub fn debug_app() {
    let mut game = Game::new(8);
    let fen: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
    game.load_fen(fen);
    println!("Perfit 1: {:#?}", game.perft_debug(1));
    println!("Perfit 2: {:#?}", game.perft_debug(2));
    println!("Perfit 3: {:#?}", game.perft_debug(3));

}

#[cfg(test)]
mod tests {
    use super::*;

    const FEN: &str = "r3k2r/p1pp1pb1/bn2Qnp1/2qPN3/1p2P3/2N2N2/PPPB2PP/R3K2R w KQkq - 0 1";

    const EX_D1: u64 = 48;
    const EX_D2: u64 = 2039;
    const EX_D3: u64 = 97862;
    const EX_D4: u64 = 4085603;

    #[test]
    fn perft_1d() {
        let mut game = Game::new(8);
        game.load_fen(FEN);
        assert_eq!(game.perft(1), EX_D1);
    }

    #[test]
    fn perft_2d() {
        let mut game = Game::new(8);
        game.load_fen(FEN);
        assert_eq!(game.perft(2), EX_D2);
    }

    #[test]
    fn perft_3d() {
        let mut game = Game::new(8);
        game.load_fen(FEN);
        assert_eq!(game.perft(3), EX_D3);
    }

    #[test]
    fn perft_4d() {
        let mut game = Game::new(8);
        game.load_fen(FEN);
        assert_eq!(game.perft(4), EX_D4);
    }
}

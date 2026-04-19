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
    let game = Game::new(8);
    let mut ui = GUI::new(game);
    ui.run().await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_fen1() {
        let mut game = Game::new(8);
        game.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        game.board().print_cli_board();
    }

    #[test]
    fn load_fen2() {
        let mut game = Game::new(8);
        game.load_fen("rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPP2PPP/RNBQKBNR w KQkq d6 0 3");

        game.board().print_cli_board();
    }

    #[test]
    fn load_fen3() {
        let mut game = Game::new(8);
        game.load_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1");

        game.board().print_cli_board();
    }

    #[test]
    fn load_fen4() {
        let mut game = Game::new(8);
        game.load_fen("8/2p5/3p4/3Pp3/3P4/8/8/8 w - e6 0 1");

        game.board().print_cli_board();
    }
}

mod board;
mod piece;
use crate::board::Board;

fn main() {
    println!("Hello, world!");
    let board = Board::new(8);
    board.print_board();
}

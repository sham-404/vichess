mod board;
mod piece;
use crate::board::Board;

fn main() {
    println!("Hello, world!");
    let mut board = Board::new(8);
    board.setup_standard();
    board.print_board();
}

use crate::piece::{Piece, PieceKind};

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub board: Vec<Option<Piece>>,
}

impl Board {
    fn setup_board(size: usize) -> Vec<Option<Piece>> {
        let mut board = vec![None; size * size];

        for col in 0..size {
            let idx = 1 * size + col;
            board[idx] = Some(Piece::new(PieceKind::Pawn, (1, col)));
        }

        board
    }

    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            board: Self::setup_board(size),
        }
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }

    pub fn print_board(&self) {
        println!("--------------------------------");
        for row in 0..self.size {
            for col in 0..self.size {
                match &self.board[self.idx(row, col)] {
                    Some(piece) => print!("{:?} ", piece.kind.get_name()),
                    None => print!(" "),
                }
            }
            println!();
        }

        println!("--------------------------------");
    }
}

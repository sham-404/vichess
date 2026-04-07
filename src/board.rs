use crate::{
    game::Square,
    piece::{Piece},
};

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    squares: Vec<Square>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            squares: vec![Square::Empty; size * size],
        }
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get(&mut self, pos:usize) -> &mut Square {
        &mut self.squares[pos]
    }

    pub fn peek(&self, pos:usize) -> &Square {
        &self.squares[pos]
    }

    pub fn place(&mut self, square: Square, pos:usize) {
        self.squares[pos] = square;
    }

    pub fn place_piece(&mut self, idx: usize, piece: Piece) {
        self.squares[idx] = Square::Occupied(piece);
    }

    pub fn get_xy(&self, idx: usize) -> (f32, f32) {
        let row = idx / self.size;
        let col = idx % self.size;

        (col as f32, row as f32)
    }

    pub fn within_bounds(&self, pos: usize) -> bool {
        pos < self.size - 1
    }

    pub fn squares(&self) -> &[Square] {
        &self.squares
    }

    #[allow(dead_code)]
    pub fn print_cli_board(&self) {
        println!("---------------");
        for (idx, square) in self.squares().iter().enumerate() {
            match *square {
                Square::Occupied(piece) => print!("{} ", piece.name()),
                _ => print!("  "),
            }

            if idx % 8 == 7 {
                println!();
            }
        }

        println!("---------------");
    }

    pub fn idx(&self, row: i32, col: i32) -> usize {
        (row as usize) * self.get_size() + (col as usize)
    }
}

use crate::{
    game::Square,
    piece::{Piece, Pos},
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

    pub fn get(&mut self, pos: Pos) -> &mut Square {
        let idx = self.idx(pos);
        &mut self.squares[idx]
    }

    pub fn peek(&self, pos: Pos) -> &Square {
        &self.squares[self.idx(pos)]
    }

    pub fn place(&mut self, square: Square, pos: Pos) {
        let idx = self.idx(pos);
        self.squares[idx] = square;
    }

    pub fn place_piece(&mut self, idx: usize, piece: Piece) {
        self.squares[idx] = Square::Occupied(piece);
    }

    pub fn get_xy(&self, idx: usize) -> (f32, f32) {
        let row = idx / self.size;
        let col = idx % self.size;

        (col as f32, row as f32)
    }

    pub fn within_bounds(&self, pos: &Pos) -> bool {
        pos.row >= 0
            && pos.col >= 0
            && (pos.row as usize) < self.size
            && (pos.col as usize) < self.size
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

    fn idx(&self, pos: Pos) -> usize {
        (pos.row as usize) * self.get_size() + (pos.col as usize)
    }
}

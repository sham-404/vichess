use crate::piece::{Color, Piece, PieceKind, Pos};

#[derive(Debug, Clone)]
pub enum Square {
    _NotExists,
    Empty,
    Occupied(Piece),
}

#[derive(Debug, Clone)]
pub struct Board {
    size: usize,
    squares: Vec<Square>,
}

impl Board {
    pub fn get_xy(&self, idx: usize) -> (f32, f32) {
        let row = idx / self.size;
        let col = idx % self.size;

        (col as f32, row as f32)
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let board = Board {
            size,
            squares: vec![Square::Empty; size * size],
        };
        Self { board }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn squares(&self) -> &[Square] {
        &self.board.squares
    }

    pub fn get_size(&self) -> usize {
        self.board.size
    }

    pub fn setup_standard(&mut self) {
        // Pawns
        for col in 0..self.board.size {
            let col = col as i32;

            let white_pawn = Pos::new(1, col);
            let black_pawn = Pos::new((self.board.size - 2) as i32, col);

            self.set(
                white_pawn,
                Piece::new(PieceKind::Pawn, white_pawn, Color::White),
            );
            self.set(
                black_pawn,
                Piece::new(PieceKind::Pawn, black_pawn, Color::Black),
            );
        }

        // Back rank (order matters)
        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::King,
            PieceKind::Queen,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (col, kind) in back_rank.iter().enumerate() {
            let col = col as i32;

            let white_pos = Pos::new(0, col);
            let black_pos = Pos::new((self.board.size - 1) as i32, col);

            self.set(white_pos, Piece::new(*kind, white_pos, Color::White));
            self.set(black_pos, Piece::new(*kind, black_pos, Color::Black));
        }
    }

    fn set(&mut self, pos: Pos, piece: Piece) {
        let idx = self.idx(pos);
        self.board.squares[idx] = Square::Occupied(piece);
    }

    fn idx(&self, pos: Pos) -> usize {
        (pos.row as usize) * self.board.size + (pos.col as usize)
    }
}

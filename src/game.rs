use crate::piece::{Color, Piece, PieceKind};

#[derive(Debug, Clone)]
pub struct Game {
    pub size: usize,
    pub board: Vec<Option<Piece>>,
}

impl Game {
    pub fn setup_standard(&mut self) {
        let mut index: usize;

        // White pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(1, col);
            self.board[index] = Some(Piece::new(PieceKind::Pawn, index, Color::White));
        }

        // Rooks
        index = self.idx(0, 0);
        self.board[index] = Some(Piece::new(PieceKind::Rook, index, Color::White));
        index = self.idx(0, 7);
        self.board[index] = Some(Piece::new(PieceKind::Rook, index, Color::White));

        // Knights
        index = self.idx(0, 1);
        self.board[index] = Some(Piece::new(PieceKind::Knight, index, Color::White));
        index = self.idx(0, 6);
        self.board[index] = Some(Piece::new(PieceKind::Knight, index, Color::White));

        // Bishops
        index = self.idx(0, 2);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, index, Color::White));
        index = self.idx(0, 5);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, index, Color::White));

        // King
        index = self.idx(0, 3);
        self.board[index] = Some(Piece::new(PieceKind::King, index, Color::White));

        // Queen
        index = self.idx(0, 4);
        self.board[index] = Some(Piece::new(PieceKind::Queen, index, Color::White));

        // Black Pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(6, col);
            self.board[index] = Some(Piece::new(PieceKind::Pawn, index, Color::Black));
        }

        // Rooks
        index = self.idx(7, 0);
        self.board[index] = Some(Piece::new(PieceKind::Rook, index, Color::White));
        index = self.idx(7, 7);
        self.board[index] = Some(Piece::new(PieceKind::Rook, index, Color::White));

        // Knights
        index = self.idx(7, 1);
        self.board[index] = Some(Piece::new(PieceKind::Knight, index, Color::White));
        index = self.idx(7, 6);
        self.board[index] = Some(Piece::new(PieceKind::Knight, index, Color::White));

        // Bishops
        index = self.idx(7, 2);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, index, Color::White));
        index = self.idx(7, 5);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, index, Color::White));

        // King
        index = self.idx(7, 3);
        self.board[index] = Some(Piece::new(PieceKind::King, index, Color::White));

        // Queen
        index = self.idx(7, 4);
        self.board[index] = Some(Piece::new(PieceKind::Queen, index, Color::White));
    }

    pub fn new(size: usize) -> Self {
        Self {
            size: size,
            board: vec![None; size * size],
        }
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }
}

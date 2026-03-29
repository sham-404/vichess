use crate::piece::{Color, Piece, PieceKind};

#[derive(Debug, Clone)]
pub enum Square {
    _NotExists,
    Empty,
    Occupied(Piece),
}

#[derive(Debug, Clone)]
pub struct Game {
    size: usize,
    board: Vec<Square>,
    selected_piece: Option<Piece>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            selected_piece: None,
            size: size,
            board: vec![Square::Empty; size * size],
        }
    }

    pub fn board(&self) -> &Vec<Square> {
        &self.board
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn setup_standard(&mut self) {
        let mut index: usize;
        let mut piece: Piece;

        // White pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(1, col);
            piece = Piece::new(PieceKind::Pawn, index, Color::White);
            self.board[index] = Square::Occupied(piece);
        }

        // Rooks
        index = self.idx(0, 0);
        piece = Piece::new(PieceKind::Rook, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(0, 7);
        piece = Piece::new(PieceKind::Rook, index, Color::White);
        self.board[index] = Square::Occupied(piece);


        // Knights
        index = self.idx(0, 1);
        piece = Piece::new(PieceKind::Knight, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(0, 6);
        piece = Piece::new(PieceKind::Knight, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Bishops
        index = self.idx(0, 2);
        piece = Piece::new(PieceKind::Bishop, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(0, 5);
        piece = Piece::new(PieceKind::Bishop, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // King
        index = self.idx(0, 3);
        piece = Piece::new(PieceKind::King, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Queen
        index = self.idx(0, 4);
        piece = Piece::new(PieceKind::Queen, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Black Pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(6, col);
            piece = Piece::new(PieceKind::Pawn, index, Color::Black);
        self.board[index] = Square::Occupied(piece);
        }

        // Rooks
        index = self.idx(7, 0);
        piece = Piece::new(PieceKind::Rook, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(7, 7);
        piece = Piece::new(PieceKind::Rook, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Knights
        index = self.idx(7, 1);
        piece = Piece::new(PieceKind::Knight, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(7, 6);
        piece = Piece::new(PieceKind::Knight, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Bishops
        index = self.idx(7, 2);
        piece = Piece::new(PieceKind::Bishop, index, Color::White);
        self.board[index] = Square::Occupied(piece);
        index = self.idx(7, 5);
        piece = Piece::new(PieceKind::Bishop, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // King
        index = self.idx(7, 3);
        piece = Piece::new(PieceKind::King, index, Color::White);
        self.board[index] = Square::Occupied(piece);

        // Queen
        index = self.idx(7, 4);
        piece = Piece::new(PieceKind::Queen, index, Color::White);
        self.board[index] = Square::Occupied(piece);
    }


    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.size + col
    }
}

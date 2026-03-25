use crate::piece::{Color, Piece, PieceKind};

#[derive(Debug, Clone)]
pub struct Board {
    pub size: usize,
    pub board: Vec<Option<Piece>>,
}

impl Board {
    pub fn setup_standard(&mut self) {
        let mut index: usize;

        // White pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(1, col);
            self.board[index] = Some(Piece::new(PieceKind::Pawn, (1, col), Color::White));
        }

        // Rooks
        index = self.idx(0, 0);
        self.board[index] = Some(Piece::new(PieceKind::Rook, (0, 0), Color::White));
        index = self.idx(0, 7);
        self.board[index] = Some(Piece::new(PieceKind::Rook, (0, 7), Color::White));

        // Knights
        index = self.idx(0, 1);
        self.board[index] = Some(Piece::new(PieceKind::Knight, (0, 1), Color::White));
        index = self.idx(0, 6);
        self.board[index] = Some(Piece::new(PieceKind::Knight, (0, 6), Color::White));

        // Bishops
        index = self.idx(0, 2);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, (0, 2), Color::White));
        index = self.idx(0, 5);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, (0, 5), Color::White));

        // King
        index = self.idx(0, 3);
        self.board[index] = Some(Piece::new(PieceKind::King, (0, 3), Color::White));

        // Queen
        index = self.idx(0, 4);
        self.board[index] = Some(Piece::new(PieceKind::Queen, (0, 4), Color::White));

        // Black Pieces
        // Pawns
        for col in 0..self.size {
            index = self.idx(6, col);
            self.board[index] = Some(Piece::new(PieceKind::Pawn, (6, col), Color::Black));
        }

        // Rooks
        index = self.idx(7, 0);
        self.board[index] = Some(Piece::new(PieceKind::Rook, (7, 0), Color::White));
        index = self.idx(7, 7);
        self.board[index] = Some(Piece::new(PieceKind::Rook, (7, 7), Color::White));

        // Knights
        index = self.idx(7, 1);
        self.board[index] = Some(Piece::new(PieceKind::Knight, (7, 1), Color::White));
        index = self.idx(7, 6);
        self.board[index] = Some(Piece::new(PieceKind::Knight, (7, 6), Color::White));

        // Bishops
        index = self.idx(7, 2);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, (7, 2), Color::White));
        index = self.idx(7, 5);
        self.board[index] = Some(Piece::new(PieceKind::Bishop, (7, 5), Color::White));

        // King
        index = self.idx(7, 3);
        self.board[index] = Some(Piece::new(PieceKind::King, (7, 4), Color::White));

        // Queen
        index = self.idx(7, 4);
        self.board[index] = Some(Piece::new(PieceKind::Queen, (7, 3), Color::White));
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

    pub fn print_board(&self) {
        println!("---------------");
        for row in 0..self.size {
            for col in 0..self.size {
                match &self.board[self.idx(row, col)] {
                    Some(piece) => print!("{} ", piece.kind.get_name()),
                    None => print!(" "),
                }
            }
            println!();
        }

        println!("---------------");
    }
}

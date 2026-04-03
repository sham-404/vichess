use crate::board::Board;
use crate::piece::{MyColor, Piece, PieceKind, Pos};

#[derive(Debug, Clone)]
pub enum Square {
    _NotExists,
    Empty,
    Occupied(Piece),
}

#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(size: usize) -> Self {
        Self {
            board: Board::new(size),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn squares(&self) -> &[Square] {
        &self.board.squares()
    }

    pub fn get_size(&self) -> usize {
        self.board.get_size()
    }

    pub fn setup_standard(&mut self) {
        // Pawns
        for col in 0..self.board.get_size() {
            let col = col as i32;

            let white_pawn = Pos::new(1, col);
            let black_pawn = Pos::new((self.board.get_size() - 2) as i32, col);

            self.set(
                white_pawn,
                Piece::new(PieceKind::Pawn, white_pawn, MyColor::White),
            );
            self.set(
                black_pawn,
                Piece::new(PieceKind::Pawn, black_pawn, MyColor::Black),
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
            let black_pos = Pos::new((self.board.get_size() - 1) as i32, col);

            self.set(white_pos, Piece::new(*kind, white_pos, MyColor::White));
            self.set(black_pos, Piece::new(*kind, black_pos, MyColor::Black));
        }
    }

    fn generate_moves(&self) {
        for square in self.board.squares() {
            match square {
                Square::Occupied(piece) => _ = piece.get_piece_moves(&self.board),
                _ => continue,
            }
        }
    }

    fn set(&mut self, pos: Pos, piece: Piece) {
        let idx = self.idx(pos);
        self.board.place_piece(idx, piece);
    }

    fn idx(&self, pos: Pos) -> usize {
        (pos.row as usize) * self.board.get_size() + (pos.col as usize)
    }
}

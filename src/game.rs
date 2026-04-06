use crate::board::Board;
use crate::piece::{MyColor, Piece, PieceKind, Pos};

#[derive(Debug, Clone, Copy)]
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
        let game = Self {
            board: Board::new(size),
        };
        game.generate_moves();
        game
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

    pub fn get_piece_moves(&self, piece: &Piece) -> Vec<Pos> {
        let moves = match piece.kind {
            PieceKind::Pawn => self.pawn_moves(piece),
            PieceKind::King => self.king_moves(piece),
            PieceKind::Queen => self.queen_moves(piece),
            PieceKind::Bishop => self.bishop_moves(piece),
            PieceKind::Rook => self.rook_moves(piece),
            PieceKind::Knight => self.knight_moves(piece),
        };
        moves
    }

    fn sliding_moves(&self, piece: &Piece, dir: &[(i32, i32)]) -> Vec<Pos> {
        let board = &self.board;
        let mut moves: Vec<Pos> = Vec::new();
        for &(dr, dc) in dir {
            for i in 1..board.get_size() as i32 {
                let new_pos = piece.pos.offset(dr * i, dc * i);

                if !board.within_bounds(&new_pos) {
                    break;
                }

                let square = board.peek(new_pos);
                match square {
                    Square::Empty => moves.push(new_pos),
                    Square::Occupied(p) => {
                        if piece.color != p.color {
                            moves.push(new_pos);
                        }
                        break;
                    }
                    Square::_NotExists => {}
                }
            }
        }
        moves
    }

    fn get_pawn_dir(pawn: &Piece) -> i32 {
        if pawn.color() == MyColor::White {
            return 1;
        }
        -1
    }

    fn has_pawn_moved(pawn: &Piece) -> bool {
        let mut moved = true;
        match pawn.color() {
            MyColor::White => {
                if pawn.pos.row == 1 {
                    moved = false
                }
            }

            MyColor::Black => {
                if pawn.pos.row == 6 {
                    moved = false
                }
            }
        }
        moved
    }

    fn pawn_moves(&self, piece: &Piece) -> Vec<Pos> {
        let mut moves: Vec<Pos> = Vec::new();
        let dir = Self::get_pawn_dir(piece);
        let new_pos = piece.pos.offset(dir, 0);

        if self.board.within_bounds(&new_pos) {
            if matches!(self.board.peek(new_pos), Square::Empty) {
                moves.push(new_pos);
            } else {
                return moves;
            }
        } else {
            return moves;
        }

        if Self::has_pawn_moved(piece) {
            return moves;
        }

        // Second step if pawn hasn't moved yet
        let new_pos = piece.pos.offset(dir * 2, 0);

        if self.board.within_bounds(&new_pos) {
            if matches!(self.board.peek(new_pos), Square::Empty) {
                moves.push(new_pos);
            }
        }

        moves
    }

    fn king_moves(&self, piece: &Piece) -> Vec<Pos> {
        let mut moves: Vec<Pos> = Vec::new();

        let offset = [
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (0, 1),
        ];

        for (dr, dc) in offset {
            let new_pos = piece.pos.offset(dr, dc);

            if !self.board.within_bounds(&new_pos) {
                continue;
            }

            let square = self.board.peek(new_pos);
            match square {
                Square::_NotExists => continue,
                Square::Empty => moves.push(new_pos),
                Square::Occupied(p) => {
                    if p.color != piece.color {
                        moves.push(new_pos);
                    }
                }
            }
        }

        moves
    }

    fn queen_moves(&self, piece: &Piece) -> Vec<Pos> {
        let dir = [
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
            (1, 0),
            (-1, 0),
            (0, -1),
            (0, 1),
        ];

        let moves = self.sliding_moves(piece, &dir);
        moves
    }

    fn bishop_moves(&self, piece: &Piece) -> Vec<Pos> {
        let dir = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

        let moves = self.sliding_moves(piece, &dir);
        moves
    }

    fn rook_moves(&self, piece: &Piece) -> Vec<Pos> {
        let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let moves = self.sliding_moves(piece, &dir);
        moves
    }

    fn knight_moves(&self, piece: &Piece) -> Vec<Pos> {
        let mut moves: Vec<Pos> = Vec::new();

        let offset = [
            (2, 1),
            (2, -1),
            (-2, 1),
            (-2, -1),
            (1, 2),
            (-1, 2),
            (1, -2),
            (-1, -2),
        ];

        for (dr, dc) in offset {
            let new_pos = piece.pos.offset(dr, dc);

            if !self.board.within_bounds(&new_pos) {
                continue;
            }

            let square = self.board.peek(new_pos);
            match square {
                Square::_NotExists => continue,
                Square::Empty => moves.push(new_pos),
                Square::Occupied(p) => {
                    if p.color != piece.color {
                        moves.push(new_pos);
                    }
                }
            }
        }

        moves
    }

    pub fn make_move(&mut self, from: Pos, to: Pos) -> bool {
        let square = self.board.peek(from);
        if let Square::Occupied(piece) = square {
            if !self.get_piece_moves(piece).contains(&to) {
                return false;
            }
        } else {
            return false;
        } // filters if it is not a valid move

        let square = self.board.get(from);
        if let Square::Occupied(piece) = square {
            piece.change_pos(to);
        }

        let square = std::mem::replace(square, Square::Empty);
        self.board.place(square, to);

        // Post move activities
        self.generate_moves();

        // Debugging area
        // self.board().print_cli_board();

        true
    }

    fn generate_moves(&self) {
        for square in self.board.squares() {
            match square {
                Square::Occupied(piece) => _ = self.get_piece_moves(piece),
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

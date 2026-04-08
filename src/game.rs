use crate::board::Board;
use crate::piece::{Move, Piece, PieceColor};

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
        // game.generate_moves();
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

            let white_pawn = self.board.idx(1, col);
            let black_pawn = self.board().idx((self.board().get_size() - 2) as i32, col);

            self.set(white_pawn, Piece::Pawn(PieceColor::White));
            self.set(black_pawn, Piece::Pawn(PieceColor::Black));
        }

        // Back rank (order matters)
        let back_rank = [
            Piece::Rook,
            Piece::Knight,
            Piece::Bishop,
            Piece::King,
            Piece::Queen,
            Piece::Bishop,
            Piece::Knight,
            Piece::Rook,
        ];

        for (col, kind) in back_rank.iter().enumerate() {
            let col = col as i32;

            let white_pos = self.board.idx(0, col);
            let black_pos = self.board().idx((self.board().get_size() - 1) as i32, col);

            self.set(white_pos, kind(PieceColor::White));
            self.set(black_pos, kind(PieceColor::Black));
        }
    }

    pub fn is_valid_step(from: i32, to: i32, dir: i32) -> bool {
        if to < 0 || to >= 64 {
            return false;
        }

        let f1 = from % 8;
        let f2 = to % 8;

        match dir {
            1 => f2 == f1 + 1,
            -1 => f2 == f1 - 1,

            9 => f2 == f1 + 1,
            -7 => f2 == f1 + 1,

            7 => f2 == f1 - 1,
            -9 => f2 == f1 - 1,

            8 | -8 => true,

            _ => true,
        }
    }

    fn gen_dir_moves(&self, piece: &Piece, pos: usize) -> Vec<Move> {
        let dir = piece.get_dir();
        let board = &self.board;
        let mut moves: Vec<Move> = Vec::new();
        for &di in dir {
            let mut cur_pos = pos;
            for _ in 1..board.get_size() as i32 {
                let new_pos = cur_pos as i32 + di;

                if !Self::is_valid_step(cur_pos as i32, new_pos, di) {
                    break;
                }

                let square = board.peek(new_pos as usize);
                match square {
                    Square::Empty => moves.push(Move::new(pos, new_pos as usize)),
                    Square::Occupied(p) => {
                        if piece.color() != p.color() {
                            moves.push(Move::new(pos, new_pos as usize));
                        }
                        break;
                    }
                    Square::_NotExists => {}
                }

                // Breaking for king and knight as they go only once per direction
                if matches!(piece, Piece::King(_) | Piece::Knight(_)) {
                    break;
                }
                cur_pos = new_pos as usize;
            }
        }
        moves
    }

    pub fn get_moves(&self, piece: &Piece, pos: usize) -> Vec<Move> {
        match piece {
            Piece::Pawn(_) => self.gen_pawn_moves(piece, pos),
            _ => self.gen_dir_moves(piece, pos),
        }
    }

    fn gen_pawn_moves(&self, piece: &Piece, pos: usize) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pos = pos as i32;
        let file = pos % 8;
        let rank = pos / 8;

        let (forward, start_rank, captures) = match piece.color() {
            PieceColor::White => (8, 1, [7, 9]),
            PieceColor::Black => (-8, 6, [-7, -9]),
        };

        // 1. Single forward movement
        let one = pos + forward;
        if one >= 0 && one < 64 {
            if matches!(self.board.peek(one as usize), Square::Empty) {
                moves.push(Move::new(pos as usize, one as usize));

                // 2. Double forward (only if it is the first move)
                if rank == start_rank {
                    let two = pos + 2 * forward;
                    if two >= 0
                        && two < 64
                        && matches!(self.board.peek(two as usize), Square::Empty)
                    {
                        moves.push(Move::new(pos as usize, two as usize));
                    }
                }
            }
        }

        // 3. Captures
        for &cap in &captures {
            let target = pos + cap;

            if target < 0 || target >= 64 {
                continue;
            }

            let target_file = target % 8;

            // ensure diagonal (no wrapping)
            if (target_file - file).abs() != 1 {
                continue;
            }

            // check if the diagonal piece is the opponent
            let square = self.board.peek(target as usize);
            match square {
                Square::Occupied(p) => {
                    if p.color() != piece.color() {
                        moves.push(Move::new(pos as usize, target as usize));
                    }
                }
                _ => continue,
            }
        }

        moves
    }

    pub fn make_move(&mut self, mov: Move) -> bool {
        let square = self.board.peek(mov.from);
        if let Square::Occupied(piece) = square {
            if !self.get_moves(piece, mov.from).contains(&mov) {
                return false;
            }
        } else {
            return false;
        } // filters if it is not a valid move

        let square = self.board.get(mov.from);

        let square = std::mem::replace(square, Square::Empty);
        self.board.place(square, mov.to);

        // Post move activities
        // self.generate_moves();

        // Debugging area
        // self.board().print_cli_board();

        true
    }

    fn set(&mut self, idx: usize, piece: Piece) {
        self.board.place_piece(idx, piece);
    }

    pub fn row(&self, idx: usize) -> i32 {
        (idx / self.board.get_size()) as i32
    }

    pub fn col(&self, idx: usize) -> i32 {
        (idx % self.board.get_size()) as i32
    }
}

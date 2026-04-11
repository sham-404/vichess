use crate::board::Board;
use crate::piece::{Move, MoveKind, Piece, PieceColor};

#[derive(Debug, Clone, Copy)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    history: Vec<Move>,
    players: Vec<Player>,
    cur_player: Player,
    legal_moves: Vec<Move>,
    attack_map: [bool; 64],
    // redo_list: Vec<Move>,
}

impl Game {
    pub fn new(size: usize) -> Self {
        let players = vec![
            Player::new(PieceColor::White),
            Player::new(PieceColor::Black),
        ];
        let cur_player = players[0];

        let game = Self {
            board: Board::new(size),
            history: Vec::new(),
            players,
            cur_player, // redo_list: Vec::new(),
            legal_moves: Vec::new(),
            attack_map: [false; 64],
        };

        game
    }

    pub fn get_attack_map(&self) -> &[bool] {
        &self.attack_map
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

    pub fn get_last_move(&self) -> Option<&Move> {
        self.history.last()
    }

    fn change_turn(&mut self, reverse: bool) {
        let fac: isize = if reverse { -1 } else { 1 };

        let len = self.players.len() as isize;

        let idx = self
            .players
            .iter()
            .position(|&p| p == self.cur_player)
            .expect("Current player not in players") as isize;

        let new_idx = (idx + fac).rem_euclid(len) as usize;

        self.cur_player = self.players[new_idx];
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

        // generate moves after placing the pieces
        self.generate_moves();
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
                    Square::Occupied(opp_piece) => {
                        if piece.color() != opp_piece.color() {
                            moves.push(Move::new(pos, new_pos as usize).with_capture(*opp_piece));
                        }
                        break;
                    }
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

    fn gen_pawn_moves(&self, pawn: &Piece, pos: usize) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();

        let pos = pos as i32;
        let file = pos % 8;
        let rank = pos / 8;

        let (forward, start_rank, captures) = match pawn.color() {
            PieceColor::White => (8, 1, [7, 9]),
            PieceColor::Black => (-8, 6, [-7, -9]),
        };

        // 1. Single forward movement
        let one = pos + forward;
        if one >= 0 && one < 64 {
            if matches!(self.board.peek(one as usize), Square::Empty) {
                let mut cur_move = Move::new(pos as usize, one as usize);

                // Checking if it is the promotion square
                match pawn.color() {
                    PieceColor::White => {
                        if self.row(one as usize) == 7 {
                            cur_move.promote_to(Piece::Queen(PieceColor::White));
                        }
                    }
                    PieceColor::Black => {
                        if self.row(one as usize) == 0 {
                            cur_move.promote_to(Piece::Queen(PieceColor::Black));
                        }
                    }
                }
                moves.push(cur_move);

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
                Square::Occupied(opp_piece) => {
                    if opp_piece.color() != pawn.color() {
                        let mut cur_move =
                            Move::new(pos as usize, target as usize).with_capture(*opp_piece);

                        // Checking if it is the promotion square
                        match pawn.color() {
                            PieceColor::White => {
                                if self.row(one as usize) == 7 {
                                    cur_move.promote_to(Piece::Queen(PieceColor::White));
                                }
                            }
                            PieceColor::Black => {
                                if self.row(one as usize) == 0 {
                                    cur_move.promote_to(Piece::Queen(PieceColor::Black));
                                }
                            }
                        }
                        moves.push(cur_move);
                    }
                }
                _ => continue,
            }
        }

        moves
    }

    fn get_moves(&self, piece: &Piece, pos: usize) -> Vec<Move> {
        match piece {
            Piece::Pawn(_) => self.gen_pawn_moves(piece, pos),
            _ => self.gen_dir_moves(piece, pos),
        }
    }

    pub fn moves(&self, pos: usize) -> Vec<&Move> {
        let moves = self.legal_moves.iter().filter(|&m| m.from == pos).collect();
        moves
    }

    fn gen_dir_attacks(&self, piece: &Piece, pos: usize) -> Vec<usize> {
        let dir = piece.get_dir();
        let board = &self.board;
        let mut attacks: Vec<usize> = Vec::new();
        for &di in dir {
            let mut cur_pos = pos;
            for _ in 1..board.get_size() as i32 {
                let new_pos = cur_pos as i32 + di;

                if !Self::is_valid_step(cur_pos as i32, new_pos, di) {
                    break;
                }

                let square = board.peek(new_pos as usize);
                attacks.push(new_pos as usize);

                if let Square::Occupied(_) = *square {
                    break;
                }

                // Breaking for king and knight as they go only once per direction
                if matches!(piece, Piece::King(_) | Piece::Knight(_)) {
                    break;
                }
                cur_pos = new_pos as usize;
            }
        }
        attacks
    }

    fn gen_pawn_attacks(&self, pawn: &Piece, pos: usize) -> Vec<usize> {
        let mut attacks: Vec<usize> = Vec::new();

        let pos = pos as i32;
        let file = pos % 8;

        let captures = match pawn.color() {
            PieceColor::White => [7, 9],
            PieceColor::Black => [-7, -9],
        };

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

            attacks.push(target as usize);
        }

        attacks
    }

    pub fn get_attacks(&self, piece: &Piece, pos: usize) -> Vec<usize> {
        match piece {
            Piece::Pawn(_) => self.gen_pawn_attacks(piece, pos),
            _ => self.gen_dir_attacks(piece, pos),
        }
    }

    fn generate_moves(&mut self) {
        self.legal_moves.clear();
        self.attack_map = [false; 64];

        for (idx, square) in self.board.squares().iter().enumerate() {
            match square {
                Square::Occupied(piece) => {
                    if piece.color() != &self.cur_player.color {
                        //generate attack map if it is the opponent piece

                        let attacks = self.get_attacks(piece, idx);
                        attacks.iter().for_each(|&idx| self.attack_map[idx] = true);

                        continue;
                    }

                    let mut moves = self.get_moves(piece, idx);
                    self.legal_moves.append(&mut moves);
                }
                Square::Empty => continue,
            }
        }
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> bool {
        let square = self.board.peek(from);

        let piece = match square {
            Square::Occupied(p) => p,
            _ => return false,
        };

        // Return if it is not the current players move
        if piece.color() != &self.cur_player.color {
            return false;
        }

        let moves = self.get_moves(piece, from);
        let mov = match moves.iter().find(|mov| mov.to == to) {
            Some(&m) => m,
            None => return false,
        }; // filters if it is not a valid move

        let square = self.board.get(mov.from);

        let square = std::mem::replace(square, Square::Empty);
        self.board.place(square, to);

        // taking care of special moves
        match mov.kind {
            MoveKind::Promotion(p) => {
                self.board.place_piece(to, p);
            }
            _ => {}
        };

        // Post move activities

        self.history.push(mov);
        self.change_turn(false);
        self.generate_moves();

        // Debugging area
        // self.board().print_cli_board();

        true
    }

    pub fn undo_move(&mut self) {
        if self.history.is_empty() {
            return;
        }

        let mov = self.history.pop().unwrap();

        let square = self.board.get(mov.to);
        let square = std::mem::replace(square, Square::Empty);
        self.board.place(square, mov.from);

        // Returning the captured piece
        if let Some(piece) = mov.capture {
            self.board.place_piece(mov.to, piece);
        }

        // taking care of special moves
        match mov.kind {
            MoveKind::Normal => {}
            MoveKind::Promotion(p) => {
                self.board.place_piece(mov.from, Piece::Pawn(*p.color()));
            }
            _ => {}
        }
        self.change_turn(true);
        self.generate_moves();
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Player {
    color: PieceColor,
}

impl Player {
    pub fn new(color: PieceColor) -> Self {
        Self { color }
    }
}

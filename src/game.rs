use crate::board::Board;
use crate::piece::{Move, MoveKind, Piece, PieceColor};

pub const WK: u8 = 0b0001;
pub const WQ: u8 = 0b0010;
pub const BK: u8 = 0b0100;
pub const BQ: u8 = 0b1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

#[derive(Debug)]
pub struct Game {
    board: Board,
    history: Vec<Move>,
    players: Vec<Player>,
    king_pos: KingPos,
    cur_player: Player,
    legal_moves: Vec<Move>,
    attack_map: [bool; 64],
    castling: CastlingRights,
    en_passant_sq: Option<usize>,
    // redo_list: Vec<Move>,
}

const WK_START_POS: usize = 60;
const BK_START_POS: usize = 4;

impl Game {
    pub fn new(size: usize) -> Self {
        let players = vec![
            Player::new(PieceColor::White),
            Player::new(PieceColor::Black),
        ];
        let cur_player = players[0];

        let mut game = Self {
            board: Board::new(size),
            history: Vec::new(),
            players,
            king_pos: KingPos {
                white: 200,
                black: 200,
            },
            castling: CastlingRights::new(),
            cur_player, // redo_list: Vec::new(),
            legal_moves: Vec::new(),
            attack_map: [false; 64],
            en_passant_sq: None,
        };
        game.setup_board();

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

    pub fn load_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        //  Clear board
        self.board = Board::new(self.get_size());

        // 2. Piece placement
        let mut idx = 0;

        for ch in parts[0].chars() {
            println!("{}", ch);
            match ch {
                '/' => continue,

                '1'..='8' => {
                    let empty = ch.to_digit(10).unwrap();
                    idx += empty as usize;
                }

                _ => {
                    let color = if ch.is_uppercase() {
                        PieceColor::White
                    } else {
                        PieceColor::Black
                    };

                    let piece = match ch.to_ascii_lowercase() {
                        'k' => {
                            match color {
                                PieceColor::White => self.king_pos.white = idx,
                                PieceColor::Black => self.king_pos.black = idx,
                            }
                            Piece::King(color)
                        }
                        'q' => Piece::Queen(color),
                        'r' => {
                            println!("hi");
                            Piece::Rook(color)
                        }
                        'b' => Piece::Bishop(color),
                        'n' => Piece::Knight(color),
                        'p' => Piece::Pawn(color),
                        _ => panic!("Invalid FEN character"),
                    };

                    self.set(idx, piece);
                    idx += 1;
                }
            }
        }

        // 3. Side to move
        let color = match parts[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => panic!("Invalid side to move"),
        };

        self.cur_player = match self.players.iter().find(|&p| p.color == color) {
            Some(p) => *p,
            None => unreachable!("if reacheched, check load_fen"),
        };

        // 4. Castling rights
        self.castling.remove(WK | WQ | BK | BQ); // however you store it

        if parts[2] != "-" {
            for ch in parts[2].chars() {
                match ch {
                    'K' => self.castling.add(WK),
                    'Q' => self.castling.add(WQ),
                    'k' => self.castling.add(BK),
                    'q' => self.castling.add(BQ),
                    _ => {}
                }
            }
        }

        // 5. En passant
        self.en_passant_sq = if parts[3] == "-" {
            None
        } else {
            Some(Self::algebraic_to_index(parts[3]))
        };

        // ignoring halfmove + fullmove for now
        self.generate_moves();
    }

    fn algebraic_to_index(sq: &str) -> usize {
        let bytes = sq.as_bytes();

        let file = (bytes[0] - b'a') as usize;
        let rank = (bytes[1] - b'1') as usize;

        // convert to your indexing (0 = a8)
        (7 - rank) * 8 + file
    }

    pub fn setup_board(&mut self) {
        self.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    fn is_valid_step(from: i32, to: i32, dir: i32) -> bool {
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

            17 | 10 | -6 | -15 => f2 > f1,
            15 | 6 | -10 | -17 => f2 < f1,

            _ => false,
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
                    Square::Empty => moves.push(Move::new(
                        pos,
                        new_pos as usize,
                        self.castling,
                        self.en_passant_sq,
                    )),
                    Square::Occupied(opp_piece) => {
                        if piece.color() != opp_piece.color() {
                            moves.push(
                                Move::new(pos, new_pos as usize, self.castling, self.en_passant_sq)
                                    .with_capture(*opp_piece),
                            );
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
            PieceColor::Black => (8, 1, [7, 9]),
            PieceColor::White => (-8, 6, [-7, -9]),
        };

        // 1. Single forward movement
        let one = pos + forward;
        if one >= 0 && one < 64 {
            if matches!(self.board.peek(one as usize), Square::Empty) {
                let mut cur_move = Move::new(
                    pos as usize,
                    one as usize,
                    self.castling,
                    self.en_passant_sq,
                );

                // Checking if it is the promotion square
                match pawn.color() {
                    PieceColor::White => {
                        if self.row(one as usize) == 0 {
                            cur_move.promote_to(Piece::Queen(PieceColor::White));
                        }
                    }
                    PieceColor::Black => {
                        if self.row(one as usize) == 7 {
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
                        moves.push(Move::new(
                            pos as usize,
                            two as usize,
                            self.castling,
                            self.en_passant_sq,
                        ));
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
                        let mut cur_move = Move::new(
                            pos as usize,
                            target as usize,
                            self.castling,
                            self.en_passant_sq,
                        )
                        .with_capture(*opp_piece);

                        // Checking if it is the promotion square
                        match pawn.color() {
                            PieceColor::White => {
                                if self.row(one as usize) == 0 {
                                    cur_move.promote_to(Piece::Queen(PieceColor::White));
                                }
                            }
                            PieceColor::Black => {
                                if self.row(one as usize) == 7 {
                                    cur_move.promote_to(Piece::Queen(PieceColor::Black));
                                }
                            }
                        }
                        moves.push(cur_move);
                    }
                }

                // if target square is empty, check for en passant possibility
                Square::Empty => {
                    if let Some(sq) = self.en_passant_sq {
                        if sq as i32 == target {
                            moves.push(
                                Move::new(pos as usize, sq, self.castling, self.en_passant_sq)
                                    .with_enpassant(),
                            );
                        }
                    }
                }
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

    pub fn moves(&self, pos: usize) -> Vec<Move> {
        let moves = self
            .legal_moves
            .iter()
            .filter(|&m| m.from == pos)
            .cloned()
            .collect();
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
            PieceColor::Black => [7, 9],
            PieceColor::White => [-7, -9],
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

    fn get_attacks(&self, piece: &Piece, pos: usize) -> Vec<usize> {
        match piece {
            Piece::Pawn(_) => self.gen_pawn_attacks(piece, pos),
            _ => self.gen_dir_attacks(piece, pos),
        }
    }

    fn is_square_attacked(&self, pos: usize, cur: PieceColor) -> bool {
        let board = &self.board;

        // Sliding pieces
        let directions = [
            ([9, 7, -7, -9], true),  // diagonals
            ([8, -8, 1, -1], false), // straight
        ];

        for (dirs, is_diag) in directions {
            for di in dirs {
                let mut cur_pos = pos;

                loop {
                    let new_pos = cur_pos as i32 + di;

                    if !Self::is_valid_step(cur_pos as i32, new_pos, di) {
                        break;
                    }

                    match board.peek(new_pos as usize) {
                        Square::Empty => {
                            cur_pos = new_pos as usize;
                            continue;
                        }
                        Square::Occupied(p) => {
                            if *p.color() != cur {
                                match p {
                                    Piece::Queen(_) => return true,
                                    Piece::Bishop(_) if is_diag => return true,
                                    Piece::Rook(_) if !is_diag => return true,
                                    _ => {}
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        // Knights
        let knight_dirs = [17, 15, 10, 6, -6, -10, -15, -17];
        for di in knight_dirs {
            let new_pos = pos as i32 + di;

            if !Self::is_valid_step(pos as i32, new_pos, di) {
                continue;
            }

            if let Square::Occupied(Piece::Knight(c)) = board.peek(new_pos as usize) {
                if *c != cur {
                    return true;
                }
            }
        }

        // King
        let king_dirs = [8, -8, 1, -1, 9, 7, -7, -9];
        for di in king_dirs {
            let new_pos = pos as i32 + di;

            if !Self::is_valid_step(pos as i32, new_pos, di) {
                continue;
            }

            if let Square::Occupied(Piece::King(c)) = board.peek(new_pos as usize) {
                if *c != cur {
                    return true;
                }
            }
        }

        // Pawns
        let pawn_dirs = match cur {
            PieceColor::Black => [7, 9],
            PieceColor::White => [-7, -9],
        };

        // if cur is white, check the bottom diags, cuz that is where that an opp
        // pawn can attack from and vise versa

        for di in pawn_dirs {
            let new_pos = pos as i32 + di;

            if !Self::is_valid_step(pos as i32, new_pos, di) {
                continue;
            }

            if let Square::Occupied(Piece::Pawn(c)) = board.peek(new_pos as usize) {
                if *c != cur {
                    return true;
                }
            }
        }

        false
    }

    pub fn in_check(&self, color: PieceColor) -> bool {
        let king_pos = match color {
            PieceColor::White => self.king_pos.white,
            PieceColor::Black => self.king_pos.black,
        };

        if self.is_square_attacked(king_pos, color) {
            return true;
        }

        false
    }

    fn generate_moves(&mut self) -> Vec<Move> {
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

        self.gen_castling_moves();
        self.filter_illegal();

        return self.legal_moves.clone();
    }

    fn can_castle_kingside(&self, k_pos: usize, color: PieceColor) -> bool {
        let start_pos = match color {
            PieceColor::White => WK_START_POS,
            PieceColor::Black => BK_START_POS,
        };

        // return true if king is in start pos
        // king is not in attack
        // adjacent squares are Empty
        // rook is in place
        // adjacent Empty squares are not in attack

        start_pos == k_pos
            && !self.attack_map[k_pos]
            && self.board.peek(k_pos + 1) == &Square::Empty
            && self.board.peek(k_pos + 2) == &Square::Empty
            && self.board.peek(k_pos + 3) == &Square::Occupied(Piece::Rook(color))
            && !self.attack_map[k_pos + 1]
            && !self.attack_map[k_pos + 2]
    }

    fn can_castle_queenside(&self, k_pos: usize, color: PieceColor) -> bool {
        let start_pos = match color {
            PieceColor::White => WK_START_POS,
            PieceColor::Black => BK_START_POS,
        };

        // return true if king is in start pos
        // king is not in attack
        // adjacent squares are Empty
        // rook is in place
        // adjacent Empty squares are not in attack

        start_pos == k_pos
            && !self.attack_map[k_pos]
            && self.board.peek(k_pos - 1) == &Square::Empty
            && self.board.peek(k_pos - 2) == &Square::Empty
            && self.board.peek(k_pos - 3) == &Square::Empty
            && self.board.peek(k_pos - 4) == &Square::Occupied(Piece::Rook(color))
            && !self.attack_map[k_pos - 1]
            && !self.attack_map[k_pos - 2]
    }

    fn gen_castling_moves(&mut self) {
        match self.cur_player.color {
            PieceColor::White => {
                // cheking castling possibility for white king side
                let k_pos = self.king_pos.white;

                if self.castling.white_kingside() {
                    if self.can_castle_kingside(k_pos, PieceColor::White) {
                        self.legal_moves.push(
                            Move::new(k_pos, k_pos + 2, self.castling, self.en_passant_sq)
                                .with_castle_king(),
                        );
                    }
                }

                // cheking castling possibility for white queen side
                if self.castling.white_queenside() {
                    if self.can_castle_queenside(k_pos, PieceColor::White) {
                        self.legal_moves.push(
                            Move::new(k_pos, k_pos - 2, self.castling, self.en_passant_sq)
                                .with_castle_queen(),
                        );
                    }
                }
            }

            PieceColor::Black => {
                // cheking castling possibility for black king side
                let k_pos = self.king_pos.black;

                if self.castling.black_kingside() {
                    if self.can_castle_kingside(k_pos, PieceColor::Black) {
                        self.legal_moves.push(
                            Move::new(k_pos, k_pos + 2, self.castling, self.en_passant_sq)
                                .with_castle_king(),
                        );
                    }
                }

                // cheking castling possibility for black queen side
                if self.castling.black_queenside() {
                    if self.can_castle_queenside(k_pos, PieceColor::Black) {
                        self.legal_moves.push(
                            Move::new(k_pos, k_pos - 2, self.castling, self.en_passant_sq)
                                .with_castle_queen(),
                        );
                    }
                }
            }
        }
    }

    fn filter_illegal(&mut self) {
        let moves = std::mem::take(&mut self.legal_moves);
        let mut legal: Vec<Move> = Vec::new();
        for mov in moves {
            // move the piece
            self.move_piece(&mov);

            if !self.in_check(self.cur_player.color) {
                legal.push(mov);
            }

            self.unmove_piece(&mov);
        }

        self.legal_moves = legal;
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

        let moves = self.moves(from);
        let mov = match moves.iter().find(|mov| mov.to == to) {
            Some(&m) => m,
            None => return false,
        }; // filters if it is not a valid move

        self.move_piece(&mov);

        // Post move activities

        // Handling castling rights
        // piece that moved (after move is applied)
        let moved_piece = self.board.peek(mov.to);

        // KING MOVES
        if matches!(
            moved_piece,
            Square::Occupied(Piece::King(PieceColor::White))
        ) {
            self.castling.remove(WK | WQ);
        }

        if matches!(
            moved_piece,
            Square::Occupied(Piece::King(PieceColor::Black))
        ) {
            self.castling.remove(BK | BQ);
        }

        // ROOK MOVES (from starting squares)
        if matches!(
            moved_piece,
            Square::Occupied(Piece::Rook(PieceColor::White))
        ) {
            if mov.from == WK_START_POS + 3 {
                self.castling.remove(WK); // h1
            } else if mov.from == WK_START_POS - 4 {
                self.castling.remove(WQ); // a1
            }
        }

        if matches!(
            moved_piece,
            Square::Occupied(Piece::Rook(PieceColor::Black))
        ) {
            if mov.from == BK_START_POS + 3 {
                self.castling.remove(BK); // h8
            } else if mov.from == BK_START_POS - 4 {
                self.castling.remove(BQ); // a8
            }
        }

        // ROOK CAPTURED
        if let Some(Piece::Rook(color)) = mov.capture {
            match (color, mov.to) {
                (PieceColor::White, sq) if sq == WK_START_POS + 3 => {
                    self.castling.remove(WK);
                }
                (PieceColor::White, sq) if sq == WK_START_POS - 4 => {
                    self.castling.remove(WQ);
                }
                (PieceColor::Black, sq) if sq == BK_START_POS + 3 => {
                    self.castling.remove(BK);
                }
                (PieceColor::Black, sq) if sq == BK_START_POS - 4 => {
                    self.castling.remove(BQ);
                }
                _ => {}
            }
        }

        // Handling enpassant square
        let is_pawn = matches!(self.board.peek(mov.to), Square::Occupied(Piece::Pawn(_)));

        if is_pawn && mov.from.abs_diff(mov.to) == 16 {
            self.en_passant_sq = Some((mov.from + mov.to) / 2);
        } else {
            self.en_passant_sq = None;
        }

        self.update_king(from, to);

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

        self.unmove_piece(&mov);
        // Post undo activities

        self.castling = mov.prev_castle_rights;
        self.en_passant_sq = mov.prev_en_passant_sq;
        self.update_king(mov.to, mov.from);

        self.change_turn(true);
        self.generate_moves();
    }

    fn update_king(&mut self, from: usize, to: usize) {
        // update king_pos if white king moved
        if self.king_pos.white == from {
            self.king_pos.white = to;
        }

        // update king_pos if black king moved
        if self.king_pos.black == from {
            self.king_pos.black = to;
        }
    }

    fn move_piece(&mut self, mov: &Move) {
        let square = self.board.get(mov.from);

        let square = std::mem::replace(square, Square::Empty);
        self.board.place(square, mov.to);

        if mov.from == 0 {
            println!("Hi, {:#?}", mov);
        }

        // taking care of special moves
        match mov.kind {
            MoveKind::Normal => {}
            MoveKind::Promotion(p) => {
                self.board.place_piece(mov.to, p);
            }
            MoveKind::CastleKing => {
                if let Square::Occupied(Piece::King(color)) = square {
                    match color {
                        PieceColor::White => {
                            self.move_piece(&Move::new(
                                WK_START_POS + 3,
                                WK_START_POS + 1,
                                self.castling,
                                self.en_passant_sq,
                            ));
                        }
                        PieceColor::Black => {
                            self.move_piece(&Move::new(
                                BK_START_POS + 3,
                                BK_START_POS + 1,
                                self.castling,
                                self.en_passant_sq,
                            ));
                        }
                    }
                }
            }
            MoveKind::CastleQueen => {
                if let Square::Occupied(Piece::King(color)) = square {
                    match color {
                        PieceColor::White => {
                            self.move_piece(&Move::new(
                                WK_START_POS - 4,
                                WK_START_POS - 1,
                                self.castling,
                                self.en_passant_sq,
                            ));
                        }
                        PieceColor::Black => {
                            self.move_piece(&Move::new(
                                BK_START_POS - 1,
                                BK_START_POS - 1,
                                self.castling,
                                self.en_passant_sq,
                            ));
                        }
                    }
                }
            }
            MoveKind::EnPassant => {
                let captured_sq = (mov.from / 8) * 8 + (mov.to % 8);
                self.board.place(Square::Empty, captured_sq);
            }
        };

        // Post move activities
        self.update_king(mov.from, mov.to);
    }

    fn unmove_piece(&mut self, mov: &Move) {
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
            MoveKind::CastleKing => {
                if let Square::Occupied(Piece::King(color)) = square {
                    match color {
                        PieceColor::White => {
                            self.move_piece(&Move::new(
                                WK_START_POS + 1,
                                WK_START_POS + 3,
                                self.castling,
                                self.en_passant_sq,
                            ));
                            self.castling.add(WK);
                        }
                        PieceColor::Black => {
                            self.move_piece(&Move::new(
                                BK_START_POS + 1,
                                BK_START_POS + 3,
                                self.castling,
                                self.en_passant_sq,
                            ));
                            self.castling.add(BK);
                        }
                    }
                }
            }
            MoveKind::CastleQueen => {
                if let Square::Occupied(Piece::King(color)) = square {
                    match color {
                        PieceColor::White => {
                            self.move_piece(&Move::new(
                                WK_START_POS - 1,
                                WK_START_POS - 4,
                                self.castling,
                                self.en_passant_sq,
                            ));
                            self.castling.add(WQ);
                        }
                        PieceColor::Black => {
                            self.move_piece(&Move::new(
                                BK_START_POS - 1,
                                BK_START_POS - 4,
                                self.castling,
                                self.en_passant_sq,
                            ));
                            self.castling.add(BQ);
                        }
                    }
                }
            }
            MoveKind::EnPassant => {
                if let Square::Occupied(Piece::Pawn(color)) = square {
                    let captured_sq = (mov.from / 8) * 8 + (mov.to % 8);
                    let captured_pawn = match color {
                        PieceColor::White => Piece::Pawn(PieceColor::Black),
                        PieceColor::Black => Piece::Pawn(PieceColor::White),
                    };
                    self.board.place_piece(captured_sq, captured_pawn);
                }
            }
        }

        self.update_king(mov.to, mov.from);
    }

    pub fn get_game_state(&self) -> GameState {
        let mut res: GameState = GameState::Draw;

        if self.legal_moves.is_empty() {
            match self.cur_player.color {
                PieceColor::Black => {
                    if self.attack_map[self.king_pos.black] {
                        res = GameState::CheckMate(PieceColor::White);
                    }
                }
                PieceColor::White => {
                    if self.attack_map[self.king_pos.white] {
                        res = GameState::CheckMate(PieceColor::Black);
                    }
                }
            }
        } else {
            res = GameState::Playing;
        }

        res
    }

    pub fn restart(&mut self) {
        *self = Self::new(self.get_size());
    }

    #[allow(dead_code)]
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }

        let moves = self.generate_moves();
        let mut nodes = 0;

        for mv in moves {
            self.make_move(mv.from, mv.to);
            nodes += self.perft(depth - 1);
            self.undo_move();
        }

        nodes
    }

    #[allow(dead_code)]
    pub fn perft_divide(&mut self, depth: u32) {
        let moves = self.generate_moves();
        let mut total_nodes = 0;

        for mv in moves {
            self.make_move(mv.from, mv.to);

            let stats = self.perft_debug(depth - 1);

            self.undo_move();

            total_nodes += stats.nodes;

            // print move + nodes
            println!("{}: {}", self.move_to_string(&mv), stats.nodes);
        }

        println!("\nTotal nodes: {}", total_nodes);
    }

    fn move_to_string(&self, mv: &Move) -> String {
        format!(
            "{}{}",
            self.square_to_string(mv.from),
            self.square_to_string(mv.to)
        )
    }

    fn square_to_string(&self, sq: usize) -> String {
        let file = (sq % 8) as u8;
        let rank = (sq / 8) as u8;

        let file_char = (b'a' + file) as char;
        let rank_char = (b'1' + (7 - rank)) as char; // adjust if your board is flipped

        format!("{}{}", file_char, rank_char)
    }

    #[allow(dead_code)]
    pub fn perft_debug(&mut self, depth: u32) -> PerftStats {
        if depth == 0 {
            return PerftStats {
                nodes: 1,
                ..Default::default()
            };
        }

        let moves = self.generate_moves();
        let mut total = PerftStats::default();

        for mv in moves {
            self.make_move(mv.from, mv.to);

            // recurse
            let mut child = self.perft_debug(depth - 1);

            // count this move (only at depth 1 OR always depending on style)
            if depth == 1 {
                child.nodes = 1;

                if mv.capture.is_some() {
                    child.captures += 1;
                }

                if mv.kind == MoveKind::EnPassant {
                    child.en_passant += 1;
                    child.captures += 1;
                }

                if mv.kind == MoveKind::CastleKing || mv.kind == MoveKind::CastleQueen {
                    child.castles += 1;
                }

                if let MoveKind::Promotion(_) = mv.kind {
                    child.promotions += 1;
                }

                // check detection AFTER move
                if self.in_check(self.cur_player.color) {
                    child.checks += 1;
                }

                // checkmate detection
                let moves_after = self.generate_moves();
                if moves_after.is_empty() && self.in_check(self.cur_player.color) {
                    child.checkmates += 1;
                }
            }

            self.undo_move();

            // accumulate
            total.nodes += child.nodes;
            total.captures += child.captures;
            total.en_passant += child.en_passant;
            total.castles += child.castles;
            total.promotions += child.promotions;
            total.checks += child.checks;
            total.discovered_checks += child.discovered_checks;
            total.double_checks += child.double_checks;
            total.checkmates += child.checkmates;
        }

        total
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

#[derive(Debug)]
struct KingPos {
    white: usize,
    black: usize,
}

pub enum GameState {
    Playing,
    CheckMate(PieceColor),
    Draw,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CastlingRights(pub u8);

impl CastlingRights {
    pub fn new() -> Self {
        Self(WK | WQ | BK | BQ)
    }

    pub fn white_kingside(self) -> bool {
        self.0 & WK != 0
    }

    pub fn white_queenside(self) -> bool {
        self.0 & WQ != 0
    }

    pub fn black_kingside(self) -> bool {
        self.0 & BK != 0
    }

    pub fn black_queenside(self) -> bool {
        self.0 & BQ != 0
    }

    // remove rights
    pub fn remove(&mut self, mask: u8) {
        self.0 &= !mask;
    }

    // add rights
    pub fn add(&mut self, mask: u8) {
        self.0 |= mask;
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct PerftStats {
    pub nodes: u64,
    pub captures: u64,
    pub en_passant: u64,
    pub castles: u64,
    pub promotions: u64,
    pub checks: u64,
    pub discovered_checks: u64,
    pub double_checks: u64,
    pub checkmates: u64,
}

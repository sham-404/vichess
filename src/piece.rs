use crate::board::Board;
use crate::game::Square;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pos {
    pub row: i32,
    pub col: i32,
}

impl Pos {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn offset(&self, dr: i32, dc: i32) -> Self {
        Self {
            row: self.row + dr,
            col: self.col + dc,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    kind: PieceKind,
    pos: Pos,
    color: MyColor,
}

impl Piece {
    pub fn new(kind: PieceKind, pos: Pos, color: MyColor) -> Self {
        Self { kind, pos, color }
    }

    pub fn name(&self) -> String {
        self.kind.get_name()
    }

    pub fn color(&self) -> MyColor {
        self.color
    }

    pub fn change_pos(&mut self, to: Pos) {
        self.pos = to;
    }

    pub fn get_piece_moves(&self, board: &Board) -> Vec<Pos> {
        let moves = match self.kind {
            PieceKind::Pawn => self.pawn_moves(board),
            PieceKind::King => self.king_moves(board),
            _ => Vec::<Pos>::new(),
        };
        moves
    }

    fn get_pawn_dir(pawn: &Piece) -> i32 {
        if pawn.color == MyColor::White {
            return 1;
        }
        -1
    }

    fn pawn_moves(&self, board: &Board) -> Vec<Pos> {
        let mut moves: Vec<Pos> = Vec::new();
        let dir = Self::get_pawn_dir(&self);
        let new_pos = self.pos.offset(dir, 0);

        if board.within_bounds(&new_pos) {
            if matches!(board.peek(new_pos), Square::Empty) {
                moves.push(new_pos);
            }
        }

        let new_pos = self.pos.offset(dir * 2, 0);

        if board.within_bounds(&new_pos) {
            if matches!(board.peek(new_pos), Square::Empty) {
                moves.push(new_pos);
            }
        }

        moves
    }

    fn king_moves(&self, board: &Board) -> Vec<Pos> {
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
            let new_pos = self.pos.offset(dr, dc);

            if !board.within_bounds(&new_pos) {
                continue;
            }

            let square = board.peek(new_pos);
            match square {
                Square::_NotExists => continue,
                Square::Empty => moves.push(new_pos),
                Square::Occupied(p) => {
                    if p.color != self.color {
                        moves.push(new_pos);
                    }
                }
            }
        }

        moves
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl PieceKind {
    pub fn get_name(&self) -> String {
        match self {
            Self::King => "K".to_string(),
            Self::Queen => "Q".to_string(),
            Self::Rook => "R".to_string(),
            Self::Knight => "N".to_string(),
            Self::Bishop => "B".to_string(),
            Self::Pawn => "P".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyColor {
    White,
    Black,
}

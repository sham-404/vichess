use crate::board::Board;
use crate::game::Square;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
pub struct Piece {
    kind: PieceKind,
    pos: Pos,
    color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, pos: Pos, color: Color) -> Self {
        Self { kind, pos, color }
    }

    fn king_moves(&self, king: Piece, board: Board) -> Vec<Pos> {
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
            let new_pos = king.pos.offset(dr, dc);

            if !board.within_bounds(new_pos) {
                continue;
            }

            let square = board.get(new_pos);
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

    pub fn name(&self) -> String {
        self.kind.get_name()
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
pub enum Color {
    White,
    Black,
}

use crate::game::{Game, Square};

#[derive(Debug, Copy, Clone)]
pub struct Pos {
    pub row: i32,
    pub col: i32,
}

impl Pos {
    pub fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    _kind: PieceKind,
    _pos: Pos,
    _color: Color,
    moves: Vec<usize>,
    attacking_squares: Vec<usize>,
}

impl Piece {
    pub fn new(kind: PieceKind, pos: Pos, color: Color) -> Self {
        let moves: Vec<usize> = Vec::new();
        let attacking_squares: Vec<usize> = Vec::new();
        Self {
            _kind: kind,
            _pos: pos,
            _color: color,
            moves,
            attacking_squares,
        }
    }


    fn king_moves(&mut self, board: &Vec<Square>) {
        self.moves.clear();
        self.attacking_squares.clear();

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

        for (row, col) in offset {
            let (dx, dy) = (self._pos.row + row, self._pos.col + col);


        }
    }

    pub fn name(&self) -> String {
        self._kind.get_name()
    }

    pub fn generate_moves(board: Vec<Square>) {}
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

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

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
}

impl Piece {
    pub fn new(kind: PieceKind, pos: Pos, color: Color) -> Self {
        Self {
            _kind: kind,
            _pos: pos,
            _color: color,
        }
    }

    pub fn name(&self) -> String {
        self._kind.get_name()
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

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

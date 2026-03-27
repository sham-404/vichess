#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub pos: usize,
    pub name: String,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, pos: usize, color: Color) -> Self {
        let name = kind.get_name();
        Self { kind, pos, name, color }
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

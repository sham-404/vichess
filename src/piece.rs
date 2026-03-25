#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub pos: usize,
    pub name: char,
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
    pub fn get_name(&self) -> char {
        match self {
            Self::King => 'K',
            Self::Queen => 'Q',
            Self::Rook => 'R',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Pawn => 'P',
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

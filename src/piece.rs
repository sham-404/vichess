#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: PieceKind,
    pub pos: (usize, usize),
    pub name: char,
}

impl Piece {
    pub fn new(kind: PieceKind, pos: (usize, usize)) -> Self {
        let name = kind.get_name();
        Self { kind, pos, name }
    }
}

#[derive(Debug, Clone)]
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

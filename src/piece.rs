#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor) -> Self {
        Self { kind, color }
    }

    pub fn name(&self) -> String {
        self.kind.get_name()
    }

    pub fn color(&self) -> PieceColor {
        self.color
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

    pub fn get_dir(&self) -> &[i32] {
        match self {
            Self::Rook => &[8, -8, 1, -1],
            Self::Bishop => &[9, 7, -7, -9],
            Self::Queen => &[8, -8, 1, -1, 9, 7, -7, -9],
            Self::Knight => &[17, 15, 10, 6, -6, -10, -15, -17],
            Self::King => &[8, -8, 1, -1, 9, 7, -7, -9],
            Self::Pawn => panic!("Improbable case, find the error asap"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

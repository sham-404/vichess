#[derive(Debug, Clone, Copy)]
pub enum Piece {
    King(PieceColor),
    Queen(PieceColor),
    Rook(PieceColor),
    Knight(PieceColor),
    Bishop(PieceColor),
    Pawn(PieceColor),
}

impl Piece {
    pub fn get_name(&self) -> String {
        match self {
            Self::King(_) => "K".to_string(),
            Self::Queen(_) => "Q".to_string(),
            Self::Rook(_) => "R".to_string(),
            Self::Knight(_) => "N".to_string(),
            Self::Bishop(_) => "B".to_string(),
            Self::Pawn(_) => "P".to_string(),
        }
    }

    pub fn get_dir(&self) -> &[i32] {
        match self {
            Self::Rook(_) => &[8, -8, 1, -1],
            Self::Bishop(_) => &[9, 7, -7, -9],
            Self::Queen(_) => &[8, -8, 1, -1, 9, 7, -7, -9],
            Self::Knight(_) => &[17, 15, 10, 6, -6, -10, -15, -17],
            Self::King(_) => &[8, -8, 1, -1, 9, 7, -7, -9],
            Self::Pawn(_) => panic!("Improbable case, find the error asap"),
        }
    }

    pub fn color(&self) -> &PieceColor {
        match self {
            Self::Queen(c)
            | Self::King(c)
            | Self::Rook(c)
            | Self::Knight(c)
            | Self::Bishop(c)
            | Self::Pawn(c) => c,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

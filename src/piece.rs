#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub capture: Option<Piece>,
    pub kind: MoveKind,
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            capture: None,
            kind: MoveKind::Normal,
        }
    }

    pub fn with_capture(mut self, piece: Piece) -> Self {
        self.capture = Some(piece);
        self.kind = MoveKind::Capture;
        self
    }

    pub fn promote_to(&mut self, piece: Piece) {
        self.kind = MoveKind::Promotion(piece);
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum MoveKind {
    Normal,
    Capture,
    CastleKing,
    CastleQueen,
    Promotion(Piece),
    EnPassant,
}

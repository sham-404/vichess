// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub struct Pos {
//     pub row: i32,
//     pub col: i32,
// }
//
// impl Pos {
//     pub fn new(row: i32, col: i32) -> Self {
//         Self { row, col }
//     }
//
//     pub fn offset(&self, dr: i32, dc: i32) -> Self {
//         Self {
//             row: self.row + dr,
//             col: self.col + dc,
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: MyColor,
}

impl Piece {
    pub fn new(kind: PieceKind, color: MyColor) -> Self {
        Self { kind, color }
    }

    pub fn name(&self) -> String {
        self.kind.get_name()
    }

    pub fn color(&self) -> MyColor {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyColor {
    White,
    Black,
}

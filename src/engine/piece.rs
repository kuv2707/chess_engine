use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceColor {
    WHITE,
    BLACK,
}
impl PieceColor {
    pub fn opponent_color(&self) -> PieceColor {
        match self {
            PieceColor::WHITE => PieceColor::BLACK,
            PieceColor::BLACK => PieceColor::WHITE,
        }
    }
    pub fn get_value(&self) -> i32 {
        match self {
            PieceColor::WHITE => 1,
            PieceColor::BLACK => -1,
        }
    }
}
impl fmt::Display for PieceColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieceColor::WHITE => write!(f, "WHITE_COLOR"),
            PieceColor::BLACK => write!(f, "BLACK_COLOR"),
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece = match self.piece_type {
            super::piece::PieceType::PAWN => "P",
            super::piece::PieceType::KNIGHT => "N",
            super::piece::PieceType::BISHOP => "B",
            super::piece::PieceType::ROOK => "R",
            super::piece::PieceType::QUEEN => "Q",
            super::piece::PieceType::KING => "K",
        };
        let fen: String = match self.color {
            super::piece::PieceColor::WHITE => piece.to_uppercase(),
            super::piece::PieceColor::BLACK => piece.to_lowercase(),
        };
        write!(f, "{}", fen)
    }
}

impl Piece {
    // pub fn new(color: PieceColor, piece_type: PieceType) -> Piece {
    //     Piece {
    //         color: color,
    //         piece_type: piece_type,
    //     }
    // }
    pub fn get_color(&self) -> PieceColor {
        self.color
    }
    // pub fn moves(&self, base: super::board::Position) -> Vec<super::board::Position> {
    //     match self.piece_type {
    //         PieceType::PAWN => pawn_moves(base, self.color),
    //         PieceType::KNIGHT => knight_moves(base),
    //         PieceType::BISHOP => bishop_moves(base),
    //         PieceType::ROOK => rook_moves(base),
    //         PieceType::QUEEN => queen_moves(base),
    //         PieceType::KING => king_moves(base),
    //     }
    // }
}

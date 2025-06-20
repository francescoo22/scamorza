use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub const PROMOTABLE_KINDS: [PieceKind; 4] = [
    PieceKind::Queen,
    PieceKind::Rook,
    PieceKind::Bishop,
    PieceKind::Knight,
];

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

pub const WHITE_PAWN: Piece = Piece {
    kind: PieceKind::Pawn,
    color: Color::White,
};

pub const WHITE_ROOK: Piece = Piece {
    kind: PieceKind::Rook,
    color: Color::White,
};

pub const WHITE_KNIGHT: Piece = Piece {
    kind: PieceKind::Knight,
    color: Color::White,
};

pub const WHITE_BISHOP: Piece = Piece {
    kind: PieceKind::Bishop,
    color: Color::White,
};

pub const WHITE_QUEEN: Piece = Piece {
    kind: PieceKind::Queen,
    color: Color::White,
};

pub const WHITE_KING: Piece = Piece {
    kind: PieceKind::King,
    color: Color::White,
};

pub const BLACK_PAWN: Piece = Piece {
    kind: PieceKind::Pawn,
    color: Color::Black,
};

pub const BLACK_ROOK: Piece = Piece {
    kind: PieceKind::Rook,
    color: Color::Black,
};

pub const BLACK_KNIGHT: Piece = Piece {
    kind: PieceKind::Knight,
    color: Color::Black,
};

pub const BLACK_BISHOP: Piece = Piece {
    kind: PieceKind::Bishop,
    color: Color::Black,
};

pub const BLACK_QUEEN: Piece = Piece {
    kind: PieceKind::Queen,
    color: Color::Black,
};

pub const BLACK_KING: Piece = Piece {
    kind: PieceKind::King,
    color: Color::Black,
};

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let symbol = match (self.color, self.kind) {
            (Color::White, PieceKind::Pawn) => "♙",
            (Color::White, PieceKind::Rook) => "♖",
            (Color::White, PieceKind::Knight) => "♘",
            (Color::White, PieceKind::Bishop) => "♗",
            (Color::White, PieceKind::Queen) => "♕",
            (Color::White, PieceKind::King) => "♔",

            (Color::Black, PieceKind::Pawn) => "♟",
            (Color::Black, PieceKind::Rook) => "♜",
            (Color::Black, PieceKind::Knight) => "♞",
            (Color::Black, PieceKind::Bishop) => "♝",
            (Color::Black, PieceKind::Queen) => "♛",
            (Color::Black, PieceKind::King) => "♚",
        };

        write!(f, "{}", symbol)
    }
}

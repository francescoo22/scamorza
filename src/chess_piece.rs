use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub(crate) kind: PieceKind,
    pub(crate) color: Color,
}

impl Piece {
    pub(crate) fn white_pawn() -> Self {
        Piece {
            kind: PieceKind::Pawn,
            color: Color::White,
        }
    }

    pub(crate) fn white_queen() -> Self {
        Piece {
            kind: PieceKind::Queen,
            color: Color::White,
        }
    }

    pub(crate) fn black_pawn() -> Self {
        Piece {
            kind: PieceKind::Pawn,
            color: Color::Black,
        }
    }

    pub(crate) fn black_queen() -> Self {
        Piece {
            kind: PieceKind::Queen,
            color: Color::Black,
        }
    }
}

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

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
    pub(crate) kind: PieceKind,
    pub(crate) color: Color,
}

impl Piece {
    // todo: check whether there is a better way to have this kind of functions
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

    pub(crate) fn white_king() -> Self {
        Piece {
            kind: PieceKind::King,
            color: Color::White,
        }
    }

    pub(crate) fn white_rook() -> Self {
        Piece {
            kind: PieceKind::Rook,
            color: Color::White,
        }
    }

    pub(crate) fn white_knight() -> Self {
        Piece {
            kind: PieceKind::Knight,
            color: Color::White,
        }
    }

    pub(crate) fn white_bishop() -> Self {
        Piece {
            kind: PieceKind::Bishop,
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
    pub(crate) fn black_king() -> Self {
        Piece {
            kind: PieceKind::King,
            color: Color::Black,
        }
    }

    pub(crate) fn black_rook() -> Self {
        Piece {
            kind: PieceKind::Rook,
            color: Color::Black,
        }
    }

    pub(crate) fn black_knight() -> Self {
        Piece {
            kind: PieceKind::Knight,
            color: Color::Black,
        }
    }

    pub(crate) fn black_bishop() -> Self {
        Piece {
            kind: PieceKind::Bishop,
            color: Color::Black,
        }
    }
}

pub(crate) fn knight_directions() -> [(i32, i32); 8] {
    [
        (1, 2),
        (2, 1),
        (-1, 2),
        (2, -1),
        (1, -2),
        (-2, 1),
        (-1, -2),
        (-2, -1),
    ]
}

pub(crate) fn rook_directions() -> [(i32, i32); 4] {
    [(0, 1), (1, 0), (-1, 0), (0, -1)]
}

pub(crate) fn bishop_directions() -> [(i32, i32); 4] {
    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
}

pub fn king_directions() -> [(i32, i32); 8] {
    [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
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

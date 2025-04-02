use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy)]
enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Piece {
    kind: PieceKind,
    color: Color,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Copy, Clone)]
enum Cell {
    Empty,
    Occupied(Piece),
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, "-"),
            Cell::Occupied(piece) => write!(f, "{}", piece),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    pieces: [[Cell; 8]; 8],
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.pieces {
            for cell in row {
                write!(f, " {}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ChessBoard {
    pub fn initial_board() -> Self {
        let mut board = [[Cell::Empty; 8]; 8];
        for i in 0..8 {
            board[1][i] = Cell::Occupied(Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            });
            board[6][i] = Cell::Occupied(Piece {
                kind: PieceKind::Pawn,
                color: Color::Black,
            });
        }

        for i in 0..8 {
            let kind = match i {
                0 => PieceKind::Rook,
                1 => PieceKind::Knight,
                2 => PieceKind::Bishop,
                3 => PieceKind::King,
                4 => PieceKind::Queen,
                5 => PieceKind::Bishop,
                6 => PieceKind::Knight,
                7 => PieceKind::Rook,
                _ => unreachable!(),
            };
            board[0][i] = Cell::Occupied(Piece {
                kind,
                color: Color::White,
            });
            board[7][i] = Cell::Occupied(Piece {
                kind,
                color: Color::Black,
            });
        }
        Self { pieces: board }
    }

    pub fn move_piece(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.pieces[to.0][to.1] = self.pieces[from.0][from.1];
        self.pieces[from.0][from.1] = Cell::Empty;
    }
}

use crate::chess_move::Move;
use crate::chess_piece::{Color, Piece, PieceKind};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Square::Empty => write!(f, "-"),
            Square::Occupied(piece) => write!(f, "{}", piece),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    squares: [[Square; 8]; 8],
}

fn within_bounds(i: i32, j: i32) -> bool {
    i < 8 && j < 8 && i >= 0 && j >= 0
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in &self.squares {
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
        let mut board = [[Square::Empty; 8]; 8];
        for i in 0..8 {
            board[1][i] = Square::Occupied(Piece {
                kind: PieceKind::Pawn,
                color: Color::White,
            });
            board[6][i] = Square::Occupied(Piece {
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
            board[0][i] = Square::Occupied(Piece {
                kind,
                color: Color::White,
            });
            board[7][i] = Square::Occupied(Piece {
                kind,
                color: Color::Black,
            });
        }
        Self { squares: board }
    }

    pub(crate) fn for_each_piece<F>(&self, mut block: F)
    where
        F: FnMut(i32, i32, &Piece),
    {
        for i in 0..8 {
            for j in 0..8 {
                match self.squares[i as usize][j as usize] {
                    Square::Empty => {}
                    Square::Occupied(piece) => {
                        block(i, j, &piece);
                    }
                }
            }
        }
    }

    pub(crate) fn within_bounds_and_empty(&self, i: i32, j: i32) -> bool {
        within_bounds(i, j) && self.squares[i as usize][j as usize] == Square::Empty
    }

    pub(crate) fn within_bounds_and_occupied_by_opponent(
        &self,
        i: i32,
        j: i32,
        color: &Color,
    ) -> bool {
        if !within_bounds(i, j) {
            return false;
        }
        match self.squares[i as usize][j as usize] {
            Square::Empty => false,
            Square::Occupied(piece) => piece.color != *color,
        }
    }

    pub fn at(&self, i: usize, j: usize) -> Square {
        self.squares[i][j]
    }

    pub fn set_at(&mut self, i: usize, j: usize, square: Square) {
        self.squares[i][j] = square;
    }

    pub fn piece_at_source(self, mov: &Move) -> Piece {
        match self.squares[mov.from.0][mov.from.1] {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        }
    }
}

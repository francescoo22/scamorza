use crate::chess_move::Move;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;

#[derive(Clone, Copy, PartialEq)]
enum PieceKind {
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

#[derive(Clone, Copy, PartialEq)]
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

#[derive(Copy, Clone, PartialEq)]
enum Square {
    Empty,
    Occupied(Piece),
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

    pub fn move_piece(&mut self, uci: &str) {
        let mov = Move::from_uci_string(uci);
        self.squares[mov.to.0][mov.to.1] = self.squares[mov.from.0][mov.from.1];
        self.squares[mov.from.0][mov.from.1] = Square::Empty;
    }

    fn for_each_piece<F>(&self, mut block: F)
    where
        F: FnMut(usize, usize, &Piece),
    {
        for i in 0..8 {
            for j in 0..8 {
                match self.squares[i][j] {
                    Square::Empty => {}
                    Square::Occupied(piece) => {
                        block(i, j, &piece);
                    }
                }
            }
        }
    }

    fn pawn_valid_moves(&self, i: usize, j: usize, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        match color {
            Color::White => {
                if self.squares[i + 1][j] == Square::Empty {
                    moves.push(Move {
                        from: (i, j),
                        to: (i + 1, j),
                    });
                    if i == 1 && self.squares[i + 2][j] == Square::Empty {
                        moves.push(Move {
                            from: (i, j),
                            to: (i + 2, j),
                        });
                    }
                }
                if j < 7 {
                    match self.squares[i + 1][j + 1] {
                        Square::Occupied(other_piece) => {
                            if other_piece.color != *color {
                                moves.push(Move {
                                    from: (i, j),
                                    to: (i + 1, j + 1),
                                });
                            }
                        }
                        Square::Empty => {}
                    }
                }
                if j > 0 {
                    match self.squares[i + 1][j - 1] {
                        Square::Occupied(other_piece) => {
                            if other_piece.color != *color {
                                moves.push(Move {
                                    from: (i, j),
                                    to: (i + 1, j - 1),
                                });
                            }
                        }
                        Square::Empty => {}
                    }
                }
            }
            Color::Black => {
                todo!()
            }
        }
        moves
    }

    fn piece_valid_moves(&self, i: usize, j: usize, piece: &Piece) -> Vec<Move> {
        match piece.kind {
            PieceKind::Pawn => self.pawn_valid_moves(i, j, &piece.color),
            _ => Vec::new(), // PieceKind::Rook => { Vec::new() }
            // PieceKind::Knight => {}
            // PieceKind::Bishop => {}
            // PieceKind::Queen => {}
            // PieceKind::King => {}
        }
    }

    pub fn all_valid_moves(&self, move_color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        self.for_each_piece(|i, j, piece| {
            if piece.color == move_color {
                let valid_moves = self.piece_valid_moves(i, j, piece);
                println!(
                    "{}: {}",
                    piece,
                    valid_moves
                        .iter()
                        .map(|m| m.to_uci_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                );
                moves.extend(valid_moves);
            }
        });
        moves
    }
}

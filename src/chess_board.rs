use crate::chess_move::Move;
use crate::chess_piece::{Color, Piece, PieceKind};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Not;

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
    pub squares: [[Square; 8]; 8],
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

    pub fn move_piece(&mut self, mov: &Move) {
        let moving_piece = match self.squares[mov.from.0][mov.from.1] {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        };

        let promoted_piece = if mov.to.0 == 7 && moving_piece == Piece::white_pawn() {
            Piece::white_queen()
        } else if mov.to.0 == 0 && moving_piece == Piece::black_pawn() {
            Piece::black_queen()
        } else {
            moving_piece
        };

        self.squares[mov.from.0][mov.from.1] = Square::Empty;
        self.squares[mov.to.0][mov.to.1] = Square::Occupied(promoted_piece);
    }

    pub fn move_piece_uci(&mut self, uci: &str) {
        let mov = Move::from_uci_string(uci);
        self.move_piece(&mov);
    }

    pub fn move_piece_back(&mut self, mov: &Move) {
        self.move_piece(&mov.not());
    }
    fn for_each_piece<F>(&self, mut block: F)
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

    fn within_bounds_and_empty(&self, i: i32, j: i32) -> bool {
        within_bounds(i, j) && self.squares[i as usize][j as usize] == Square::Empty
    }

    fn within_bounds_and_occupied_by_opponent(&self, i: i32, j: i32, color: &Color) -> bool {
        if !within_bounds(i, j) {
            return false;
        }
        match self.squares[i as usize][j as usize] {
            Square::Empty => false,
            Square::Occupied(piece) => piece.color != *color,
        }
    }

    fn slider_valid_moves(
        &self,
        i: i32,
        j: i32,
        color: &Color,
        directions: &Vec<(i32, i32)>,
    ) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for (dx, dy) in directions {
            let mut dist = 1;
            while self.within_bounds_and_empty(i + dist * dy, j + dist * dx) {
                dist += 1;
            }
            for cur_dist in 1..dist {
                moves.push(Move {
                    from: (i as usize, j as usize),
                    to: ((i + cur_dist * dy) as usize, (j + cur_dist * dx) as usize),
                });
            }
            if self.within_bounds_and_occupied_by_opponent(i + dist * dy, j + dist * dx, color) {
                moves.push(Move {
                    from: (i as usize, j as usize),
                    to: ((i + dist * dy) as usize, (j + dist * dx) as usize),
                });
            }
        }
        moves
    }

    fn leaper_valid_moves(
        &self,
        i: i32,
        j: i32,
        color: &Color,
        directions: &Vec<(i32, i32)>,
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        for (dx, dy) in directions {
            if self.within_bounds_and_empty(i + dy, j + dx)
                || self.within_bounds_and_occupied_by_opponent(i + dy, j + dx, color)
            {
                moves.push(Move {
                    from: (i as usize, j as usize),
                    to: ((i + dy) as usize, (j + dx) as usize),
                });
            }
        }

        moves
    }

    fn knight_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        let dirs = vec![
            (2, 1),
            (1, 2),
            (-1, 2),
            (-2, 1),
            (-2, -1),
            (-1, -2),
            (1, -2),
            (2, -1),
        ];
        self.leaper_valid_moves(i, j, color, &dirs)
    }

    fn king_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        // todo: castle
        let dirs = vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        self.leaper_valid_moves(i, j, color, &dirs)
    }

    fn rook_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        let dirs = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
        self.slider_valid_moves(i, j, color, &dirs)
    }

    fn bishop_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        let dirs = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
        self.slider_valid_moves(i, j, color, &dirs)
    }

    fn queen_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        let dirs = vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ];
        self.slider_valid_moves(i, j, color, &dirs)
    }

    fn pawn_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        // todo: en-passant
        let mut moves = Vec::new();
        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let initial_row = match color {
            Color::White => 1,
            Color::Black => 6,
        };
        if self.within_bounds_and_empty(i + dir, j) {
            moves.push(Move {
                from: (i as usize, j as usize),
                to: ((i + dir) as usize, j as usize),
            });
            if self.within_bounds_and_empty(i + 2 * dir, j) && i == initial_row {
                moves.push(Move {
                    from: (i as usize, j as usize),
                    to: ((i + 2 * dir) as usize, j as usize),
                });
            }
        }
        if self.within_bounds_and_occupied_by_opponent(i + dir, j + 1, color) {
            moves.push(Move {
                from: (i as usize, j as usize),
                to: ((i + dir) as usize, (j + 1) as usize),
            });
        }
        if self.within_bounds_and_occupied_by_opponent(i + dir, j - 1, color) {
            moves.push(Move {
                from: (i as usize, j as usize),
                to: ((i + dir) as usize, (j - 1) as usize),
            });
        }
        moves
    }

    fn piece_valid_moves(&self, i: i32, j: i32, piece: &Piece) -> Vec<Move> {
        match piece.kind {
            PieceKind::Pawn => self.pawn_valid_moves(i, j, &piece.color),
            PieceKind::Rook => self.rook_valid_moves(i, j, &piece.color),
            PieceKind::Bishop => self.bishop_valid_moves(i, j, &piece.color),
            PieceKind::Queen => self.queen_valid_moves(i, j, &piece.color),
            PieceKind::Knight => self.knight_valid_moves(i, j, &piece.color),
            PieceKind::King => self.king_valid_moves(i, j, &piece.color),
        }
    }

    pub fn all_possible_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        self.for_each_piece(|i, j, piece| {
            if piece.color == color {
                let valid_moves = self.piece_valid_moves(i, j, piece);
                moves.extend(valid_moves);
            }
        });
        moves
    }

    fn filter_king_going_under_check(&self, moves: Vec<Move>) -> Vec<Move> {
        moves
            .iter()
            .filter(|mov| {
                let piece = self.piece_at_source(mov);
                let mut board_after_move = self.clone();
                board_after_move.move_piece_uci(&mov.to_uci_string());
                !board_after_move.is_king_checked(piece.color)
            })
            .copied()
            .collect()
    }

    pub fn all_valid_moves(&self, color: Color) -> Vec<Move> {
        let moves = self.all_possible_moves(color);
        self.filter_king_going_under_check(moves)
    }

    pub fn piece_at_source(self, mov: &Move) -> Piece {
        match self.squares[mov.from.0][mov.from.1] {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        }
    }
}

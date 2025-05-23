use crate::chess_move::Move;
use crate::chess_piece::{Color, Piece, PieceKind};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

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

pub type BitBoard = u64;

#[derive(Clone, Copy)]
pub struct ChessBoard {
    side_to_move: Color,

    pub white_pieces: BitBoard,
    pub black_pieces: BitBoard,

    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queens: BitBoard,
    pub kings: BitBoard,

    pub(crate) can_white_castle_kingside: bool,
    pub(crate) can_white_castle_queenside: bool,
    pub(crate) can_black_castle_kingside: bool,
    pub(crate) can_black_castle_queenside: bool,

    pub(crate) en_passant_target_square: Option<(usize, usize)>,
}

fn within_bounds(i: i32, j: i32) -> bool {
    i < 8 && j < 8 && i >= 0 && j >= 0
}

pub fn square_mask(i: usize, j: usize) -> BitBoard {
    // TODO: Replace '7 - j' with 'j'
    // '7 - j' is done to be coherent with previous representation.
    // It would be better to change representation and use 'j' instead of '7 - j'
    1 << (i * 8 + (7 - j))
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for i in 0..8 {
            for j in 0..8 {
                write!(f, " {}", self.at(i, j))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl ChessBoard {
    pub(crate) fn for_each_piece<F>(&self, mut block: F)
    where
        F: FnMut(i32, i32, &Piece),
    {
        for i in 0..8 {
            for j in 0..8 {
                match self.at(i as usize, j as usize) {
                    Square::Empty => {}
                    Square::Occupied(piece) => {
                        block(i, j, &piece);
                    }
                }
            }
        }
    }

    pub(crate) fn within_bounds_and_empty(&self, i: i32, j: i32) -> bool {
        within_bounds(i, j) && self.at(i as usize, j as usize) == Square::Empty
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

        match self.at(i as usize, j as usize) {
            Square::Empty => false,
            Square::Occupied(piece) => piece.color != *color,
        }
    }

    pub(crate) fn within_bounds_and_pawn_take_target(&self, i: i32, j: i32, color: &Color) -> bool {
        if !within_bounds(i, j) {
            return false;
        }

        if self.en_passant_target_square == Some((i as usize, j as usize)) {
            return true;
        }

        match self.at(i as usize, j as usize) {
            Square::Empty => false,
            Square::Occupied(piece) => piece.color != *color,
        }
    }

    pub fn at(&self, i: usize, j: usize) -> Square {
        let square_mask = square_mask(i, j);

        match square_mask & (self.white_pieces | self.black_pieces) {
            0 => Square::Empty,
            _ => {
                let color = match square_mask & self.white_pieces {
                    0 => Color::Black,
                    _ => Color::White,
                };
                let kind = if square_mask & self.pawns != 0 {
                    PieceKind::Pawn
                } else if square_mask & self.knights != 0 {
                    PieceKind::Knight
                } else if square_mask & self.bishops != 0 {
                    PieceKind::Bishop
                } else if square_mask & self.rooks != 0 {
                    PieceKind::Rook
                } else if square_mask & self.queens != 0 {
                    PieceKind::Queen
                } else if square_mask & self.kings != 0 {
                    PieceKind::King
                } else {
                    unreachable!("Mismatch between color bitboard and piece bitboard")
                };
                Square::Occupied(Piece { kind, color })
            }
        }
    }

    pub fn maybe_piece_at(&self, i: i32, j: i32) -> Option<Piece> {
        if within_bounds(i, j) {
            match self.at(i as usize, j as usize) {
                Square::Empty => None,
                Square::Occupied(piece) => Some(piece),
            }
        } else {
            None
        }
    }

    pub fn set_at(&mut self, i: usize, j: usize, square: Square) {
        let square_mask = square_mask(i, j);
        self.white_pieces &= !square_mask;
        self.black_pieces &= !square_mask;
        self.pawns &= !square_mask;
        self.knights &= !square_mask;
        self.bishops &= !square_mask;
        self.rooks &= !square_mask;
        self.queens &= !square_mask;
        self.kings &= !square_mask;
        match square {
            Square::Occupied(piece) => {
                match piece.color {
                    Color::White => self.white_pieces |= square_mask,
                    Color::Black => self.black_pieces |= square_mask,
                }
                match piece.kind {
                    PieceKind::Pawn => self.pawns |= square_mask,
                    PieceKind::Rook => self.rooks |= square_mask,
                    PieceKind::Knight => self.knights |= square_mask,
                    PieceKind::Bishop => self.bishops |= square_mask,
                    PieceKind::Queen => self.queens |= square_mask,
                    PieceKind::King => self.kings |= square_mask,
                }
            }
            Square::Empty => {}
        }
    }

    pub fn piece_at_source_or_panic(self, mov: &Move) -> Piece {
        match self.at(mov.from.0, mov.from.1) {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        }
    }

    pub fn contains_piece_at(self, i: i32, j: i32, piece_to_find: Piece) -> bool {
        match self.maybe_piece_at(i, j) {
            None => false,
            Some(piece) => piece == piece_to_find,
        }
    }

    pub fn contains_piece_in_any_direction(
        self,
        i: i32,
        j: i32,
        piece_to_find: Piece,
        directions: Vec<(i32, i32)>,
    ) -> bool {
        directions
            .iter()
            .any(|(di, dj)| self.contains_piece_at(i + di, j + dj, piece_to_find))
    }

    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }
}

impl Default for ChessBoard {
    fn default() -> Self {
        Self {
            white_pieces: 0x000000000000FFFF,
            black_pieces: 0xFFFF000000000000,
            pawns: 0x00FF00000000FF00,
            knights: 0x4200000000000042,
            bishops: 0x2400000000000024,
            rooks: 0x8100000000000081,
            queens: 0x0800000000000008,
            kings: 0x1000000000000010,
            side_to_move: Color::White,
            can_white_castle_queenside: true,
            can_white_castle_kingside: true,
            can_black_castle_queenside: true,
            can_black_castle_kingside: true,
            en_passant_target_square: None,
        }
    }
}

impl FromStr for ChessBoard {
    type Err = String;

    fn from_str(fen: &str) -> Result<Self, Self::Err> {
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        let mut pawns = 0;
        let mut knights = 0;
        let mut bishops = 0;
        let mut rooks = 0;
        let mut queens = 0;
        let mut kings = 0;

        let parts = fen.split(" ").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 6, "Invalid FEN, expected 6 parts");

        let rows = parts[0].split("/").collect::<Vec<&str>>();
        assert_eq!(rows.len(), 8, "Invalid FEN, expected 8 rows");

        for (i, row) in rows.iter().enumerate() {
            let mut j = 0;
            for c in row.chars() {
                match c {
                    '1'..='8' => {
                        j += c as usize - '0' as usize;
                    }
                    _ => {
                        let piece = match c {
                            'P' => Piece::white_pawn(),
                            'N' => Piece::white_knight(),
                            'B' => Piece::white_bishop(),
                            'R' => Piece::white_rook(),
                            'Q' => Piece::white_queen(),
                            'K' => Piece::white_king(),
                            'p' => Piece::black_pawn(),
                            'n' => Piece::black_knight(),
                            'b' => Piece::black_bishop(),
                            'r' => Piece::black_rook(),
                            'q' => Piece::black_queen(),
                            'k' => Piece::black_king(),
                            _ => panic!(
                                "Invalid FEN character, expected a piece representation (PNBRQKpnbrqk), found '{}'",
                                c
                            ),
                        };

                        match piece.color {
                            Color::White => white_pieces |= square_mask(7 - i, 7 - j),
                            Color::Black => black_pieces |= square_mask(7 - i, 7 - j),
                        }

                        match piece.kind {
                            PieceKind::Pawn => pawns |= square_mask(7 - i, 7 - j),
                            PieceKind::Knight => knights |= square_mask(7 - i, 7 - j),
                            PieceKind::Bishop => bishops |= square_mask(7 - i, 7 - j),
                            PieceKind::Rook => rooks |= square_mask(7 - i, 7 - j),
                            PieceKind::Queen => queens |= square_mask(7 - i, 7 - j),
                            PieceKind::King => kings |= square_mask(7 - i, 7 - j),
                        }

                        j += 1;
                    }
                }
            }
        }

        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!(
                "Invalid FEN, expected 'w' or 'b' for side to move, found {}",
                parts[1]
            ),
        };

        let can_white_castle_queenside = parts[2].contains("Q");
        let can_white_castle_kingside = parts[2].contains("K");
        let can_black_castle_queenside = parts[2].contains("q");
        let can_black_castle_kingside = parts[2].contains("k");

        let en_passant_target_square = if parts[3] == "-" {
            None
        } else {
            let col = (b'h' - parts[3].as_bytes()[0]) as usize;
            let row = (parts[3].as_bytes()[1] - b'1') as usize;
            if col > 7 || (row != 2 && row != 5) {
                panic!(
                    "Invalid FEN, en passant target square {} is not valid",
                    parts[3]
                )
            }
            Some((row, col))
        };

        Ok(Self {
            white_pieces,
            black_pieces,
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
            side_to_move,
            can_white_castle_kingside,
            can_white_castle_queenside,
            can_black_castle_kingside,
            can_black_castle_queenside,
            en_passant_target_square,
        })
    }
}

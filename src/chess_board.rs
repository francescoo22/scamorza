use crate::chess_move::Move;
use crate::chess_piece::*;
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

pub type SquareIndex = u8;
pub type UnsafeSquareIndex = i8;
pub type SquareIndexDelta = (i8, i8);

pub fn apply_delta(index: SquareIndex, delta: SquareIndexDelta) -> UnsafeSquareIndex {
    let i8index = index as i8;
    if i8index / 8 + delta.0 >= 0
        && i8index / 8 + delta.0 < 8
        && i8index % 8 + delta.1 >= 0
        && i8index % 8 + delta.1 < 8
    {
        i8index + delta.0 * 8 + delta.1
    } else {
        -1
    }
}

pub fn apply_delta_with_dist(index: SquareIndex, delta: SquareIndexDelta, dist: u8) -> UnsafeSquareIndex {
    apply_delta(index, (delta.0 * dist as i8, delta.1 * dist as i8))
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
    pub white_pieces: BitBoard,
    pub black_pieces: BitBoard,

    pub pawns: BitBoard,
    pub knights: BitBoard,
    pub bishops: BitBoard,
    pub rooks: BitBoard,
    pub queens: BitBoard,
    pub kings: BitBoard,

    /// bit 0: can_white_castle_kingside.
    /// bit 1: can_white_castle_queenside.
    /// bit 2: can_black_castle_kingside.
    /// bit 3: can_black_castle_queenside.
    /// bit 4: current_turn. (1 -> white, 0 -> black)
    /// bit 5-10: en_passant_target_square. (111111 -> None)
    status: BitBoard,
}

fn within_bounds(index: UnsafeSquareIndex) -> Option<SquareIndex> {
    if index >= 0 && index < 64 {
        Some(index as SquareIndex)
    } else {
        None
    }
}

impl fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for index in 0..64 {
            write!(f, " {}", self.at(index))?;
            if index % 8 == 7 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl ChessBoard {
    pub(crate) fn for_each_piece<F>(&self, mut block: F)
    where
        F: FnMut(SquareIndex, &Piece),
    {
        for index in 0..64 {
            match self.at(index) {
                Square::Empty => {}
                Square::Occupied(piece) => {
                    block(index, &piece);
                }
            }
        }
    }

    pub(crate) fn within_bounds_and_empty(&self, index: UnsafeSquareIndex) -> Option<SquareIndex> {
        match within_bounds(index) {
            None => None,
            Some(index) => match self.at(index) {
                Square::Empty => Some(index),
                Square::Occupied(_) => None,
            },
        }
    }

    fn occupied_by_opponent(&self, index: SquareIndex, color: &Color) -> Option<SquareIndex> {
        match self.at(index) {
            Square::Empty => None,
            Square::Occupied(piece) => {
                if piece.color != *color {
                    Some(index)
                } else {
                    None
                }
            }
        }
    }

    pub(crate) fn within_bounds_and_occupied_by_opponent(
        &self,
        index: UnsafeSquareIndex,
        color: &Color,
    ) -> Option<SquareIndex> {
        match within_bounds(index) {
            None => None,
            Some(index) => self.occupied_by_opponent(index, color),
        }
    }

    pub(crate) fn within_bounds_and_pawn_take_target(
        &self,
        index: UnsafeSquareIndex,
        color: &Color,
    ) -> Option<SquareIndex> {
        match within_bounds(index) {
            None => None,
            Some(index) => {
                if self.en_passant_target_square() == Some(index) {
                    Some(index)
                } else {
                    self.occupied_by_opponent(index, color)
                }
            }
        }
    }

    pub fn at(&self, index: SquareIndex) -> Square {
        let square_mask = 1 << index;

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

    pub fn at_2(&self, row: u8, col: u8) -> Square {
        debug_assert!(row < 8 && col < 8);
        self.at(row * 8 + col)
    }

    pub fn maybe_piece_at(&self, index: UnsafeSquareIndex) -> Option<Piece> {
        match within_bounds(index) {
            None => None,
            Some(index) => match self.at(index) {
                Square::Empty => None,
                Square::Occupied(piece) => Some(piece),
            },
        }
    }

    pub fn set_at(&mut self, index: SquareIndex, square: Square) {
        let square_mask = 1 << index;
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

    pub fn set_at_2(&mut self, row: u8, col: u8, square: Square) {
        debug_assert!(row < 8 && col < 8);
        self.set_at(row * 8 + col, square);
    }

    pub fn piece_at_source_or_panic(self, mov: &Move) -> Piece {
        match self.at(mov.from) {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        }
    }

    pub fn contains_piece_at(self, index: UnsafeSquareIndex, piece_to_find: Piece) -> bool {
        match self.maybe_piece_at(index) {
            None => false,
            Some(piece) => piece == piece_to_find,
        }
    }

    pub fn contains_piece_in_any_direction(
        self,
        index: SquareIndex,
        piece_to_find: Piece,
        directions: &[SquareIndexDelta],
    ) -> bool {
        directions
            .iter()
            .any(|delta| self.contains_piece_at(apply_delta(index, *delta), piece_to_find))
    }

    pub fn can_white_castle_kingside(&self) -> bool {
        (self.status & WHITE_KINGSIDE_CASTLE_MASK) != 0
    }

    pub fn set_white_castle_kingside(&mut self, can_castle: bool) {
        if can_castle {
            self.status |= WHITE_KINGSIDE_CASTLE_MASK;
        } else {
            self.status &= !WHITE_KINGSIDE_CASTLE_MASK;
        }
    }
    pub fn can_white_castle_queenside(&self) -> bool {
        (self.status & WHITE_QUEENSIDE_CASTLE_MASK) != 0
    }

    pub fn set_white_castle_queenside(&mut self, can_castle: bool) {
        if can_castle {
            self.status |= WHITE_QUEENSIDE_CASTLE_MASK;
        } else {
            self.status &= !WHITE_QUEENSIDE_CASTLE_MASK;
        }
    }

    pub fn can_black_castle_kingside(&self) -> bool {
        (self.status & BLACK_KINGSIDE_CASTLE_MASK) != 0
    }

    pub fn set_black_castle_kingside(&mut self, can_castle: bool) {
        if can_castle {
            self.status |= BLACK_KINGSIDE_CASTLE_MASK;
        } else {
            self.status &= !BLACK_KINGSIDE_CASTLE_MASK;
        }
    }

    pub fn can_black_castle_queenside(&self) -> bool {
        (self.status & BLACK_QUEENSIDE_CASTLE_MASK) != 0
    }

    pub fn set_black_castle_queenside(&mut self, can_castle: bool) {
        if can_castle {
            self.status |= BLACK_QUEENSIDE_CASTLE_MASK;
        } else {
            self.status &= !BLACK_QUEENSIDE_CASTLE_MASK;
        }
    }

    pub fn current_turn(&self) -> Color {
        if self.status & CURRENT_TURN_MASK != 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn next_turn(&mut self) {
        self.status ^= CURRENT_TURN_MASK;
    }

    pub fn en_passant_target_square(&self) -> Option<SquareIndex> {
        if self.status & EN_PASSANT_MASK == EN_PASSANT_MASK {
            None
        } else {
            let square_index = (self.status & EN_PASSANT_MASK) >> 5;
            debug_assert!(square_index < 64);
            Some(square_index as SquareIndex)
        }
    }

    pub fn set_en_passant_target_square(&mut self, square: Option<SquareIndex>) {
        self.status &= !EN_PASSANT_MASK;
        match square {
            None => {
                self.status |= EN_PASSANT_MASK;
            }
            Some(index) => {
                self.status |= (index as BitBoard) << 5;
            }
        }
        // TODO: reintroduce this assert
        // debug_assert_eq!(self.en_passant_target_square(), square);
    }
}

const WHITE_KINGSIDE_CASTLE_MASK: BitBoard = 1 << 0;
const WHITE_QUEENSIDE_CASTLE_MASK: BitBoard = 1 << 1;
const BLACK_KINGSIDE_CASTLE_MASK: BitBoard = 1 << 2;
const BLACK_QUEENSIDE_CASTLE_MASK: BitBoard = 1 << 3;
const CURRENT_TURN_MASK: BitBoard = 1 << 4;
pub const EN_PASSANT_MASK: BitBoard = 63 << 5;

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
            status: 0x7FF,
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
        let mut status = 0;

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
                            'P' => WHITE_PAWN,
                            'N' => WHITE_KNIGHT,
                            'B' => WHITE_BISHOP,
                            'R' => WHITE_ROOK,
                            'Q' => WHITE_QUEEN,
                            'K' => WHITE_KING,
                            'p' => BLACK_PAWN,
                            'n' => BLACK_KNIGHT,
                            'b' => BLACK_BISHOP,
                            'r' => BLACK_ROOK,
                            'q' => BLACK_QUEEN,
                            'k' => BLACK_KING,
                            _ => panic!(
                                "Invalid FEN character, expected a piece representation (PNBRQKpnbrqk), found '{}'",
                                c
                            ),
                        };

                        let square_mask = 1 << ((7 - i) * 8 + 7 - j);
                        match piece.color {
                            Color::White => white_pieces |= square_mask,
                            Color::Black => black_pieces |= square_mask,
                        }

                        match piece.kind {
                            PieceKind::Pawn => pawns |= square_mask,
                            PieceKind::Knight => knights |= square_mask,
                            PieceKind::Bishop => bishops |= square_mask,
                            PieceKind::Rook => rooks |= square_mask,
                            PieceKind::Queen => queens |= square_mask,
                            PieceKind::King => kings |= square_mask,
                        }

                        j += 1;
                    }
                }
            }
        }

        match parts[1] {
            "w" => {
                status |= CURRENT_TURN_MASK;
            }
            "b" => {}
            _ => panic!(
                "Invalid FEN, expected 'w' or 'b' for side to move, found {}",
                parts[1]
            ),
        };

        if parts[2].contains("Q") {
            status |= WHITE_QUEENSIDE_CASTLE_MASK;
        }
        if parts[2].contains("K") {
            status |= WHITE_KINGSIDE_CASTLE_MASK;
        }
        if parts[2].contains("q") {
            status |= BLACK_QUEENSIDE_CASTLE_MASK;
        }
        if parts[2].contains("k") {
            status |= BLACK_KINGSIDE_CASTLE_MASK;
        }

        // TODO: this is not tested in perft
        if parts[3] != "-" {
            let col = (b'h' - parts[3].as_bytes()[0]) as usize;
            let row = (parts[3].as_bytes()[1] - b'1') as usize;
            if col > 7 || (row != 2 && row != 5) {
                panic!(
                    "Invalid FEN, en passant target square {} is not valid",
                    parts[3]
                )
            }
            status |= ((row * 8 + col) as BitBoard) << 5;
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
            status,
        })
    }
}

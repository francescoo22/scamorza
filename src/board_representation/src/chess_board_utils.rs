use crate::chess_board::{ChessBoard, Square, SquareIndex, UnsafeSquareIndex};
use crate::chess_piece::{Color, Piece};

pub fn within_bounds(index: UnsafeSquareIndex) -> Option<SquareIndex> {
    if index >= 0 && index < 64 {
        Some(index as SquareIndex)
    } else {
        None
    }
}

impl ChessBoard {
    pub fn for_each_piece<F>(&self, mut block: F)
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

    pub fn within_bounds_and_empty(&self, index: UnsafeSquareIndex) -> Option<SquareIndex> {
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

    pub fn within_bounds_and_occupied_by_opponent(
        &self,
        index: UnsafeSquareIndex,
        color: &Color,
    ) -> Option<SquareIndex> {
        match within_bounds(index) {
            None => None,
            Some(index) => self.occupied_by_opponent(index, color),
        }
    }

    pub fn within_bounds_and_pawn_take_target(
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

    pub fn maybe_piece_at(&self, index: UnsafeSquareIndex) -> Option<Piece> {
        match within_bounds(index) {
            None => None,
            Some(index) => match self.at(index) {
                Square::Empty => None,
                Square::Occupied(piece) => Some(piece),
            },
        }
    }

    pub fn piece_at_source_or_panic(self, index: SquareIndex) -> Piece {
        match self.at(index) {
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

    pub fn find_king(&self, color: Color) -> SquareIndex {
        let king_bit_board = match color {
            Color::White => self.white_pieces & self.kings,
            Color::Black => self.black_pieces & self.kings,
        };
        king_bit_board.trailing_zeros() as SquareIndex
    }
}

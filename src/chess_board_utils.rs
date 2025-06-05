use crate::chess_board::{ChessBoard, Square, SquareIndex, SquareIndexDelta, UnsafeSquareIndex};
use crate::chess_move::Move;
use crate::chess_piece::{Color, Piece};

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

pub fn apply_delta_with_dist(
    index: SquareIndex,
    delta: SquareIndexDelta,
    dist: u8,
) -> UnsafeSquareIndex {
    apply_delta(index, (delta.0 * dist as i8, delta.1 * dist as i8))
}

pub fn within_bounds(index: UnsafeSquareIndex) -> Option<SquareIndex> {
    if index >= 0 && index < 64 {
        Some(index as SquareIndex)
    } else {
        None
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

    pub fn maybe_piece_at(&self, index: UnsafeSquareIndex) -> Option<Piece> {
        match within_bounds(index) {
            None => None,
            Some(index) => match self.at(index) {
                Square::Empty => None,
                Square::Occupied(piece) => Some(piece),
            },
        }
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
}
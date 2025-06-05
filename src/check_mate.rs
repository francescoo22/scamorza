use crate::chess_board::*;
use crate::chess_piece::*;

impl ChessBoard {
    fn find_king(self, color: Color) -> SquareIndex {
        let king_bit_board = match color {
            Color::White => self.white_pieces & self.kings,
            Color::Black => self.black_pieces & self.kings,
        };
        king_bit_board.trailing_zeros() as SquareIndex
    }

    fn is_square_checked_by_knight(self, index: SquareIndex, color: Color) -> bool {
        let piece_to_find = Piece {
            kind: PieceKind::Knight,
            color: !color,
        };
        self.contains_piece_in_any_direction(index, piece_to_find, &KNIGHT_DIRECTIONS)
    }

    fn is_square_checked_by_slider(self, index: SquareIndex, color: Color) -> bool {
        for delta in KING_DIRECTIONS {
            let mut dist = 1;
            while self.within_bounds_and_empty(apply_delta_with_dist(index, delta, dist)).is_some() {
                dist += 1;
            }
            match self.maybe_piece_at(apply_delta_with_dist(index, delta, dist)) {
                None => {}
                Some(piece) => {
                    if piece.color != color {
                        match piece.kind {
                            PieceKind::Rook => {
                                if delta.0 * delta.1 == 0 {
                                    return true;
                                }
                            }
                            PieceKind::Bishop => {
                                if delta.0 * delta.1 != 0 {
                                    return true;
                                }
                            }
                            PieceKind::Queen => {
                                return true;
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        false
    }

    fn is_square_checked_by_king(self, index: SquareIndex, color: Color) -> bool {
        let piece_to_find = Piece {
            kind: PieceKind::King,
            color: !color,
        };
        self.contains_piece_in_any_direction(index, piece_to_find, &KING_DIRECTIONS)
    }

    pub fn is_square_checked_by_pawn(self, index: SquareIndex, color: Color) -> bool {
        let di = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        for dj in [1, -1] {
            match self.maybe_piece_at(apply_delta(index, (di, dj))) {
                None => {}
                Some(piece) => {
                    if piece.kind == PieceKind::Pawn && color != piece.color {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn is_square_checked(self, index: SquareIndex, color: Color) -> bool {
        self.is_square_checked_by_slider(index, color)
            || self.is_square_checked_by_knight(index, color)
            || self.is_square_checked_by_king(index, color)
            || self.is_square_checked_by_pawn(index, color)
    }

    pub fn is_king_checked(self, color: Color) -> bool {
        self.is_square_checked(self.find_king(color), color)
    }

    pub fn king_cannot_move(self, color: Color) -> bool {
        self.is_king_checked(color) && self.all_valid_moves(color).is_empty()
    }

    // todo: implement properly
    pub fn is_stalemate(self) -> bool {
        for index in 0..64 {
            match self.at(index) {
                    Square::Occupied(piece) => {
                        if piece.kind != PieceKind::King {
                            return false;
                        }
                    }
                    _ => {}
                }
        }
        true
    }
}

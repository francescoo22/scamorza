use crate::chess_board::*;
use crate::chess_piece::*;

impl ChessBoard {
    fn find_king(self, color: Color) -> (i32, i32) {
        let king_bit_board = match color {
            Color::White => self.white_pieces & self.kings,
            Color::Black => self.black_pieces & self.kings,
        };
        let king_position = king_bit_board.trailing_zeros();
        ((king_position / 8) as i32, (king_position % 8) as i32)
    }

    fn is_square_checked_by_knight(self, i: i32, j: i32, color: Color) -> bool {
        let piece_to_find = Piece {
            kind: PieceKind::Knight,
            color: !color,
        };
        self.contains_piece_in_any_direction(i, j, piece_to_find, knight_directions().to_vec())
    }

    fn is_square_checked_by_slider(self, i: i32, j: i32, color: Color) -> bool {
        for (di, dj) in king_directions() {
            let mut dist = 1;
            while self.within_bounds_and_empty(i + di * dist, j + dj * dist) {
                dist += 1;
            }
            match self.maybe_piece_at(i + di * dist, j + dj * dist) {
                None => {}
                Some(piece) => {
                    if piece.color != color {
                        match piece.kind {
                            PieceKind::Rook => {
                                if di * dj == 0 {
                                    return true;
                                }
                            }
                            PieceKind::Bishop => {
                                if di * dj != 0 {
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

    fn is_square_checked_by_king(self, i: i32, j: i32, color: Color) -> bool {
        let piece_to_find = Piece {
            kind: PieceKind::King,
            color: !color,
        };
        self.contains_piece_in_any_direction(i, j, piece_to_find, king_directions().to_vec())
    }

    pub fn is_square_checked_by_pawn(self, i: i32, j: i32, color: Color) -> bool {
        let di = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        for dj in [1, -1] {
            match self.maybe_piece_at(i + di, j + dj) {
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

    pub fn is_square_checked(self, i: i32, j: i32, color: Color) -> bool {
        self.is_square_checked_by_slider(i, j, color)
            || self.is_square_checked_by_knight(i, j, color)
            || self.is_square_checked_by_king(i, j, color)
            || self.is_square_checked_by_pawn(i, j, color)
    }

    pub fn is_king_checked(self, color: Color) -> bool {
        let (i, j) = self.find_king(color);
        self.is_square_checked(i, j, color)
    }

    pub fn king_cannot_move(self, color: Color) -> bool {
        self.is_king_checked(color) && self.all_valid_moves(color).is_empty()
    }

    // todo: implement properly
    pub fn is_stalemate(self) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                match self.at(i, j) {
                    Square::Occupied(piece) => {
                        if piece.kind != PieceKind::King {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
        true
    }
}

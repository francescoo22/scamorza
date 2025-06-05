use crate::chess_board::{
    apply_delta, apply_delta_with_dist, ChessBoard, Square, SquareIndex, SquareIndexDelta,
};
use crate::chess_move::Move;
use crate::chess_piece::{Color, Piece, PieceKind, BISHOP_DIRECTIONS, KING_DIRECTIONS, KNIGHT_DIRECTIONS, PROMOTABLE_KINDS, ROOK_DIRECTIONS};

// TODO: use builder for valid moves creation
impl ChessBoard {
    fn slider_valid_moves(
        &self,
        index: SquareIndex,
        color: &Color,
        directions: &[SquareIndexDelta],
    ) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for delta in directions {
            let mut dist = 1;

            loop {
                let next_position = apply_delta_with_dist(index, *delta, dist);
                match self.within_bounds_and_empty(next_position) {
                    Some(to_index) => {
                        moves.push(Move::base_move(index, to_index));
                        dist += 1
                    }
                    None => {
                        match self.within_bounds_and_occupied_by_opponent(next_position, color) {
                            None => {}
                            Some(to_index) => moves.push(Move::base_move(index, to_index)),
                        }
                        break;
                    }
                }
            }
        }
        moves
    }

    fn leaper_valid_moves(
        &self,
        index: SquareIndex,
        color: &Color,
        directions: &[SquareIndexDelta],
    ) -> Vec<Move> {
        let mut moves = Vec::new();

        for delta in directions {
            let to_index = apply_delta(index, *delta);
            if let Some(to_index) = self.within_bounds_and_empty(to_index) {
                moves.push(Move::base_move(index, to_index));
            } else if let Some(to_index) =
                self.within_bounds_and_occupied_by_opponent(to_index, color)
            {
                moves.push(Move::base_move(index, to_index));
            }
        }
        moves
    }

    fn knight_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        self.leaper_valid_moves(index, color, &KNIGHT_DIRECTIONS)
    }

    fn king_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        self.leaper_valid_moves(index, color, &KING_DIRECTIONS)
    }

    fn rook_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(index, color, &ROOK_DIRECTIONS)
    }

    fn bishop_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(index, color, &BISHOP_DIRECTIONS)
    }

    fn queen_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(index, color, &KING_DIRECTIONS)
    }

    fn is_promotion_row(index: SquareIndex, color: &Color) -> bool {
        // todo: it is a bit scary that this is not covered properly
        match color {
            Color::White => index > 55,
            Color::Black => index < 8,
        }
    }

    fn maybe_promotion_moves(from: SquareIndex, to: SquareIndex, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        if !Self::is_promotion_row(to, color) {
            moves.push(Move::base_move(from, to));
        } else {
            PROMOTABLE_KINDS.into_iter().for_each(|promoted_piece| {
                moves.push(Move {
                    from,
                    to,
                    promoted_piece_kind: Some(promoted_piece),
                })
            })
        }
        moves
    }

    fn is_initial_pawn_raw(index: SquareIndex, color: &Color) -> bool {
        match color {
            Color::White => index >= 8 && index < 16,
            Color::Black => index >= 48 && index < 56,
        }
    }

    fn pawn_valid_moves(&self, index: SquareIndex, color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let to_index_single = apply_delta(index, (dir, 0));
        if let Some(to_index_single) = self.within_bounds_and_empty(to_index_single) {
            moves.extend(Self::maybe_promotion_moves(index, to_index_single, color));
            if Self::is_initial_pawn_raw(index, color) {
                let to_index_double = apply_delta(index, (dir * 2, 0));
                if let Some(to_index_double) = self.within_bounds_and_empty(to_index_double) {
                    moves.push(Move::base_move(index, to_index_double));
                }
            }
        }

        let to_index_take_right = apply_delta(index, (dir, 1));
        if let Some(to_index_take_right) =
            self.within_bounds_and_pawn_take_target(to_index_take_right, color)
        {
            moves.extend(Self::maybe_promotion_moves(
                index,
                to_index_take_right,
                color,
            ));
        }
        let to_index_take_left = apply_delta(index, (dir, -1));
        if let Some(to_index_take_left) =
            self.within_bounds_and_pawn_take_target(to_index_take_left, color)
        {
            moves.extend(Self::maybe_promotion_moves(
                index,
                to_index_take_left,
                color,
            ));
        }
        moves
    }

    fn is_kingside_castle_possible(&self, color: &Color) -> bool {
        if *color == Color::White && !self.can_white_castle_kingside() {
            return false;
        }

        if *color == Color::Black && !self.can_black_castle_kingside() {
            return false;
        }

        let empty_square_indexes = match color {
            Color::White => [1, 2],
            Color::Black => [57, 58],
        };
        for index in empty_square_indexes {
            if self.at(index) != Square::Empty {
                return false;
            }
        }

        let non_checked_square_indexes = match color {
            Color::White => [1, 2, 3],
            Color::Black => [57, 58, 59],
        };
        for index in non_checked_square_indexes {
            if self.is_square_checked(index, *color) {
                return false;
            }
        }

        true
    }

    fn is_queenside_castle_possible(&self, color: &Color) -> bool {
        if *color == Color::White && !self.can_white_castle_queenside() {
            return false;
        }

        if *color == Color::Black && !self.can_black_castle_queenside() {
            return false;
        }

        let empty_square_indexes = match color {
            Color::White => [4, 5, 6],
            Color::Black => [60, 61, 62],
        };
        for index in empty_square_indexes {
            if self.at(index) != Square::Empty {
                return false;
            }
        }

        let non_checked_square_indexes = match color {
            Color::White => [3, 4, 5],
            Color::Black => [59, 60, 61],
        };
        for index in non_checked_square_indexes {
            if self.is_square_checked(index, *color) {
                return false;
            }
        }

        true
    }

    fn piece_valid_moves(&self, index: SquareIndex, piece: &Piece) -> Vec<Move> {
        match piece.kind {
            PieceKind::Pawn => self.pawn_valid_moves(index, &piece.color),
            PieceKind::Rook => self.rook_valid_moves(index, &piece.color),
            PieceKind::Bishop => self.bishop_valid_moves(index, &piece.color),
            PieceKind::Queen => self.queen_valid_moves(index, &piece.color),
            PieceKind::Knight => self.knight_valid_moves(index, &piece.color),
            PieceKind::King => self.king_valid_moves(index, &piece.color),
        }
    }

    pub fn all_possible_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        self.for_each_piece(|index, piece| {
            if piece.color == color {
                let valid_moves = self.piece_valid_moves(index, piece);
                moves.extend(valid_moves);
            }
        });
        moves
    }

    fn filter_king_going_under_check(&self, moves: Vec<Move>) -> Vec<Move> {
        moves
            .iter()
            .filter(|mov| {
                let piece = self.piece_at_source_or_panic(mov);
                let mut board_after_move = self.clone();
                board_after_move.move_piece(&mov);
                !board_after_move.is_king_checked(piece.color)
            })
            .copied()
            .collect()
    }

    pub fn all_valid_moves(&self, color: Color) -> Vec<Move> {
        let mut moves = self.filter_king_going_under_check(self.all_possible_moves(color));
        if self.is_kingside_castle_possible(&color) {
            let (from, to) = match color {
                Color::White => (3, 1),
                Color::Black => (59, 57),
            };
            moves.push(Move::base_move(from, to))
        }
        if self.is_queenside_castle_possible(&color) {
            let (from, to) = match color {
                Color::White => (3, 5),
                Color::Black => (59, 61),
            };
            moves.push(Move::base_move(from, to))
        }
        moves
    }
}

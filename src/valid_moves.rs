use crate::chess_board::{ChessBoard, Square};
use crate::chess_move::Move;
use crate::chess_piece::{bishop_directions, king_directions, knight_directions, rook_directions, Color, Piece, PieceKind};

// TODO: use builder for valid moves creation
impl ChessBoard {
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
                let from = (i as usize, j as usize);
                let to = ((i + cur_dist * dy) as usize, (j + cur_dist * dx) as usize);
                moves.push(Move::base_move(from, to));
            }
            if self.within_bounds_and_occupied_by_opponent(i + dist * dy, j + dist * dx, color) {
                let from = (i as usize, j as usize);
                let to = ((i + dist * dy) as usize, (j + dist * dx) as usize);
                moves.push(Move::base_move(from, to));
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
                let from = (i as usize, j as usize);
                let to = ((i + dy) as usize, (j + dx) as usize);
                moves.push(Move::base_move(from, to));
            }
        }
        moves
    }

    fn knight_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        self.leaper_valid_moves(i, j, color, &knight_directions().to_vec())
    }

    fn king_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        self.leaper_valid_moves(i, j, color, &king_directions().to_vec())
    }

    fn rook_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(i, j, color, &rook_directions().to_vec())
    }

    fn bishop_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(i, j, color, &bishop_directions().to_vec())
    }

    fn queen_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
        self.slider_valid_moves(i, j, color, &king_directions().to_vec())
    }

    fn maybe_promotion_moves(from: (usize, usize), to: (usize, usize), color: &Color) -> Vec<Move> {
        let mut moves = Vec::new();
        let dir = match color {
            Color::White => 1,
            Color::Black => -1,
        };
        let initial_row = match color {
            Color::White => 1,
            Color::Black => 6,
        };
        if to.0 != (initial_row + dir * 6) as usize {
            moves.push(Move::base_move(from, to));
        } else {
            let promotable_kinds = [
                PieceKind::Queen,
                PieceKind::Rook,
                PieceKind::Bishop,
                PieceKind::Knight,
            ]
                .map(|it| Some(it));

            promotable_kinds.into_iter().for_each(|promoted_piece| {
                moves.push(Move {
                    from,
                    to,
                    promoted_piece_kind: promoted_piece,
                })
            })
        }
        moves
    }

    fn pawn_valid_moves(&self, i: i32, j: i32, color: &Color) -> Vec<Move> {
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
            let from = (i as usize, j as usize);
            let to = ((i + dir) as usize, j as usize);
            moves.extend(Self::maybe_promotion_moves(from, to, color));

            if self.within_bounds_and_empty(i + 2 * dir, j) && i == initial_row {
                let from = (i as usize, j as usize);
                let to = ((i + 2 * dir) as usize, j as usize);
                moves.push(Move::base_move(from, to));
            }
        }
        if self.within_bounds_and_pawn_take_target(i + dir, j + 1, color) {
            let from = (i as usize, j as usize);
            let to = ((i + dir) as usize, (j + 1) as usize);
            moves.extend(Self::maybe_promotion_moves(from, to, color));
        }
        if self.within_bounds_and_pawn_take_target(i + dir, j - 1, color) {
            let from = (i as usize, j as usize);
            let to = ((i + dir) as usize, (j - 1) as usize);
            moves.extend(Self::maybe_promotion_moves(from, to, color));
        }
        moves
    }

    fn is_kingside_castle_possible(&self, color: &Color) -> bool {
        if *color == Color::White && !self.can_white_castle_kingside {
            return false;
        }

        if *color == Color::Black && !self.can_black_castle_kingside {
            return false;
        }

        let row = match color {
            Color::White => 0,
            Color::Black => 7,
        };

        if self.at(row, 1) != Square::Empty {
            return false;
        }

        if self.at(row, 2) != Square::Empty {
            return false;
        }

        for col in 1..=3 {
            if self.is_square_checked(row as i32, col, *color) {
                return false;
            }
        }

        true
    }

    fn is_queenside_castle_possible(&self, color: &Color) -> bool {
        if *color == Color::White && !self.can_white_castle_queenside {
            return false;
        }

        if *color == Color::Black && !self.can_black_castle_queenside {
            return false;
        }

        let row = match color {
            Color::White => 0,
            Color::Black => 7,
        };

        if self.at(row, 4) != Square::Empty {
            return false;
        }

        if self.at(row, 5) != Square::Empty {
            return false;
        }

        if self.at(row, 6) != Square::Empty {
            return false;
        }

        for col in 3..=5 {
            if self.is_square_checked(row as i32, col, *color) {
                return false;
            }
        }

        true
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
                let piece = self.piece_at_source_or_panic(mov);
                let mut board_after_move = self.clone();
                board_after_move.move_piece_uci(&mov.to_uci_string());
                !board_after_move.is_king_checked(piece.color)
            })
            .copied()
            .collect()
    }

    pub fn all_valid_moves(&self, color: Color) -> Vec<Move> {
        let moves = self.all_possible_moves(color);
        let mut moves = self.filter_king_going_under_check(moves);
        if self.is_kingside_castle_possible(&color) {
            let row = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            moves.push(Move::base_move((row, 3), (row, 1)))
        }
        if self.is_queenside_castle_possible(&color) {
            let row = match color {
                Color::White => 0,
                Color::Black => 7,
            };
            moves.push(Move::base_move((row, 3), (row, 5)))
        }
        moves
    }
}

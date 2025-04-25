use crate::chess_board::{ChessBoard, Square};
use crate::chess_piece::{Piece, PieceKind};
use regex::Regex;
use std::ops::Not;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Move {
    pub fn from_uci_string(s: &str) -> Self {
        let re = Regex::new(r"^[a-h][1-8][a-h][1-8]$").unwrap();
        if !re.is_match(s) {
            panic!("Invalid UCI string");
        }

        let bytes = s.as_bytes();
        let from_col = (b'h' - bytes[0]) as usize;
        let from_row = (bytes[1] - b'1') as usize;
        let to_col = (b'h' - bytes[2]) as usize;
        let to_row = (bytes[3] - b'1') as usize;
        Move {
            from: (from_row, from_col),
            to: (to_row, to_col),
        }
    }

    pub fn to_uci_string(&self) -> String {
        let from_row = char::from(self.from.0 as u8 + b'1');
        let from_col = char::from(b'h' - self.from.1 as u8);
        let to_row = char::from(self.to.0 as u8 + b'1');
        let to_col = char::from(b'h' - self.to.1 as u8);
        format!("{}{}{}{}", from_col, from_row, to_col, to_row)
    }
}

impl Not for Move {
    type Output = Move;

    fn not(self) -> Self {
        Move {
            from: self.to,
            to: self.from,
        }
    }
}

impl ChessBoard {
    fn castle_invalidation(&mut self, mov: &Move) {
        match mov.from {
            (0, 0) => self.can_white_castle_kingside = false,
            (0, 7) => self.can_white_castle_queenside = false,
            (7, 0) => self.can_black_castle_kingside = false,
            (7, 7) => self.can_black_castle_queenside = false,
            (0, 3) => {
                self.can_white_castle_kingside = false;
                self.can_white_castle_queenside = false;
            }
            (7, 3) => {
                self.can_black_castle_kingside = false;
                self.can_black_castle_queenside = false;
            }
            _ => {}
        }
    }

    fn move_rook_if_castle(&mut self, mov: &Move, moving_piece: &Piece) {
        if moving_piece.kind != PieceKind::King {
            return;
        }

        if mov.from.1 != 3 {
            return;
        }

        if mov.to.1 == 1 {
            self.set_at(mov.from.0, 2, self.at(mov.from.0, 0));
            self.set_at(mov.from.0, 0, Square::Empty);
        }

        if mov.to.1 == 5 {
            self.set_at(mov.from.0, 4, self.at(mov.from.0, 7));
            self.set_at(mov.from.0, 7, Square::Empty);
        }
    }

    pub fn move_piece(&mut self, mov: &Move) {
        let moving_piece = match self.at(mov.from.0, mov.from.1) {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        };

        self.castle_invalidation(mov);

        let promoted_piece = if mov.to.0 == 7 && moving_piece == Piece::white_pawn() {
            Piece::white_queen()
        } else if mov.to.0 == 0 && moving_piece == Piece::black_pawn() {
            Piece::black_queen()
        } else {
            moving_piece
        };

        self.move_rook_if_castle(mov, &moving_piece);

        self.set_at(mov.from.0, mov.from.1, Square::Empty);
        self.set_at(mov.to.0, mov.to.1, Square::Occupied(promoted_piece));
    }

    pub fn move_piece_uci(&mut self, uci: &str) {
        let mov = Move::from_uci_string(uci);
        self.move_piece(&mov);
    }
}

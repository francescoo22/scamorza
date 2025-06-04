use crate::chess_board::{ChessBoard, Square, SquareIndex};
use crate::chess_piece::{Color, Piece, PieceKind};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: SquareIndex,
    pub to: SquareIndex,
    pub promoted_piece_kind: Option<PieceKind>,
}

impl Move {
    pub fn base_move(from: SquareIndex, to: SquareIndex) -> Self {
        Self {
            from,
            to,
            promoted_piece_kind: None,
        }
    }
    pub fn from_uci_string(s: &str) -> Self {
        let re = Regex::new(r"^[a-h][1-8][a-h][1-8][qrbn]?$").unwrap();
        if !re.is_match(s) {
            panic!("Invalid UCI string");
        }

        let bytes = s.as_bytes();
        let from_col = (bytes[0] - b'a') as usize;
        let from_row = (bytes[1] - b'1') as usize;
        let to_col = (bytes[2] - b'a') as usize;
        let to_row = (bytes[3] - b'1') as usize;
        let promoted_piece = if bytes.len() == 4 {
            None
        } else {
            let piece_kind = match bytes[4] {
                b'q' => PieceKind::Queen,
                b'r' => PieceKind::Rook,
                b'b' => PieceKind::Bishop,
                b'n' => PieceKind::Knight,
                _ => panic!("Invalid UCI string, unknown promoted piece kind"),
            };
            Some(piece_kind)
        };
        Move {
            from: (from_row * 8 + from_col) as u8,
            to: (to_row * 8 + to_col) as u8,
            promoted_piece_kind: promoted_piece,
        }
    }

    pub fn to_uci_string(&self) -> String {
        let from_row = char::from(self.from / 8 + b'1');
        let from_col = char::from(self.from % 8 + b'a');
        let to_row = char::from(self.to / 8 + b'1');
        let to_col = char::from(self.to % 8 + b'a');
        match self.promoted_piece_kind {
            None => format!("{}{}{}{}", from_col, from_row, to_col, to_row),
            Some(piece_kind) => {
                let piece_kind_char = match piece_kind {
                    PieceKind::Rook => 'r',
                    PieceKind::Knight => 'n',
                    PieceKind::Bishop => 'b',
                    PieceKind::Queen => 'q',
                    _ => panic!(
                        "Invalid move, promoted piece cannot be of kind {:?}",
                        piece_kind
                    ),
                };
                format!(
                    "{}{}{}{}{}",
                    from_col, from_row, to_col, to_row, piece_kind_char
                )
            }
        }
    }
}

impl ChessBoard {
    fn castle_invalidation(&mut self, mov: &Move) {
        match mov.from {
            0 => self.set_white_castle_kingside(false),
            7 => self.set_white_castle_queenside(false),
            56 => self.set_black_castle_kingside(false),
            63 => self.set_black_castle_queenside(false),
            3 => {
                self.set_white_castle_kingside(false);
                self.set_white_castle_queenside(false);
            }
            59 => {
                self.set_black_castle_kingside(false);
                self.set_black_castle_queenside(false);
            }
            _ => {}
        }

        match mov.to {
            0 => self.set_white_castle_kingside(false),
            7 => self.set_white_castle_queenside(false),
            56 => self.set_black_castle_kingside(false),
            63 => self.set_black_castle_queenside(false),
            _ => {}
        }
    }

    fn move_rook_if_castle(&mut self, mov: &Move, moving_piece: &Piece) {
        if moving_piece.kind != PieceKind::King {
            return;
        }

        if mov.from == 3 {
            if mov.to == 1 {
                self.set_at(2, Square::Occupied(Piece::white_rook()));
                self.set_at(0, Square::Empty)
            } else if mov.to == 5 {
                self.set_at(4, Square::Occupied(Piece::white_rook()));
                self.set_at(7, Square::Empty)
            }
        } else if mov.from == 59 {
            if mov.to == 57 {
                self.set_at(58, Square::Occupied(Piece::black_rook()));
                self.set_at(56, Square::Empty)
            } else if mov.to == 61 {
                self.set_at(60, Square::Occupied(Piece::black_rook()));
                self.set_at(63, Square::Empty)
            }
        }
    }

    fn remove_piece_after_en_passant(&mut self, mov: &Move, moving_piece: &Piece) {
        match (moving_piece.kind, self.en_passant_target_square()) {
            (PieceKind::Pawn, Some(en_passant_square)) => {
                if mov.to == en_passant_square {
                    let square_to_clear = match moving_piece.color {
                        Color::White => en_passant_square - 8,
                        Color::Black => en_passant_square + 8
                    };
                    self.set_at(square_to_clear, Square::Empty)
                }
            }
            _ => {}
        }
    }

    fn is_double_pawn_move(mov: &Move, moving_piece: &Piece) -> bool {
        if moving_piece.kind != PieceKind::Pawn {
            return false;
        }

        mov.from.abs_diff(mov.to) > 9
    }

    fn update_en_passant_target_square(&mut self, mov: &Move, moving_piece: &Piece) {
        if Self::is_double_pawn_move(mov, moving_piece) {
            self.set_en_passant_target_square(Some((mov.from + mov.to) / 2))
        } else {
            self.set_en_passant_target_square(None)
        }
    }

    pub fn move_piece(&mut self, mov: &Move) {
        let moving_piece = match self.at(mov.from) {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        };

        self.castle_invalidation(mov);
        self.move_rook_if_castle(mov, &moving_piece);
        self.remove_piece_after_en_passant(mov, &moving_piece);
        self.update_en_passant_target_square(mov, &moving_piece);

        let promoted_piece = match mov.promoted_piece_kind {
            None => moving_piece,
            Some(promoted_piece_kind) => Piece {
                color: moving_piece.color,
                kind: promoted_piece_kind,
            },
        };

        self.set_at(mov.from, Square::Empty);
        self.set_at(mov.to, Square::Occupied(promoted_piece));

        self.next_turn();
    }

    pub fn move_piece_uci(&mut self, uci: &str) {
        let mov = Move::from_uci_string(uci);
        self.move_piece(&mov);
    }
}

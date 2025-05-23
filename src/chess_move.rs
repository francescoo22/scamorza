use crate::chess_board::{ChessBoard, Square};
use crate::chess_piece::{Piece, PieceKind};
use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub promoted_piece_kind: Option<PieceKind>,
}

impl Move {
    pub fn base_move(from: (usize, usize), to: (usize, usize)) -> Self {
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
        let from_col = (b'h' - bytes[0]) as usize;
        let from_row = (bytes[1] - b'1') as usize;
        let to_col = (b'h' - bytes[2]) as usize;
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
            from: (from_row, from_col),
            to: (to_row, to_col),
            promoted_piece_kind: promoted_piece,
        }
    }

    pub fn to_uci_string(&self) -> String {
        let from_row = char::from(self.from.0 as u8 + b'1');
        let from_col = char::from(b'h' - self.from.1 as u8);
        let to_row = char::from(self.to.0 as u8 + b'1');
        let to_col = char::from(b'h' - self.to.1 as u8);
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

        match mov.to {
            (0, 0) => self.can_white_castle_kingside = false,
            (0, 7) => self.can_white_castle_queenside = false,
            (7, 0) => self.can_black_castle_kingside = false,
            (7, 7) => self.can_black_castle_queenside = false,
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

    fn remove_piece_after_en_passant(&mut self, mov: &Move, moving_piece: &Piece) {
        match (moving_piece.kind, self.en_passant_target_square) {
            (PieceKind::Pawn, Some(en_passant_square)) => {
                if mov.to == en_passant_square {
                    self.set_at(mov.from.0, en_passant_square.1, Square::Empty)
                }
            }
            _ => {}
        }
    }

    fn update_en_passant_target_square(&mut self, mov: &Move, moving_piece: &Piece) {
        match moving_piece.kind {
            PieceKind::Pawn => {
                let move_length = mov.from.0.abs_diff(mov.to.0);
                if move_length == 1 {
                    self.en_passant_target_square = None
                } else {
                    self.en_passant_target_square = Some(((mov.from.0 + mov.to.0) / 2, mov.to.1))
                }
            }
            _ => self.en_passant_target_square = None,
        }
    }

    pub fn move_piece(&mut self, mov: &Move) {
        let moving_piece = match self.at(mov.from.0, mov.from.1) {
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


        self.set_at(mov.from.0, mov.from.1, Square::Empty);
        self.set_at(mov.to.0, mov.to.1, Square::Occupied(promoted_piece));
    }

    pub fn move_piece_uci(&mut self, uci: &str) {
        let mov = Move::from_uci_string(uci);
        self.move_piece(&mov);
    }
}

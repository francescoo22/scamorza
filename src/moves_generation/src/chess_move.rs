use board_representation::chess_board::{ChessBoard, Square, SquareIndex};
use board_representation::chess_piece::{Color, Piece, PieceKind, BLACK_ROOK, WHITE_ROOK};
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

    pub fn to_uci_string(self) -> String {
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

    fn castle_invalidation(&self, board: &mut ChessBoard) {
        match self.from {
            0 => board.set_white_castle_queenside(false),
            7 => board.set_white_castle_kingside(false),
            56 => board.set_black_castle_queenside(false),
            63 => board.set_black_castle_kingside(false),
            4 => {
                board.set_white_castle_kingside(false);
                board.set_white_castle_queenside(false);
            }
            60 => {
                board.set_black_castle_kingside(false);
                board.set_black_castle_queenside(false);
            }
            _ => {}
        }

        match self.to {
            0 => board.set_white_castle_queenside(false),
            7 => board.set_white_castle_kingside(false),
            56 => board.set_black_castle_queenside(false),
            63 => board.set_black_castle_kingside(false),
            _ => {}
        }
    }

    fn move_rook_when_castling(&self, board: &mut ChessBoard, moving_piece: &Piece) {
        if moving_piece.kind != PieceKind::King {
            return;
        }

        if self.from == 4 {
            if self.to == 2 {
                board.set_at(3, Square::Occupied(WHITE_ROOK));
                board.set_at(0, Square::Empty)
            } else if self.to == 6 {
                board.set_at(5, Square::Occupied(WHITE_ROOK));
                board.set_at(7, Square::Empty)
            }
        } else if self.from == 60 {
            if self.to == 58 {
                board.set_at(59, Square::Occupied(BLACK_ROOK));
                board.set_at(56, Square::Empty)
            } else if self.to == 62 {
                board.set_at(61, Square::Occupied(BLACK_ROOK));
                board.set_at(63, Square::Empty)
            }
        }
    }

    fn remove_piece_after_en_passant(&self, board: &mut ChessBoard, moving_piece: &Piece) {
        if let (PieceKind::Pawn, Some(en_passant_square)) =
            (moving_piece.kind, board.en_passant_target_square())
        {
            if self.to == en_passant_square {
                let square_to_clear = match moving_piece.color {
                    Color::White => en_passant_square - 8,
                    Color::Black => en_passant_square + 8,
                };
                board.set_at(square_to_clear, Square::Empty)
            }
        }
    }

    fn is_double_pawn_move(&self, moving_piece: &Piece) -> bool {
        if moving_piece.kind != PieceKind::Pawn {
            return false;
        }

        self.from.abs_diff(self.to) > 9
    }

    fn update_en_passant_target_square(&self, board: &mut ChessBoard, moving_piece: &Piece) {
        if self.is_double_pawn_move(moving_piece) {
            board.set_en_passant_target_square(Some((self.from + self.to) / 2))
        } else {
            board.set_en_passant_target_square(None)
        }
    }

    pub fn move_piece(&self, board: &mut ChessBoard) {
        let moving_piece = match board.at(self.from) {
            Square::Occupied(piece) => piece,
            Square::Empty => panic!("Invalid move: Cannot move from empty square"),
        };

        self.castle_invalidation(board);
        self.move_rook_when_castling(board, &moving_piece);
        self.remove_piece_after_en_passant(board, &moving_piece);
        self.update_en_passant_target_square(board, &moving_piece);

        let promoted_piece = match self.promoted_piece_kind {
            None => moving_piece,
            Some(promoted_piece_kind) => Piece {
                color: moving_piece.color,
                kind: promoted_piece_kind,
            },
        };

        board.set_at(self.from, Square::Empty);
        board.set_at(self.to, Square::Occupied(promoted_piece));

        board.next_turn();
    }
}

pub fn move_piece_uci(board: &mut ChessBoard, uci: &str) {
    let mov = Move::from_uci_string(uci);
    mov.move_piece(board);
}

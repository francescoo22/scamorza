use crate::chess_board::{ChessBoard, Square};
use crate::chess_piece::{Color, Piece, PieceKind};

impl ChessBoard {
    fn find_king(self, color: Color) -> (i32, i32) {
        let to_find = Square::Occupied(Piece {
            kind: PieceKind::King,
            color,
        });
        for i in 0..8 {
            for j in 0..8 {
                if self.squares[i][j] == to_find {
                    return (i as i32, j as i32);
                }
            }
        }
        panic!("King not found on the chess board");
    }

    pub fn is_square_checked(self, i: i32, j: i32, color: Color) -> bool {
        self.all_possible_moves(!color)
            .iter()
            .any(|it| it.to.0 as i32 == i && it.to.1 as i32 == j)
    }

    pub fn is_king_checked(self, color: Color) -> bool {
        let (i, j) = self.find_king(color);
        self.is_square_checked(i, j, color)
    }

    pub fn king_cannot_move(self, color: Color) -> bool {
        self.is_king_checked(color) && self.all_valid_moves(color).is_empty()
    }
}

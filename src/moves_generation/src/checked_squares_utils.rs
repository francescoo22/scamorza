use board_representation::chess_board::*;
use board_representation::chess_piece::*;

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

fn find_king(board: &ChessBoard, color: Color) -> SquareIndex {
    let king_bit_board = match color {
        Color::White => board.white_pieces & board.kings,
        Color::Black => board.black_pieces & board.kings,
    };
    king_bit_board.trailing_zeros() as SquareIndex
}

pub fn contains_piece_in_any_direction(
    board: &ChessBoard,
    index: SquareIndex,
    piece_to_find: Piece,
    directions: &[SquareIndexDelta],
) -> bool {
    directions
        .iter()
        .any(|delta| board.contains_piece_at(apply_delta(index, *delta), piece_to_find))
}

fn is_square_checked_by_knight(board: &ChessBoard, index: SquareIndex, color: Color) -> bool {
    let piece_to_find = Piece {
        kind: PieceKind::Knight,
        color: !color,
    };
    contains_piece_in_any_direction(board, index, piece_to_find, &KNIGHT_DIRECTIONS)
}

fn is_square_checked_by_slider(board: &ChessBoard, index: SquareIndex, color: Color) -> bool {
    for delta in KING_DIRECTIONS {
        let mut dist = 1;
        while board
            .within_bounds_and_empty(apply_delta_with_dist(index, delta, dist))
            .is_some()
        {
            dist += 1;
        }
        match board.maybe_piece_at(apply_delta_with_dist(index, delta, dist)) {
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

fn is_square_checked_by_king(board: &ChessBoard, index: SquareIndex, color: Color) -> bool {
    let piece_to_find = Piece {
        kind: PieceKind::King,
        color: !color,
    };
    contains_piece_in_any_direction(board, index, piece_to_find, &KING_DIRECTIONS)
}

pub fn is_square_checked_by_pawn(board: &ChessBoard, index: SquareIndex, color: Color) -> bool {
    let di = match color {
        Color::White => 1,
        Color::Black => -1,
    };
    for dj in [1, -1] {
        match board.maybe_piece_at(apply_delta(index, (di, dj))) {
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

pub fn is_square_checked(board: &ChessBoard, index: SquareIndex, color: Color) -> bool {
    is_square_checked_by_slider(board, index, color)
        || is_square_checked_by_knight(board, index, color)
        || is_square_checked_by_king(board, index, color)
        || is_square_checked_by_pawn(board, index, color)
}

pub fn is_king_checked(board: &ChessBoard, color: Color) -> bool {
    is_square_checked(board, find_king(board, color), color)
}

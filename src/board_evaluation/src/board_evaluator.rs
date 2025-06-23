use crate::evaluation_constants::{BISHOP_WEIGHT, DOUBLED_PAWN_WEIGHT, KING_WEIGHT, KNIGHT_WEIGHT, PAWN_WEIGHT, QUEEN_WEIGHT, ROOK_WEIGHT};
use board_representation::chess_board::ChessBoard;

pub type BoardScore = f32;

pub struct BoardEvaluator {
    pub eval_material: bool,
    pub eval_doubled_pawns: bool,
}

impl BoardEvaluator {
    pub fn eval_board(&self, chess_board: &ChessBoard) -> BoardScore {
        let mut score = 0.0;
        if self.eval_material {
            score += eval_material(chess_board)
        }
        if self.eval_doubled_pawns {
            score += eval_doubled_pawns(chess_board)
        }
        score
    }
}

fn eval_material(chess_board: &ChessBoard) -> BoardScore {
    let king_count = (chess_board.white_pieces & chess_board.kings).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.kings).count_ones() as f32;
    let queen_count = (chess_board.white_pieces & chess_board.queens).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.queens).count_ones() as f32;
    let rook_count = (chess_board.white_pieces & chess_board.rooks).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.rooks).count_ones() as f32;
    let bishop_count = (chess_board.white_pieces & chess_board.bishops).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.bishops).count_ones() as f32;
    let knight_count = (chess_board.white_pieces & chess_board.knights).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.knights).count_ones() as f32;
    let pawn_count = (chess_board.white_pieces & chess_board.pawns).count_ones() as f32 -
        (chess_board.black_pieces & chess_board.pawns).count_ones() as f32;

    KING_WEIGHT * king_count +
        QUEEN_WEIGHT * queen_count +
        ROOK_WEIGHT * rook_count +
        BISHOP_WEIGHT * bishop_count +
        KNIGHT_WEIGHT * knight_count +
        PAWN_WEIGHT * pawn_count
}

fn eval_doubled_pawns(chess_board: &ChessBoard) -> BoardScore {
    let mut current_file = 0x0101010101010101;
    let mut doubled_pawns: i32 = 0;
    for _ in 0..8 {
        let pawns_on_file = chess_board.pawns & current_file;
        let white_pawns = (pawns_on_file & chess_board.white_pieces).count_ones() as i32;
        let black_pawns = (pawns_on_file & chess_board.black_pieces).count_ones() as i32;
        if white_pawns > 1 {
            doubled_pawns += white_pawns
        }
        if black_pawns > 1 {
            doubled_pawns -= black_pawns
        }
        current_file <<= 1;
    }
    DOUBLED_PAWN_WEIGHT * doubled_pawns as f32
}
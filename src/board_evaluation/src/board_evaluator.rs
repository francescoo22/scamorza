use crate::evaluation_constants::{BISHOP_WEIGHT, BLOCKED_PAWN_WEIGHT, DOUBLED_PAWN_WEIGHT, ISOLATED_PAWN_WEIGHT, KING_WEIGHT, KNIGHT_WEIGHT, PAWN_WEIGHT, QUEEN_WEIGHT, ROOK_WEIGHT};
use board_representation::chess_board::ChessBoard;
use board_representation::chess_board_utils::FILE_MASK;

pub type BoardScore = f32;

pub struct BoardEvaluator {
    pub eval_material: bool,
    pub eval_doubled_pawns: bool,
    pub eval_isolated_pawns: bool,
    pub eval_blocked_pawns: bool,
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
        if self.eval_isolated_pawns {
            score += eval_isolated_pawns(chess_board)
        }
        if self.eval_blocked_pawns {
            score += eval_blocked_pawns(chess_board)
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
    let mut doubled_pawns: i32 = 0;
    for i in 0..8 {
        let pawns_on_file = chess_board.pawns & FILE_MASK[i];
        let white_pawns = (pawns_on_file & chess_board.white_pieces).count_ones() as i32;
        let black_pawns = (pawns_on_file & chess_board.black_pieces).count_ones() as i32;
        if white_pawns > 1 {
            doubled_pawns += white_pawns
        }
        if black_pawns > 1 {
            doubled_pawns -= black_pawns
        }
    }
    DOUBLED_PAWN_WEIGHT * doubled_pawns as f32
}

fn eval_isolated_pawns(chess_board: &ChessBoard) -> BoardScore {
    let mut isolated_pawns = 0;
    let mut white_pawns_on_file: [i32; 8] = [0; 8];
    let mut black_pawns_on_file: [i32; 8] = [0; 8];
    for file in 0..8 {
        let pawns_on_file = chess_board.pawns & FILE_MASK[file];
        white_pawns_on_file[file] = (pawns_on_file & chess_board.white_pieces).count_ones() as i32;
        black_pawns_on_file[file] = (pawns_on_file & chess_board.black_pieces).count_ones() as i32;
    }
    for file in 0..8 {
        if white_pawns_on_file[file] > 0 {
            let pawns_on_the_left = if file > 0 { white_pawns_on_file[file - 1] } else { 0 };
            let pawns_on_the_right = if file < 7 { white_pawns_on_file[file + 1] } else { 0 };
            if pawns_on_the_left == 0 && pawns_on_the_right == 0 {
                isolated_pawns += white_pawns_on_file[file];
            }
        }
        if black_pawns_on_file[file] > 0 {
            let pawns_on_the_left = if file > 0 { black_pawns_on_file[file - 1] } else { 0 };
            let pawns_on_the_right = if file < 7 { black_pawns_on_file[file + 1] } else { 0 };
            if pawns_on_the_left == 0 && pawns_on_the_right == 0 {
                isolated_pawns -= black_pawns_on_file[file];
            }
        }
    }
    ISOLATED_PAWN_WEIGHT * isolated_pawns as f32
}


fn eval_blocked_pawns(chess_board: &ChessBoard) -> BoardScore {
    // todo: decide what to do with pawns that can take an opponent piece (ignored for now)

    let mut blocked_pawns = 0;
    let occupied = chess_board.white_pieces | chess_board.black_pieces;

    let white_pawns = chess_board.pawns & chess_board.white_pieces;
    let white_pawn_pushes = white_pawns << 8;
    let white_blockers = white_pawn_pushes & occupied;
    blocked_pawns += white_blockers.count_ones() as i32;

    let black_pawns = chess_board.pawns & chess_board.black_pieces;
    let black_pawn_pushes = black_pawns >> 8;
    let black_blockers = black_pawn_pushes & occupied;
    blocked_pawns -= black_blockers.count_ones() as i32;

    BLOCKED_PAWN_WEIGHT * blocked_pawns as f32
}

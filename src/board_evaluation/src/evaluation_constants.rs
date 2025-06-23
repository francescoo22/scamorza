use crate::board_evaluator::BoardScore;

pub(crate) const KING_WEIGHT: BoardScore = 200.0;
pub(crate) const QUEEN_WEIGHT: BoardScore = 9.0;
pub(crate) const ROOK_WEIGHT: BoardScore = 5.0;
pub(crate) const BISHOP_WEIGHT: BoardScore = 3.0;
pub(crate) const KNIGHT_WEIGHT: BoardScore = 3.0;
pub(crate) const PAWN_WEIGHT: BoardScore = 1.0;
pub(crate) const DOUBLED_PAWN_WEIGHT: BoardScore = -0.5;

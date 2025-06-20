pub(crate) type SquareIndexDelta = (i8, i8);
pub(crate) const KNIGHT_DIRECTIONS: [SquareIndexDelta; 8] = [
    (1, 2),
    (2, 1),
    (-1, 2),
    (2, -1),
    (1, -2),
    (-2, 1),
    (-1, -2),
    (-2, -1),
];
pub(crate) const BISHOP_DIRECTIONS: [SquareIndexDelta; 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

pub(crate) const ROOK_DIRECTIONS: [SquareIndexDelta; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub(crate) const KING_DIRECTIONS: [SquareIndexDelta; 8] = [
    (1, 1),
    (1, 0),
    (1, -1),
    (0, 1),
    (0, -1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

use chess_engine_poc::chess_board::ChessBoard;
use chess_engine_poc::chess_piece::Color;

fn perft(depth: u8) -> u64 {
    let mut board = ChessBoard::initial_board();
    return perft_rec(depth, &mut board, Color::White);

    fn perft_rec(depth: u8, chess_board: &mut ChessBoard, color: Color) -> u64 {
        let moves = chess_board.all_valid_moves(color);
        if depth == 1 {
            moves.len() as u64
        } else {
            let mut res = 0;
            for mov in moves {
                let prev_square = chess_board.squares[mov.to.0][mov.to.1];
                chess_board.move_piece(&mov);
                res += perft_rec(depth - 1, chess_board, !color);
                chess_board.move_piece_back(&mov);
                chess_board.squares[mov.to.0][mov.to.1] = prev_square;
            }
            res
        }
    }
}

#[test]
fn test_perft_depth_1() {
    assert_eq!(perft(1), 20);
}

#[test]
fn test_perft_depth_2() {
    assert_eq!(perft(2), 400);
}

#[test]
fn test_perft_depth_3() {
    assert_eq!(perft(3), 8902);
}

#[test]
fn test_perft_depth_4() {
    assert_eq!(perft(4), 197281);
}

#[test]
fn test_perft_depth_5() {
    assert_eq!(perft(5), 4865609);
}
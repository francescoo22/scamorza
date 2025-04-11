use chess_engine_poc::chess_board::ChessBoard;
use chess_engine_poc::chess_piece::Color;

fn perft(depth: u8) -> u64 {
    let board = ChessBoard::initial_board();
    return perft_rec(depth, board, Color::White);

    fn perft_rec(depth: u8, chess_board: ChessBoard, color: Color) -> u64 {
        let moves = chess_board.all_valid_moves(color);
        if depth == 1 {
            moves.len() as u64
        } else {
            let mut res = 0;
            for mov in moves {
                let mut new_board = chess_board.clone();
                new_board.move_piece(&mov.to_uci_string());
                res += perft_rec(depth - 1, new_board, !color);
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

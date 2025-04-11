use chess_engine_poc::chess_board::ChessBoard;
use chess_engine_poc::chess_piece::Color;
use std::str::FromStr;

fn perft(depth: u8, chess_board: &ChessBoard, color: Color) -> u64 {
    let moves = chess_board.all_valid_moves(color);
    if depth == 1 {
        moves.len() as u64
    } else {
        let mut res = 0;
        for mov in moves {
            let mut board_copy = chess_board.clone();
            board_copy.move_piece(&mov);
            res += perft(depth - 1, &board_copy, !color);
        }
        res
    }
}

#[test]
fn test_perft_depth_1() {
    assert_eq!(perft(1, &mut ChessBoard::default(), Color::White), 20);
}

#[test]
fn test_perft_depth_2() {
    assert_eq!(perft(2, &mut ChessBoard::default(), Color::White), 400);
}

#[test]
fn test_perft_depth_3() {
    assert_eq!(perft(3, &mut ChessBoard::default(), Color::White), 8902);
}

#[test]
#[ignore = "Slow test"]
fn test_perft_depth_4() {
    assert_eq!(perft(4, &mut ChessBoard::default(), Color::White), 197281);
}

#[test]
#[ignore = "Slow test"]
fn test_perft_depth_5() {
    assert_eq!(perft(5, &mut ChessBoard::default(), Color::White), 4865609);
}

#[test]
fn test_kiwipete_depth_1() {
    let chess_board = ChessBoard::from_str(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
    ).unwrap();

    assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 48);
}
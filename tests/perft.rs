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

#[cfg(test)]
mod perft {
    use super::*;
    mod initial_position {
        use super::*;
        #[test]
        fn test_initial_position_depth_1() {
            assert_eq!(perft(1, &mut ChessBoard::default(), Color::White), 20);
        }

        #[test]
        fn test_initial_position_depth_2() {
            assert_eq!(perft(2, &mut ChessBoard::default(), Color::White), 400);
        }

        #[test]
        fn test_initial_position_depth_3() {
            assert_eq!(perft(3, &mut ChessBoard::default(), Color::White), 8902);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_initial_position_depth_4() {
            assert_eq!(perft(4, &mut ChessBoard::default(), Color::White), 197281);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_initial_position_depth_5() {
            assert_eq!(perft(5, &mut ChessBoard::default(), Color::White), 4865609);
        }
    }

    mod position_2 {
        use super::*;
        #[test]
        fn test_position_2_depth_1() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 48);
        }

        #[test]
        fn test_position_2_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();

            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 2039);
        }

        #[test]
        fn test_position_2_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 97862);
        }
    }

    mod position_3 {
        use super::*;
        #[test]
        fn test_position_3_depth_1() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 14);
        }

        #[test]
        fn test_position_3_depth_2() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 191);
        }

        #[test]
        fn test_position_3_depth_3() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            // todo: add en-passant
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 2812);
        }

        #[test]
        fn test_position_3_depth_4() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 43238);
        }
    }

    mod position_4 {
        use super::*;
        #[test]
        fn test_position_4_depth_1() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 6);
        }

        #[test]
        fn test_position_4_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            // todo: all possible promotions
            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 264);
        }

        #[test]
        fn test_position_4_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 9467);
        }
    }
}

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

        // TODO: run once (~6 min) and failed, investigate
        // Expected :4085603
        // Actual   :4085718
        #[test]
        #[ignore = "Slow test"]
        fn test_position_2_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 4085603);
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

        #[test]
        fn test_position_4_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 422333);
        }
    }

    mod position_4_mirrored {
        use super::*;
        #[test]
        fn test_position_4_depth_1() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 6);
        }

        #[test]
        fn test_position_4_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 264);
        }

        #[test]
        fn test_position_4_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 9467);
        }

        #[test]
        fn test_position_4_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 422333);
        }
    }

    mod position_5 {
        use super::*;

        #[test]
        fn test_position_5_depth_1() {
            let chess_board = ChessBoard::from_str(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            )
                .unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 44);
        }

        #[test]
        fn test_position_5_depth_2() {
            let chess_board = ChessBoard::from_str(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 1486);
        }


        // TODO: investigate:
        // actual: 62416
        // expected: 62379
        #[test]
        fn test_position_5_depth_3() {
            let chess_board = ChessBoard::from_str(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 62379);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_5_depth_4() {
            let chess_board = ChessBoard::from_str(
                "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 2103487);
        }
    }

    mod position_6 {
        use super::*;

        #[test]
        fn test_position_6_depth_1() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(1, &chess_board, chess_board.side_to_move()), 46);
        }

        #[test]
        fn test_position_6_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board, chess_board.side_to_move()), 2079);
        }

        #[test]
        fn test_position_6_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board, chess_board.side_to_move()), 89890);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_6_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board, chess_board.side_to_move()), 3894594);
        }
    }
}

use chess_engine_poc::chess_board::ChessBoard;
use std::str::FromStr;

fn perft(depth: u8, chess_board: &ChessBoard) -> u64 {
    let color = chess_board.current_turn();
    let moves = chess_board.all_valid_moves(color);
    if depth == 1 {
        moves.len() as u64
    } else {
        let mut res = 0;
        for mov in moves {
            let mut board_copy = chess_board.clone();
            board_copy.move_piece(&mov);
            res += perft(depth - 1, &board_copy);
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
            assert_eq!(perft(1, &mut ChessBoard::default()), 20);
        }

        #[test]
        fn test_initial_position_depth_2() {
            assert_eq!(perft(2, &mut ChessBoard::default()), 400);
        }

        #[test]
        fn test_initial_position_depth_3() {
            assert_eq!(perft(3, &mut ChessBoard::default()), 8902);
        }

        #[test]
        fn test_initial_position_depth_4() {
            assert_eq!(perft(4, &mut ChessBoard::default()), 197281);
        }

        #[test]
        fn test_initial_position_depth_5() {
            assert_eq!(perft(5, &mut ChessBoard::default()), 4865609);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_initial_position_depth_6() {
            assert_eq!(perft(6, &mut ChessBoard::default()), 119060324);
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
            assert_eq!(perft(1, &chess_board), 48);
        }

        #[test]
        fn test_position_2_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();

            assert_eq!(perft(2, &chess_board), 2039);
        }

        #[test]
        fn test_position_2_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board), 97862);
        }

        #[test]
        fn test_position_2_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board), 4085603);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_2_depth_5() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -  ",
            )
                .unwrap();
            assert_eq!(perft(5, &chess_board), 193690690);
        }
    }

    mod position_3 {
        use super::*;
        #[test]
        fn test_position_3_depth_1() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(1, &chess_board), 14);
        }

        #[test]
        fn test_position_3_depth_2() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(2, &chess_board), 191);
        }

        #[test]
        fn test_position_3_depth_3() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(3, &chess_board), 2812);
        }

        #[test]
        fn test_position_3_depth_4() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(4, &chess_board), 43238);
        }

        #[test]
        fn test_position_3_depth_5() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(5, &chess_board), 674624);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_3_depth_6() {
            let chess_board =
                ChessBoard::from_str("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1").unwrap();
            assert_eq!(perft(6, &chess_board), 11030083);
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
            assert_eq!(perft(1, &chess_board), 6);
        }

        #[test]
        fn test_position_4_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board), 264);
        }

        #[test]
        fn test_position_4_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board), 9467);
        }

        #[test]
        fn test_position_4_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board), 422333);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_4_depth_5() {
            let chess_board = ChessBoard::from_str(
                "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            )
                .unwrap();
            assert_eq!(perft(5, &chess_board), 15833292);
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
            assert_eq!(perft(1, &chess_board), 6);
        }

        #[test]
        fn test_position_4_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board), 264);
        }

        #[test]
        fn test_position_4_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board), 9467);
        }

        #[test]
        fn test_position_4_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board), 422333);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_4_depth_5() {
            let chess_board = ChessBoard::from_str(
                "r2q1rk1/pP1p2pp/Q4n2/bbp1p3/Np6/1B3NBn/pPPP1PPP/R3K2R b KQ - 0 1",
            )
                .unwrap();
            assert_eq!(perft(5, &chess_board), 15833292);
        }
    }

    mod position_5 {
        use super::*;

        #[test]
        fn test_position_5_depth_1() {
            let chess_board =
                ChessBoard::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            assert_eq!(perft(1, &chess_board), 44);
        }

        #[test]
        fn test_position_5_depth_2() {
            let chess_board =
                ChessBoard::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            assert_eq!(perft(2, &chess_board), 1486);
        }

        #[test]
        fn test_position_5_depth_3() {
            let chess_board =
                ChessBoard::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            assert_eq!(perft(3, &chess_board), 62379);
        }

        #[test]
        fn test_position_5_depth_4() {
            let chess_board =
                ChessBoard::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            assert_eq!(perft(4, &chess_board), 2103487);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_5_depth_5() {
            let chess_board =
                ChessBoard::from_str("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8")
                    .unwrap();
            assert_eq!(perft(5, &chess_board), 89941194);
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
            assert_eq!(perft(1, &chess_board), 46);
        }

        #[test]
        fn test_position_6_depth_2() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(2, &chess_board), 2079);
        }

        #[test]
        fn test_position_6_depth_3() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(3, &chess_board), 89890);
        }

        #[test]
        fn test_position_6_depth_4() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(4, &chess_board), 3894594);
        }

        #[test]
        #[ignore = "Slow test"]
        fn test_position_6_depth_5() {
            let chess_board = ChessBoard::from_str(
                "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            )
                .unwrap();
            assert_eq!(perft(5, &chess_board), 164075551);
        }
    }
}

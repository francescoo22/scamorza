#[cfg(test)]
mod eval {
    use board_evaluation::board_evaluator::BoardEvaluator;
    use board_evaluation::board_evaluator::BoardScore;
    use board_representation::chess_board::ChessBoard;
    use std::str::FromStr;

    mod material {
        use super::*;
        fn eval_material(chess_board: &ChessBoard) -> BoardScore {
            let evaluator = BoardEvaluator {
                eval_material: true,
                eval_doubled_pawns: false,
            };
            evaluator.eval_board(chess_board)
        }

        #[test]
        fn eval_initial_board() {
            let board = ChessBoard::default();
            assert_eq!(eval_material(&board), 0.0);
        }

        #[test]
        fn eval_board_1() {
            let board = ChessBoard::from_str(
                "8/8/8/kKP5/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_material(&board), 1.0);
        }

        #[test]
        fn eval_board_2() {
            let board = ChessBoard::from_str(
                "8/8/8/kKrr4/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_material(&board), -10.0);
        }

        #[test]
        fn eval_board_3() {
            let board = ChessBoard::from_str(
                "8/8/8/QKRRNNBB/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_material(&board), 231.0);
        }

        #[test]
        fn eval_board_4() {
            let board = ChessBoard::from_str(
                "8/8/8/qkrrnnbb/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_material(&board), -231.0);
        }

        #[test]
        fn eval_board_5() {
            let board = ChessBoard::from_str(
                "8/8/4pppp/qkrrnnbb/QKRRNNBB/8/5PPP/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_material(&board), -1.0);
        }
    }

    mod doubled_pawns {
        use super::*;

        fn eval_doubled_pawns(chess_board: &ChessBoard) -> BoardScore {
            let evaluator = BoardEvaluator {
                eval_material: false,
                eval_doubled_pawns: true,
            };
            evaluator.eval_board(chess_board)
        }

        #[test]
        fn eval_initial_board() {
            let board = ChessBoard::default();
            assert_eq!(eval_doubled_pawns(&board), 0.0);
        }

        #[test]
        fn eval_board_1() {
            let board = ChessBoard::from_str(
                "p7/p7/8/8/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_doubled_pawns(&board), 1.0);
        }

        #[test]
        fn eval_board_2() {
            let board = ChessBoard::from_str(
                "P7/P7/8/8/8/8/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_doubled_pawns(&board), -1.0);
        }

        #[test]
        fn eval_board_3() {
            let board = ChessBoard::from_str(
                "3PP3/8/8/8/3PP3/P7/8/8 w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_doubled_pawns(&board), -2.0);
        }

        #[test]
        fn eval_board_4() {
            let board = ChessBoard::from_str(
                "PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP/PPPPPPPP w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_doubled_pawns(&board), -32.0);
        }

        #[test]
        fn eval_board_5() {
            let board = ChessBoard::from_str(
                "pppppppp/PPPPPPPP/pppppppp/PPPPPPPP/pppppppp/pppppppp/PPPPPPPP/PPPPPPPP w KQkq -  ",
            ).unwrap();
            assert_eq!(eval_doubled_pawns(&board), 0.0);
        }
    }
}
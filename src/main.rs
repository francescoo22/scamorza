use crate::chess_piece::Color;
use rand::rng;
use rand::seq::IndexedRandom;
use std::io;

mod check_mate;
mod chess_board;
mod chess_move;
mod chess_piece;

fn random_game() {
    let mut board = chess_board::ChessBoard::initial_board();
    let mut rng = rng();
    let mut turn = Color::White;
    let mut game = 0;
    let mut moves = 0;
    let mut input = String::new();
    loop {
        moves += 1;
        println!("Game# {}. Turn: {:?}. Move# {}", game, turn, moves);
        println!("{}", board);
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        let valid_moves = board.all_valid_moves(turn);
        let random_move = valid_moves.choose(&mut rng);
        match random_move {
            None => {
                game += 1;
                if board.king_cannot_move(turn) {
                    println!("Checkmate! {:?} wins", !turn);
                } else {
                    println!("Stalemate!");
                }
                board = chess_board::ChessBoard::initial_board();
                turn = Color::White;
            }
            Some(mov) => {
                println!("{}", random_move.unwrap().to_uci_string());
                board.move_piece(&mov.to_uci_string());
                if board.is_stalemate() {
                    println!("Stalemate!");
                    board = chess_board::ChessBoard::initial_board();
                    turn = Color::White;
                    game += 1;
                }
            }
        }
        turn = !turn;
    }
}

fn main() {
    random_game();
}

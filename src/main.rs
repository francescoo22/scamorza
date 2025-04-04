use crate::chess_piece::Color;
use rand::rng;
use rand::seq::IndexedRandom;
use std::io;

mod chess_board;
mod chess_move;
mod chess_piece;

fn random_game() {
    let mut board = chess_board::ChessBoard::initial_board();
    let mut rng = rng();
    let mut turn = Color::White;
    let mut input = String::new();
    loop {
        println!("{}", board);
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading line");
        let valid_moves = board.all_valid_moves(turn);
        let random_move = valid_moves.choose(&mut rng);
        match random_move {
            None => {
                println!("Stalemate!");
                return;
            }
            Some(mov) => {
                println!("{}", random_move.unwrap().to_uci_string());
                board.move_piece(&mov.to_uci_string());
            }
        }
        turn = !turn;
    }
}

fn main() {
    random_game();
}

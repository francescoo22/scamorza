use crate::chess_board::Color;

mod chess_board;
mod chess_move;

fn main() {
    let mut board = chess_board::ChessBoard::initial_board();
    println!("{}", board);
    board.all_valid_moves(Color::White);
    board.move_piece("d2d4");
    println!("{}", board);
    board.all_valid_moves(Color::White);
    board.move_piece("e7e5");
    println!("{}", board);
    board.all_valid_moves(Color::White);
}

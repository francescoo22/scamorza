mod chess_board;
mod chess_move;

fn main() {
    let mut board = chess_board::ChessBoard::initial_board();
    println!("{}", board);
    board.all_valid_moves();
    board.move_piece("e2e4");
    println!("{}", board);
    board.all_valid_moves();
    board.move_piece("c7c6");
    println!("{}", board);
    board.all_valid_moves();
    board.move_piece("a2a4");
    println!("{}", board);
    board.all_valid_moves();
    board.move_piece("d7d5");
    println!("{}", board);
    board.all_valid_moves();
}

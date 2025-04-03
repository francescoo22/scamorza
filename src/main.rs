mod chess_board;
mod chess_move;

fn main() {
    let mut board = chess_board::ChessBoard::initial_board();
    println!("{}", board);
    board.move_piece("d2d4");
    println!("{}", board);
    board.move_piece("f7f6");
    println!("{}", board);
}

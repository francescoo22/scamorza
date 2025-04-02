mod chess_board;

fn main() {
    let mut board = chess_board::ChessBoard::initial_board();
    println!("{}", board);
    board.move_piece((1, 3), (3, 3));
    println!("{}", board);
    board.move_piece((6, 5), (5, 5));
    println!("{}", board);
}

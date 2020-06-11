mod board;
fn main() {
    use board::*;
    let mut board = Board::new();
    board.place_piece(4, Player::X);
    board.place_piece(5, Player::O);
    board.place_piece(6, Player::X);
    println!("{}", board);
}

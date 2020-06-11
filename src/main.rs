mod board;
mod player;

fn main() {
    use board::*;

    let mut board = Board::new();

    let player_x = player::HumanPlayer::new();
    let player_o = player::HumanPlayer::new();

    fn check_game_over(board: &Board) -> bool {
        if board.is_draw() {
            println!("Draw!");
            true
        } else if let Some(piece) = board.is_victory() {
            println!("Player {} won!", piece);
            true
        } else {
            false
        }
    };

    let mut current_piece = Piece::X;
    loop {
        if check_game_over(&board) {
            break;
        }
        println!("{}", board);
        let player: &dyn player::Player = if current_piece == Piece::X {
            &player_x
        } else {
            &player_o
        };

        let index = player.play(&board, current_piece);
        board.place_piece(index, current_piece);

        current_piece = current_piece.swap();
    }
}

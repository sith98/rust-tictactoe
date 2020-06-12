mod board;
mod player;

fn main() {
    use board::*;

    let args: Vec<_> = std::env::args().collect();

    let player_x: Box<dyn player::Player> = args
        .get(1)
        .and_then(|s| player::choose_player(&s))
        .unwrap_or_else(player::random_player);

    let player_o: Box<dyn player::Player> = args
        .get(2)
        .and_then(|s| player::choose_player(&s))
        .unwrap_or_else(player::random_player);

    let mut board = Board::new();

    println!("X: {}, O: {}", player_x, player_o);
    println!("Game has started!");

    fn check_game_over(board: &Board) -> bool {
        if let Some(piece) = board.is_victory() {
            println!("Player {} won!", piece);
            true
        } else if board.is_draw() {
            println!("Draw!");
            true
        } else {
            false
        }
    };

    let mut current_piece = Piece::X;
    loop {
        println!("{}", board);
        if check_game_over(&board) {
            break;
        }
        let player: &dyn player::Player = if current_piece == Piece::X {
            player_x.as_ref()
        } else {
            player_o.as_ref()
        };

        print!("Player {}: ", current_piece);
        let index = player.play(&board, current_piece);
        board.place_piece(index, current_piece);

        current_piece = current_piece.swap();
    }
}

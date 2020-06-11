use super::Player;
use crate::board::{Board, Index, Piece};

pub struct HumanPlayer;
impl HumanPlayer {
    pub fn new() -> Self {
        HumanPlayer
    }
}

impl Player for HumanPlayer {
    fn play(&self, board: &Board, piece: Piece) -> Index {
        loop {
            println!("Player {}: select a cell...", piece);
            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer).unwrap();

            let index: usize = match buffer.trim().parse() {
                Err(_) => {
                    println!("Please enter a number!");
                    continue;
                }
                Ok(index) => index,
            };
            if !Board::is_valid_index(index) {
                println!("Please enter a valid cell number!");
                continue;
            }
            if !board.cell_is_empty(index) {
                println!("This cell is already occupied!");
                continue;
            }
            return index;
        }
    }
}

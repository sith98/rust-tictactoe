use super::Player;
use crate::board::{Board, Index, Piece};
use std::fmt::{Display, Error, Formatter};

pub struct HumanPlayer;

impl Display for HumanPlayer {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        formatter.write_str("Human player")
    }
}

impl Player for HumanPlayer {
    fn play(&self, board: &Board, _: Piece) -> Index {
        println!("select a cell...");
        loop {
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

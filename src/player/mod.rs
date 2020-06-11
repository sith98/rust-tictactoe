mod human_player;
mod minimax_player;

use super::board::{Board, Index, Piece};
use rand::Rng;
use std::fmt::Display;

pub use human_player::HumanPlayer;
pub use minimax_player::MinimaxPlayer;

pub trait Player: Display {
    fn play(&self, board: &Board, piece: Piece) -> Index;
}

pub fn choose_player(c: &str) -> Option<Box<dyn Player>> {
    match c.to_lowercase().as_str() {
        "human" | "h" => Some(Box::new(HumanPlayer)),
        "minimax" | "m" => Some(Box::new(MinimaxPlayer)),
        _ => None,
    }
}

// pub fn random_player() -> Box<dyn Player> {

//     let boxes: &mut [Option<Box<dyn Player>>] =
//         &mut [Some(Box::new(HumanPlayer)), Some(Box::new(MinimaxPlayer))];

//     let index = rand::thread_rng().gen_range(0, boxes.len());
//     boxes[index].take().unwrap()
// }

pub fn random_player() -> Box<dyn Player> {
    match rand::thread_rng().gen_range(0, 2) {
        0 => Box::new(HumanPlayer),
        1 => Box::new(MinimaxPlayer),
        _ => unreachable!(),
    }
}

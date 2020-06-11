mod human_player;

use super::board::{Board, Index, Piece};
pub use human_player::HumanPlayer;

pub trait Player {
    fn play(&self, board: &Board, piece: Piece) -> Index;
}

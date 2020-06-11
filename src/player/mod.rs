mod human_player;
mod minimax_player;

use super::board::{Board, Index, Piece};
pub use human_player::HumanPlayer;
pub use minimax_player::MinimaxPlayer;

pub trait Player {
    fn play(&self, board: &Board, piece: Piece) -> Index;
}

use super::Player;
use crate::board::{Board, Index, Piece};
use rand::seq::SliceRandom;

pub struct MinimaxPlayer;
impl MinimaxPlayer {
    pub fn new() -> Self {
        Self
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum GameResult {
    Defeat,
    Draw,
    Victory,
}

// minimax
impl MinimaxPlayer {
    fn minimax_search(board: Board, piece: Piece) -> (Index, u32) {
        let mut counter = 0;
        let mut best_move = 0;
        let mut indeces: Vec<_> = Board::VALID_INDECES.collect();

        indeces.shuffle(&mut rand::thread_rng());

        Self::minimax_step(
            board.clone(),
            piece,
            true,
            0,
            GameResult::Defeat,
            GameResult::Victory,
            &indeces[..],
            &mut best_move,
            &mut counter,
        );
        (best_move, counter)
    }

    fn minimax_step(
        board: Board,
        piece: Piece,
        maximizing: bool,
        level: u8,
        mut alpha: GameResult,
        mut beta: GameResult,
        indeces: &[Index],
        best_move: &mut Index,
        counter: &mut u32,
    ) -> GameResult {
        use GameResult::*;

        *counter += 1;

        if board.is_draw() {
            return Draw;
        }
        if board.is_victory_for_player(piece) {
            return Victory;
        }
        if board.is_victory_for_player(piece.swap()) {
            return Defeat;
        }

        let mut best_result = if maximizing { Defeat } else { Victory };
        for &index in indeces {
            if !board.cell_is_empty(index) {
                continue;
            }

            let mut new_board = board.clone();
            new_board.place_piece(index, if maximizing { piece } else { piece.swap() });
            let result = Self::minimax_step(
                new_board,
                piece,
                !maximizing,
                level + 1,
                alpha,
                beta,
                indeces,
                best_move,
                counter,
            );
            if maximizing && result > best_result || !maximizing && result < best_result {
                best_result = result;
                if level == 0 {
                    *best_move = index;
                }
            }

            // alpha beta pruning
            if maximizing {
                alpha = std::cmp::max(alpha, best_result);
                if alpha >= beta {
                    break;
                }
            } else {
                beta = std::cmp::min(beta, best_result);
                if beta <= alpha {
                    break;
                }
            }
        }
        best_result
    }
}

impl Player for MinimaxPlayer {
    fn play(&self, board: &Board, piece: Piece) -> Index {
        let (best_move, counter) = MinimaxPlayer::minimax_search(board.clone(), piece);
        println!("Searched {} possible games!", counter);

        best_move
    }
}

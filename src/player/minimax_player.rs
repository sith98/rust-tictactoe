use super::Player;
use crate::board::{Board, Index, Piece};
use rand::Rng;

pub struct MinimaxPlayer;
impl MinimaxPlayer {
    pub fn new() -> Self {
        Self
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum GameResult {
    Defeat,
    Draw,
    Victory,
}
impl std::ops::Neg for GameResult {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            GameResult::Defeat => GameResult::Victory,
            GameResult::Draw => GameResult::Draw,
            GameResult::Victory => GameResult::Defeat,
        }
    }
}

// minimax
impl MinimaxPlayer {
    fn minimax_search(board: Board, piece: Piece) -> (Vec<Index>, u32) {
        let mut counter = 0;
        let mut best_moves = Vec::new();

        Self::minimax_step(
            board.clone(),
            piece,
            true,
            true,
            GameResult::Defeat,
            GameResult::Victory,
            &mut best_moves,
            &mut counter,
        );
        (best_moves, counter)
    }

    fn minimax_step(
        board: Board,
        piece: Piece,
        maximizing: bool,
        top_level: bool,
        alpha: GameResult,
        beta: GameResult,
        best_moves: &mut Vec<Index>,
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

        let mut best_result = if maximizing { alpha } else { beta };
        for index in Board::VALID_INDECES {
            if !board.cell_is_empty(index) {
                continue;
            }

            let mut new_board = board.clone();
            new_board.place_piece(index, if maximizing { piece } else { piece.swap() });
            let result = Self::minimax_step(
                new_board,
                piece,
                !maximizing,
                false,
                if maximizing { best_result } else { alpha },
                if maximizing { beta } else { best_result },
                best_moves,
                counter,
            );
            if maximizing && result > best_result || !maximizing && result < best_result {
                best_result = result;
                if top_level {
                    best_moves.clear();
                    best_moves.push(index);
                }
                if maximizing && best_result >= beta || !maximizing && best_result <= alpha {
                    // alpha beta pruning only after first move so all first moves are still possible
                    break;
                }
            }
            if top_level {
                if result == best_result {
                    best_moves.push(index);
                }
            }
        }
        best_result
    }
}

impl Player for MinimaxPlayer {
    fn play(&self, board: &Board, piece: Piece) -> Index {
        let (best_moves, counter) = MinimaxPlayer::minimax_search(board.clone(), piece);
        println!("Searched {} possible games!", counter);

        best_moves[rand::thread_rng().gen_range(0, best_moves.len())]
    }
}

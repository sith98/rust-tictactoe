# rust-tictactoe
Command line Tic-Tac-Toe game including perfect AI, written in rust

## Installation
Using Rust's package manager Cargo (e.g. `cargo build --release`). See cargo documentation for more details.

## Usage
Pass the two players as command line arguments.

### Example
```
$ tictactoe a h
```
First player is an AI. Second player is a human.

### Available Players
- `h` or `human`: Human player plays by entering the number of the field (0 through 9) they want to play on.
- `a` or `alphabeta`: AI player who plays optimal strategy.
  If more than one optimal move is possible, they choose randomly.
- `m` or `minimax`: AI player with identical strategy but without the optimization of alpha beta pruning.
  This player actually checks all possible games before making a move.
- `r` or `random`: Plays randomly with equal probabilty for all possible moves.

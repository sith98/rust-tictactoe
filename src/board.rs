use std::fmt::{Display, Error, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}
impl Piece {
    pub fn swap(&self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}
impl Display for Piece {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Self::X => formatter.write_char('X')?,
            Self::O => formatter.write_char('O')?,
        }
        Ok(())
    }
}

pub type Cell = Option<Piece>;
pub type Index = usize;

static WINNING_INDECES: &'static [(Index, Index, Index)] = &[
    (0, 1, 2),
    (3, 4, 5),
    (6, 7, 8),
    (0, 3, 6),
    (1, 4, 7),
    (2, 5, 8),
    (0, 4, 8),
    (2, 4, 6),
];

#[derive(Clone)]
pub struct Board {
    board: [Cell; 9],
    n_pieces: u8,
}
impl Board {
    pub fn new() -> Self {
        Board {
            board: [None; 9],
            n_pieces: 0,
        }
    }

    pub fn cell_is_empty(&self, index: Index) -> bool {
        self.board[index] == None
    }

    pub fn place_piece(&mut self, index: Index, piece: Piece) -> bool {
        if !self.cell_is_empty(index) {
            panic!("Tried to place a piece on an occupied cell");
        }
        self.board[index] = Some(piece);
        self.n_pieces += 1;
        true
    }

    pub const VALID_INDECES: std::ops::Range<usize> = 0..9;

    pub fn is_valid_index(index: Index) -> bool {
        Self::VALID_INDECES.contains(&index)
    }

    pub fn is_draw(&self) -> bool {
        self.n_pieces >= 9
    }

    pub fn is_victory_for_player(&self, player: Piece) -> bool {
        for &(a, b, c) in WINNING_INDECES {
            if self.board[a] == Some(player)
                && self.board[b] == Some(player)
                && self.board[c] == Some(player)
            {
                return true;
            }
        }
        return false;
    }

    pub fn is_victory(&self) -> Option<Piece> {
        if self.is_victory_for_player(Piece::X) {
            Some(Piece::X)
        } else if self.is_victory_for_player(Piece::O) {
            Some(Piece::O)
        } else {
            None
        }
    }

    fn format_cell(&self, index: Index, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        match self.board[index] {
            None => write!(formatter, "{}", index),
            Some(player) => player.fmt(formatter),
        }
    }
}
impl Display for Board {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        self.format_cell(0, formatter)?;
        self.format_cell(1, formatter)?;
        self.format_cell(2, formatter)?;
        writeln!(formatter, "")?;

        self.format_cell(3, formatter)?;
        self.format_cell(4, formatter)?;
        self.format_cell(5, formatter)?;
        writeln!(formatter, "")?;

        self.format_cell(6, formatter)?;
        self.format_cell(7, formatter)?;
        self.format_cell(8, formatter)?;

        Ok(())
    }
}

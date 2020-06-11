use std::fmt::{Display, Error, Formatter, Write};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Player {
    X,
    O,
}
impl Display for Player {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        match *self {
            Player::X => formatter.write_char('X')?,
            Player::O => formatter.write_char('O')?,
        }
        Ok(())
    }
}

pub type Cell = Option<Player>;

pub struct Board {
    board: [Cell; 9],
}
impl Board {
    pub fn new() -> Self {
        Board { board: [None; 9] }
    }

    pub fn place_piece(&mut self, index: usize, piece: Player) -> bool {
        if self.board[index] != None {
            false
        } else {
            self.board[index] = Some(piece);
            true
        }
    }

    fn format_cell(&self, index: usize, formatter: &mut Formatter<'_>) -> Result<(), Error> {
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

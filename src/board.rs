use crate::{coordinates::Coords, piece::Piece, position::Position};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    rc::Rc,
};

const EMPTY_POSITION: char = ' ';

pub type DataPointer = Rc<RefCell<RawData>>;

#[derive(Debug)]
pub struct RawData(Vec<Vec<char>>);

impl Deref for RawData {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RawData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RawData {
    fn new(size: usize) -> Self {
        RawData(vec![vec![EMPTY_POSITION; size]; size])
    }

    pub fn write(&mut self, coords: Coords, c: char) {
        self[coords.row][coords.col] = c;
    }

    pub fn read(&self, coords: Coords) -> char {
        self[coords.row][coords.col]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoardError {
    #[error("data store disconnected")]
    ParseError,

    #[error("invalid position")]
    InvalidPosition,

    #[error("positiin occupied")]
    PositionAlreadyOccupied,

    #[error("position not occupied")]
    PositionNotOcuppiedError,

    #[error("Invalid board size")]
    InvalidBoardSize,
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    data: DataPointer,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, e) in self.data.borrow().iter().enumerate() {
            for (col, _) in e.iter().enumerate() {
                write!(
                    f,
                    " {} ",
                    Position::new(self.data.clone(), Coords::new(row, col))
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Result<Board, BoardError> {
        if size <= 4 || (1 == size % 2) {
            return Err(BoardError::InvalidBoardSize);
        }

        let mut data = RawData::new(size);
        let half = size / 2;
        data.write(Coords::new(half, half), Piece::Blue.into());
        data.write(Coords::new(half - 1, half - 1), Piece::Blue.into());
        data.write(Coords::new(half - 1, half), Piece::Red.into());
        data.write(Coords::new(half, half - 1), Piece::Red.into());

        Ok(Board {
            size,
            data: Rc::new(RefCell::new(data)),
        })
    }

    pub fn get(&self, coords: &Coords) -> Result<Position, BoardError> {
        if self.size > coords.row && self.size > coords.col {
            return Ok(Position::new(self.data.clone(), *coords));
        }
        Err(BoardError::InvalidPosition)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_valid_board_size() {
        assert!(Board::new(16).is_ok());
        assert!(Board::new(8).is_ok());
        assert!(Board::new(6).is_ok());
        assert!(Board::new(4).is_err());
        assert!(Board::new(13).is_err());
    }

    #[test]
    fn test_board_initial_setup() {
        println!("{}", Board::new(8).unwrap());
    }
}

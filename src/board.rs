use crate::{coordinates::Coords, piece::Piece, position::Position, Wrap};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub type DataPointer = Rc<RefCell<RawData>>;

//criar iterator para RawData

#[derive(Debug)]
pub struct RawData(Vec<Vec<char>>);

impl RawData {
    fn new(size: usize) -> Self {
        RawData(vec![vec![' '; size]; size])
    }

    pub fn write(&mut self, coords: Coords, c: char) {
        self.0[coords.row][coords.col] = c;
    }

    pub fn read(&self, coords: Coords) -> char {
        self.0[coords.row][coords.col]
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
        for (_, e) in self.data.borrow().0.iter().enumerate() {
            for (_, e) in e.iter().enumerate() {
                write!(f, "[{}]", e)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl From<Option<Piece>> for Wrap<Option<Piece>> {
    fn from(value: Option<Piece>) -> Self {
        Wrap(value)
    }
}

impl Board {
    pub fn new(size: usize) -> Result<Board, BoardError> {
        if size <= 4 || (1 == size % 2) {
            return Err(BoardError::InvalidBoardSize);
        }

        let mut data = RawData::new(size);
        data.write(Coords::new(size / 2, size / 2), Piece::Blue.into());
        data.write(
            Coords::new((size / 2) - 1, (size / 2) - 1),
            Piece::Blue.into(),
        );
        data.write(Coords::new((size / 2) - 1, size / 2), Piece::Red.into());
        data.write(Coords::new(size / 2, (size / 2) - 1), Piece::Red.into());

        Ok(Board {
            size,
            data: Rc::new(RefCell::new(data)),
        })
    }

    pub fn get(&self, coords: &Coords) -> Result<Position, BoardError> {
        if self.size > coords.row && self.size > coords.col {
            return Ok(Position::new(self.data.clone(), coords.clone()));
        }
        Err(BoardError::InvalidPosition)
    }
}

impl Into<char> for Wrap<Option<Piece>> {
    fn into(self) -> char {
        match self {
            Wrap(None) => ' ',
            Wrap(Some(p)) => p.into(),
        }
    }
}

impl TryFrom<char> for Wrap<Option<Piece>> {
    type Error = BoardError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'B' => Ok(Wrap(Some(Piece::Blue))),
            'R' => Ok(Wrap(Some(Piece::Red))),
            ' ' => Ok(Wrap(None)),
            _ => Err(BoardError::ParseError),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::ops::Deref;

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
    fn test_board_initial_setup() {}
}

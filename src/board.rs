use crate::{coordinates::Coords, piece::Piece, position::Position, Wrap};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub type DataPointer = Rc<RefCell<Vec<Vec<char>>>>;

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
        for (_, e) in self.data.borrow().iter().enumerate() {
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

        let positions: Vec<Vec<char>> = (0..size)
            .map(|row| {
                let x: Vec<char> = (0..size)
                    .map(|col| {
                        let half = size / 2;
                        Wrap(
                            if (row == half && col == half) || (row == half - 1 && col == half - 1)
                            {
                                Some(Piece::Blue)
                            } else if (row == half && col == half - 1)
                                || (row == half - 1 && col == half)
                            {
                                Some(Piece::Red)
                            } else {
                                None
                            },
                        )
                        .into()
                    })
                    .collect::<Vec<char>>()
                    .try_into()
                    .unwrap();
                x
            })
            .collect::<Vec<Vec<char>>>();

        Ok(Board {
            size,
            data: Rc::new(RefCell::new(positions)),
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
    fn test_board_initial_setup() {
        let b = Board::new(6).unwrap();
        let data = b.data.borrow();
        let data = data.deref();

        let x = data
            .iter()
            .enumerate()
            .map(|(r, e)| e.iter().enumerate().map(move |(col, e)| (r, col, e)))
            .flatten()
            .for_each(|(r, c, v)| match (r, c) {
                (2, 2) => assert_eq!(*v, 'B'),
                (3, 3) => assert_eq!(*v, 'B'),
                (2, 3) => assert_eq!(*v, 'R'),
                (3, 2) => assert_eq!(*v, 'R'),
                _ => assert_eq!(*v, ' '),
            });

        dbg!(x);
    }
}

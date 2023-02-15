use crate::{
    board::{BoardError, DataPointer},
    coordinates::Coords,
    piece::Piece,
};
use colored::*;
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PositionError {
    #[error("position could not be flipped")]
    FlipError,
}

pub struct Position {
    data: DataPointer,
    coords: Coords,
}

impl Position {
    pub fn new(data: DataPointer, coords: Coords) -> Self {
        Position { data, coords }
    }

    fn piece(&self) -> Option<Piece> {
        let c = self.data.borrow().read(self.coords);
        match c {
            ' ' => None,
            c => Some(Piece::from(c)),
        }
    }

    pub fn place(self, piece: Piece) -> Result<Position, BoardError> {
        if self.occupied() {
            return Err(BoardError::PositionAlreadyOccupied);
        }

        self.data.borrow_mut().write(self.coords, piece.into());

        Ok(self)
    }

    pub fn flip(self) -> Result<Self, PositionError> {
        match self.piece() {
            Some(p) => {
                self.data.borrow_mut().write(self.coords, (!p).into());
                Ok(self)
            }
            None => Err(PositionError::FlipError),
        }
    }

    fn occupied(&self) -> bool {
        self.piece().is_some()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.piece() {
            Some(p) => match p {
                Piece::Blue => write!(f, " {} ", "●".blue()),
                Piece::Red => write!(f, " {} ", "●".red()),
            },
            None => write!(f, " {} ", "●".white()),
        }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("piece", &self.piece())
            .field("coords", &self.coords)
            .finish()
    }
}

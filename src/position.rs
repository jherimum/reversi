use crate::{board::MatrixPointer, coordinates::Coords, piece::Piece, Wrap};
use colored::*;
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PositionError {
    #[error("position could not be flipped")]
    FlipError,

    #[error("position could not be flipped")]
    PositionAlreadyOccupied,
}

pub struct Position {
    matrix: MatrixPointer,
    coords: Coords,
}

impl Position {
    pub fn new(matrix: MatrixPointer, coords: Coords) -> Self {
        Position { matrix, coords }
    }

    fn piece(&self) -> Option<Piece> {
        let c = self.matrix.borrow().read(self.coords);
        let piece: Wrap<Option<Piece>> = c.into();
        *piece
    }

    pub fn place(self, piece: Piece) -> Result<Position, PositionError> {
        if self.occupied() {
            return Err(PositionError::PositionAlreadyOccupied);
        }

        self.matrix.borrow_mut().write(self.coords, piece.into());

        Ok(self)
    }

    pub fn flip(self) -> Result<Self, PositionError> {
        match self.piece() {
            Some(p) => {
                self.matrix.borrow_mut().write(self.coords, (!p).into());
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
                Piece::Blue => write!(f, "{}", "●".blue()),
                Piece::Red => write!(f, "{}", "●".red()),
            },
            None => write!(f, "{}", "○".white()),
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

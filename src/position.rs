use std::fmt::Debug;

use thiserror::Error;

use crate::{
    board::{BoardError, DataPointer},
    coordinates::Coords,
    piece::Piece,
    Wrap,
};

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

    fn piece(&self) -> Result<Option<Piece>, BoardError> {
        let x = self.data.borrow_mut().read(self.coords);
        let x: Result<Wrap<Option<Piece>>, BoardError> = x.try_into();
        x.map(|w| w.0)
    }

    pub fn r#move(self, piece: Piece) -> Result<Position, BoardError> {
        if self.occupied()? {
            return Err(BoardError::PositionAlreadyOccupied);
        }

        self.data.borrow_mut().write(self.coords, piece.into());

        Ok(self)
    }

    pub fn flip(self) -> Result<Self, PositionError> {
        match self.piece() {
            Ok(p) => match p {
                Some(p) => {
                    self.data.borrow_mut().write(self.coords, (!p).into());
                    Ok(self)
                }
                None => Err(PositionError::FlipError),
            },
            Err(_) => Err(PositionError::FlipError),
        }
    }

    fn occupied(&self) -> Result<bool, BoardError> {
        self.piece().map(|o| o.is_some())
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

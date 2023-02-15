use crate::{board::MatrixPointer, coordinates::Coords, piece::Piece, walker::Walkable, Dir, Wrap};
use colored::*;
use enum_iterator::all;
use std::fmt::{Debug, Display};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum PositionError {
    #[error("position could not be flipped")]
    FlipError,

    #[error("position could not be flipped")]
    PositionAlreadyOccupied,
}

#[derive(Clone)]
pub struct Position {
    matrix: MatrixPointer,
    coords: Coords,
}

impl Position {
    pub fn copy(&self, coords: Coords) -> Position {
        Position::new(self.matrix.clone(), coords)
    }

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
        self.solve(piece);
        self.matrix.borrow_mut().write(self.coords, piece.into());
        Ok(self)
    }

    fn solve(&self, piece: Piece) {
        for dir in all::<Dir>() {
            self.solve_dir(piece, dir);
        }
    }

    fn solve_dir(&self, piece: Piece, dir: Dir) {
        let turnnables = self
            .coords
            .walker(dir)
            .into_iter()
            .map(|c| Position::new(self.matrix.clone(), c))
            .take_while(|p| p.piece() == Some(!piece))
            .collect::<Vec<_>>();
        let turn = turnnables
            .last()
            .and_then(|last_pos| last_pos.coords.walker(dir).into_iter().next())
            .map(|c| Position::new(self.matrix.clone(), c))
            .map(|p| p.piece() == Some(piece))
            .unwrap_or(false);

        if turn {
            for p in turnnables {
                p.flip();
            }
        }
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

#[cfg(test)]
mod tests {

    use std::{cell::RefCell, rc::Rc, str::FromStr};

    use crate::{board::Matrix, piece, walker::Walkable};

    use super::*;

    #[test]
    fn x() {
        let mut raw = Matrix::new(8);
        raw.write(Coords::from_str("B:1").unwrap(), Piece::Red.into());
        raw.write(Coords::from_str("C:1").unwrap(), Piece::Red.into());
        raw.write(Coords::from_str("D:1").unwrap(), Piece::Blue.into());

        let matrix = Rc::new(RefCell::new(raw));
        let position = Position::new(matrix.clone(), Coords::from_str("A:1").unwrap());
        let piece = Piece::Blue;
        let dir = Dir::Down;

        let turnnables = position
            .coords
            .walker(dir)
            .into_iter()
            .peekable()
            .map(|c| Position::new(matrix.clone(), c))
            .take_while(|p| p.piece() == Some(!piece))
            .collect::<Vec<_>>();

        let turn = turnnables
            .last()
            .and_then(|last_pos| last_pos.coords.walker(dir).into_iter().next())
            .map(|c| Position::new(matrix.clone(), c))
            .map(|p| p.piece() == Some(piece))
            .unwrap_or(false);

        if turn {
            dbg!(turnnables);
        } else {
            dbg!("None");
        }
    }
}

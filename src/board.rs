use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{
    coordinates::{Coords, Dir},
    Wrap,
};

pub type DataPointer = Rc<RefCell<Vec<Vec<char>>>>;

pub struct PositionIterator {
    dir: Dir,
    coords: Coords,
    data: DataPointer,
}

impl Iterator for PositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dir {
            Dir::Up => todo!(),
            Dir::UpRight => todo!(),
            Dir::Right => todo!(),
            Dir::DownRight => todo!(),
            Dir::Down => todo!(),
            Dir::DownLeft => todo!(),
            Dir::Left => todo!(),
            Dir::UpLeft => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum BoardError {
    ParseError,
    InvalidPosition,
    PositionAlreadyOccupied,
    PositionNotOcuppiedError,
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    data: DataPointer,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // for (_, e) in self.positions.iter().enumerate() {
        //     for (_, e) in e.iter().enumerate() {
        //         write!(f, "[{}]", e);
        //     }
        //     writeln!(f, "");
        // }

        // Ok(())
        todo!()
    }
}

impl Board {
    pub fn get(&self, coords: &Coords) -> Result<Position, BoardError> {
        if self.size > coords.row && self.size > coords.col {
            return Ok(Position::new(self.data.clone(), coords.clone()));
        }
        Err(BoardError::InvalidPosition)
    }

    pub fn new(size: usize) -> Result<Board, BoardError> {
        let positions: Vec<Vec<char>> = (0..size)
            .map(|r| {
                let x: Vec<char> = (0..size)
                    .map(|c| {
                        let half = size / 2;
                        if (r == half && c == half) || (r == half - 1 && c == half - 1) {
                            Wrap(Some(Piece::Blue)).try_into().unwrap()
                        } else if (r == half && c == half - 1) || (r == half - 1 && c == half) {
                            Wrap(Some(Piece::Red)).try_into().unwrap()
                        } else {
                            Wrap(None).try_into().unwrap()
                        }
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
}

pub struct Position {
    data: DataPointer,
    coords: Coords,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("piece", &self.piece())
            .field("coords", &self.coords)
            .finish()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Position {
    pub fn new(data: DataPointer, coords: Coords) -> Self {
        Position { data, coords }
    }

    fn stream(&self, dir: Dir) -> PositionIterator {
        PositionIterator {
            dir,
            coords: self.coords,
            data: self.data.clone(),
        }
    }

    fn piece(&self) -> Result<Option<Piece>, BoardError> {
        let x = self.data.borrow_mut()[self.coords.row][self.coords.col];
        let x: Result<Wrap<Option<Piece>>, BoardError> = x.try_into();
        x.map(|w| w.0)
    }

    fn place(self, piece: Piece) -> Result<Position, BoardError> {
        if self.occupied()? {
            return Err(BoardError::PositionAlreadyOccupied);
        }

        self.data.borrow_mut()[self.coords.row][self.coords.col] = piece.into();

        Ok(self)
    }

    fn flip(self) -> Result<Self, BoardError> {
        match self.piece() {
            Ok(p) => match p {
                Some(p) => {
                    self.data.borrow_mut()[self.coords.row][self.coords.col] = p.flip().into();
                    Ok(self)
                }
                None => Err(BoardError::PositionNotOcuppiedError),
            },
            Err(e) => Err(e),
        }
    }

    fn occupied(&self) -> Result<bool, BoardError> {
        self.piece().map(|o| o.is_some())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Piece {
    Blue,
    Red,
}

impl Piece {
    pub fn flip(&self) -> Self {
        match self {
            Piece::Blue => Piece::Red,
            Piece::Red => Piece::Blue,
        }
    }
}

impl Into<char> for Piece {
    fn into(self) -> char {
        match self {
            Piece::Blue => 'B',
            Piece::Red => 'R',
        }
    }
}

impl TryInto<char> for Wrap<Option<Piece>> {
    type Error = BoardError;

    fn try_into(self) -> Result<char, Self::Error> {
        match self {
            Wrap(None) => Ok(' '),
            Wrap(Some(Piece::Blue)) => Ok('B'),
            Wrap(Some(Piece::Red)) => Ok('R'),
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

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_flip() {
        assert_eq!(Piece::Blue.flip(), Piece::Red);
        assert_eq!(Piece::Red.flip(), Piece::Blue);
    }

    #[test]
    fn test1() {
        let b = Board::new(8).unwrap();
        let c = Coords::from_str("A:1").unwrap();
        let p = b.get(&c).unwrap().place(Piece::Blue).unwrap();
        dbg!(p);

        let p1 = b.get(&c).unwrap().flip().unwrap();
        dbg!(p1);
    }
}

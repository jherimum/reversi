pub mod coordinates;

use coordinates::{Coordinates, Direction};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub struct PositionIterator {
    direction: Direction,
    coordinates: Coordinates,
    board: Rc<RefCell<Vec<Vec<char>>>>,
}

impl Iterator for PositionIterator {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::Up => todo!(),
            Direction::UpRight => todo!(),
            Direction::Right => todo!(),
            Direction::DownRight => todo!(),
            Direction::Down => todo!(),
            Direction::DownLeft => todo!(),
            Direction::Left => todo!(),
            Direction::UpLeft => todo!(),
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

pub mod board;

#[derive(Debug)]
pub struct Board {
    size: usize,
    positions: Rc<RefCell<Vec<Vec<char>>>>,
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
    pub fn get(&self, coord: &Coordinates) -> Result<Position, BoardError> {
        if self.size > coord.row && self.size > coord.col {
            return Ok(Position::new(self.positions.clone(), coord.clone()));
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
            positions: Rc::new(RefCell::new(positions)),
        })
    }
}

pub struct Position {
    board: Rc<RefCell<Vec<Vec<char>>>>,
    coordinates: Coordinates,
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("piece", &self.piece())
            .field("coordinates", &self.coordinates)
            .finish()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Position {
    fn stream(&self, direction: Direction) -> PositionIterator {
        PositionIterator {
            direction: direction,
            coordinates: self.coordinates,
            board: self.board.clone(),
        }
    }

    pub fn new(board: Rc<RefCell<Vec<Vec<char>>>>, coordinates: Coordinates) -> Self {
        Position { board, coordinates }
    }

    fn piece(&self) -> Result<Option<Piece>, BoardError> {
        let x = self.board.borrow_mut()[self.coordinates.row][self.coordinates.col];
        let x: Result<Wrap<Option<Piece>>, BoardError> = x.try_into();
        x.map(|w| w.0)
    }

    fn place(self, piece: Piece) -> Result<Position, BoardError> {
        if self.occupied()? {
            return Err(BoardError::PositionAlreadyOccupied);
        }

        self.board.borrow_mut()[self.coordinates.row][self.coordinates.col] = piece.into();

        Ok(self)
    }

    fn flip(self) -> Result<Self, BoardError> {
        match self.piece() {
            Ok(p) => match p {
                Some(p) => {
                    self.board.borrow_mut()[self.coordinates.row][self.coordinates.col] =
                        p.flip().into();
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

pub struct Wrap<T>(T);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{coordinates::Coordinates, Board, Piece};

    #[test]
    fn test_flip() {
        assert_eq!(Piece::Blue.flip(), Piece::Red);
        assert_eq!(Piece::Red.flip(), Piece::Blue);
    }

    #[test]
    fn test1() {
        let b = Board::new(8).unwrap();
        let c = Coordinates::from_str("A:1").unwrap();
        let p = b.get(&c).unwrap().place(Piece::Blue).unwrap();
        dbg!(p);

        let p1 = b.get(&c).unwrap().flip().unwrap();
        dbg!(p1);
    }
}

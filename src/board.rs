use crate::{coordinates::Coords, piece::Piece, position::Position};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    rc::Rc,
};

pub const EMPTY_POSITION: char = ' ';

pub type MatrixPointer = Rc<RefCell<Matrix>>;

#[derive(Debug)]
pub struct Matrix(Box<[char]>);

impl Deref for Matrix {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut()
    }
}

impl Matrix {
    pub fn size(&self) -> usize {
        (self.0.len() as f64).sqrt().round() as usize
    }

    pub fn new(size: usize) -> Self {
        Matrix(vec![EMPTY_POSITION; size * size].into_boxed_slice())
    }

    pub fn write(&mut self, coords: Coords, c: char) {
        let size = self.size();
        self[(size * coords.row) + coords.col] = c;
    }

    pub fn read(&self, coords: Coords) -> char {
        self[(self.size() * coords.row) + coords.col]
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BoardError {
    #[error("Invalid position: {0}")]
    InvalidPosition(Coords),

    #[error("Invalid board size: {0}. The size must be a number greater than 4 and even.")]
    InvalidBoardSize(usize),
}

#[derive(Debug)]
pub struct Board {
    size: usize,
    matrix: MatrixPointer,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = self.matrix.borrow();
        let x = x.chunks(self.size);

        for row in x.into_iter().enumerate() {
            for col in row.1.iter().enumerate() {
                write!(
                    f,
                    " {}",
                    Position::new(self.matrix.clone(), Coords::new(row.0, col.0))
                )?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    pub fn new(size: usize) -> Result<Board, BoardError> {
        if size <= 4 || (size % 2 == 1) {
            return Err(BoardError::InvalidBoardSize(size));
        }

        let mut data = Matrix::new(size);
        let half = size / 2;
        data.write(Coords::new(half, half), Piece::Blue.into());
        data.write(Coords::new(half - 1, half - 1), Piece::Blue.into());
        data.write(Coords::new(half - 1, half), Piece::Red.into());
        data.write(Coords::new(half, half - 1), Piece::Red.into());

        Ok(Board {
            size,
            matrix: Rc::new(RefCell::new(data)),
        })
    }

    pub fn get(&self, coords: Coords) -> Result<Position, BoardError> {
        if self.size > coords.row && self.size > coords.col {
            return Ok(Position::new(self.matrix.clone(), coords));
        }
        Err(BoardError::InvalidPosition(coords))
    }
}

#[cfg(test)]
mod tests {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_matrix() {
        let mut m = Matrix::new(8);
        let c = m.read(Coords::new(7, 7));
        let c = m.write(Coords::new(7, 7), 'B');
        dbg!(m);
    }

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
        //print!("{}[2J", 27 as char);
        let moves = vec![
            ("E:3", Piece::Blue),
            ("D:3", Piece::Red),
            ("C:3", Piece::Blue),
            ("D:2", Piece::Red),
        ];
        let board = Board::new(8).unwrap();
        //println!("{}", board);

        for m in moves {
            board
                .get(Coords::from_str(m.0).unwrap())
                .unwrap()
                .place(m.1)
                .unwrap();
            //println!("{}", board);
        }
    }
}

use crate::{board::Board, coordinates::Coords, piece::Piece};
use anyhow::Result;
use std::{fmt::Display, str::FromStr};

pub struct Game {
    pub turn: Piece,
    pub board: Board,
    moves: Vec<Move>,
}

pub struct MoveResume {
    at: Coords,
    piece: Piece,
    flips: Vec<Coords>,
    winner: Option<Piece>,
}

impl MoveResume {
    fn new(at: Coords, piece: Piece, flips: Vec<Coords>, winner: Option<Piece>) -> Self {
        Self {
            at,
            piece,
            flips,
            winner,
        }
    }
}

impl Game {
    pub fn new(board_size: usize) -> Result<Self> {
        Ok(Game {
            turn: Piece::rand(),
            board: Board::new(board_size)?,
            moves: vec![],
        })
    }

    pub fn place(&mut self, coords: &str) -> Result<MoveResume> {
        let coords = Coords::from_str(coords)?;
        let result = self
            .board
            .get(coords)?
            .place(self.turn)
            .map(|r| MoveResume::new(coords, self.turn, r, None))?;
        self.moves.push(Move::new(self.turn, coords));
        self.turn = !self.turn;
        Ok(result)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    piece: Piece,
    coords: Coords,
}

impl Move {
    fn new(piece: Piece, coords: Coords) -> Self {
        Self { piece, coords }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}", self.piece, self.coords)
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn x() {
        let mut game = Game::new(8).unwrap();
        game.place("A:1");
        game.place("A:3");
        game.place("D:8").unwrap();
        game.place("B:4").unwrap();
        game.place("C:4").unwrap();
        game.place("F:4").unwrap();

        print!("{}", game.board);
    }
}

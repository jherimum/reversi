use crate::{board::Board, coordinates::Coords, piece::Piece};
use anyhow::Result;
use std::fmt::Display;

pub struct Game {
    turn: Piece,
    board: Board,
    moves: Vec<Move>,
}

impl Game {
    pub fn new(board_size: usize) -> Result<Self> {
        Ok(Game {
            turn: Piece::rand(),
            board: Board::new(board_size)?,
            moves: vec![],
        })
    }

    pub fn place(&mut self, coords: Coords) -> Result<()> {
        self.board.get(coords)?.place(self.turn)?;
        self.moves.push(Move::new(self.turn, coords));
        self.turn = !self.turn;
        Ok(())
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

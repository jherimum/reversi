use std::ops::{Deref, Not};

pub mod board;
pub mod coordinates;
pub mod game;
pub mod piece;
pub mod position;
pub mod walker;
pub struct Wrap<T>(pub T);

impl<T> Deref for Wrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Dir = Direction;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Self::Down,
            Direction::UpRight => Self::DownLeft,
            Direction::Right => Self::Left,
            Direction::DownRight => Self::UpLeft,
            Direction::Down => Self::Up,
            Direction::DownLeft => Self::UpRight,
            Direction::Left => Self::Right,
            Direction::UpLeft => Self::DownRight,
        }
    }
}
